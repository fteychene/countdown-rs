# Countdown util

All in one exec to run some countdown.  
This project was created for my streams to have a unique tool cross platform to use with [OBS](https://obsproject.com/)

## Install

#### From sources
```bash
git clone https://github.com/fteychene/countdown-rs.git
cd countdown-rs
cargo build --release
# Move ./target/release/countdown in you path
```

#### Install via cargo

```bash
cargo install --git https://github.com/fteychene/countdown-rs.git
```

## Usage

```
countdown 0.0.1
Countdown util to console or file

USAGE:
    countdown [FLAGS] [OPTIONS] <time>

FLAGS:
    -d, --debug      Activate debug mode
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o, --output <out-type>    Output could be stdout or file [default: stdout]
    -s, --step <step>          Duration between steps [default: 1s]
        --to <to>              File name: only required when `out-type` is set to `file`

ARGS:
    <time>    Countdown time. Example: 3m, 1h5m, 5m15s
```

### Samples

Start a 3min countdown in stdout : `countdown 3m`

Start a 3min countdown in file `/tmp/countdown` : `countdown -o file --to /tmp/countdown 3m`

Start a 3min countdown with 5 sec steps : `countdown -s 5s 3m`
