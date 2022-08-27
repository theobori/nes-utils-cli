use nes_utils::chr::chr::NesChr;
use nes_utils::disassembler::disassembler::NesDisassembler;
use nes_utils::game_genie::game_genie::NesGameGenie;
use nes_utils::models::nesutil_model::NesUtil;
use nes_utils::prng::prng::NesPrng;

use structopt::StructOpt;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[derive(StructOpt, Debug)]
#[structopt(name = "nes-utils-cli")]
struct Opt {
    /// Dump CHR ROM graphics data into PNGs
    #[structopt(short, long)]
    extract_chr: bool,

    /// Disassemble a NES file
    #[structopt(short, long)]
    disassemble: bool,

    /// Decode Game Genie
    #[structopt(short, long)]
    code: Option<String>,

    /// Output filename
    #[structopt(short, long)]
    output: Option<String>,

    /// PRNG seed
    #[structopt(long)]
    seed: Option<u16>,

    /// PRNG iteration
    #[structopt(long)]
    it: Option<u16>,

    /// Input file
    #[structopt(parse(from_os_str))]
    input: Option<PathBuf>
}

/// Basic function to read a file as a vector of bytes
pub fn read_file(path: &String) -> Vec<u8> {
    let f = File::open(path);
        
    match f {
        Ok(mut file) => {
            let mut data = Vec::<u8>::new();

            match file.read_to_end(&mut data) {
                Ok(_) => data,
                Err(_) => panic!("Invalid file")
            }
        },
        Err(_) => panic!("File not found")
    }
}

/// Smart pointer to handle types that has implemented NesUtil
type Object = Box<dyn NesUtil>;

/// Interacts with the CLI arguments
pub struct CliArgs {
    opt: Opt,
    pub objs: Vec<Object>
}

impl CliArgs {
    pub fn new() -> Self {
        Self {
            opt: Opt::from_args(),
            objs: Vec::new()
        }
    }

    /// Manages options that need a file as input
    pub fn fill_dump(&mut self) {
        let input = match &self.opt.input {
            Some(value) => value,
            None => return
        };

        let input = input.to_str().unwrap();
        let path = String::from(input);
        let mem = read_file(&path);

        if self.opt.extract_chr {
            self.objs.push(Box::new(NesChr::new(&path, &mem)));
        }
        if self.opt.disassemble {
            self.objs.push(Box::new(NesDisassembler::new(&path, &mem)));
        }
    }

    /// Just load the nes random with a seed
    pub fn fill_prng(&mut self) {
        let seed = match self.opt.seed {
            Some(value) => value,
            None => return
        };
        let mut prng = NesPrng::new(seed, None);
            
        if let Some(it) = self.opt.it {
            prng.set_it(it);
        }
        
        self.objs.push(Box::new(prng));
    }

    /// Game genie (decode / encode)
    pub fn fill_gg(&mut self) {
        let code = match &self.opt.code {
            Some(code) => code.clone(),
            None => return,
        };

        let gg = NesGameGenie::new(code);

        self.objs.push(Box::new(gg));
    }

    /// Return a clone of output
    pub fn output(&self) -> Option<String> {
        self.opt.output.clone()
    }

    /// Parsing every arguments
    pub fn parse(&mut self) {
        self.fill_dump();
        self.fill_prng();
        self.fill_gg();
    }
}

fn main() {
    let mut args = CliArgs::new();
    
    args.parse();

    let output = args.output();

    for obj in args.objs.iter_mut() {
        obj.run();
        
        match output {
            Some(ref path) => obj.save_as(path),
            None => obj.save()
        };
    }
}
