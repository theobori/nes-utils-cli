# nes-utils-cli

## How to build and run ?

1. Install the dependencies
    - `cargo`
2. Compile and install it
    - `cargo install --path .`
3. Run `nes-utils-cli --help`

## Help

```bash
USAGE:
    nes-utils-cli [FLAGS] [OPTIONS] [input]

FLAGS:
    -d, --disassemble    Disassemble a NES file
    -e, --extract-chr    Dump CHR ROM graphics data into PNGs
    -h, --help           Prints help information
    -V, --version        Prints version information

OPTIONS:
    -c, --code <code>        Decode Game Genie
        --it <it>            PRNG iteration
    -o, --output <output>    Output filename
        --seed <seed>        PRNG seed

ARGS:
    <input>    Input file
```

## Usage example(s)

### Disassemble and extract CHR ROM

```bash
nes-utils-cli -de game.nes
```

### Using NES random

#### Only one iteration
```bash
nes-utils-cli --seed 14
```

#### One or multiple iterations
```bash
nes-utils-cli --seed 14 --it 1
```

or

```bash
nes-utils-cli --seed 14 --it 5
```

#### Decoding Game Genie
```bash
nes-utils-cli --code SXIOPO
```
