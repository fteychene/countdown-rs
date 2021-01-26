use std::convert::TryFrom;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;

use anyhow::{anyhow, Context, Error};
use humantime::parse_duration;
use log::debug;
use simplelog::*;
use structopt::StructOpt;
use term::{stdout, StdoutTerminal};

#[derive(Debug)]
enum OutType {
    STDOUT,
    FILE,
}

impl FromStr for OutType {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "stdout" => Ok(OutType::STDOUT),
            "file" => Ok(OutType::FILE),
            _ => Err(anyhow!("{} is not a valid input. Use stdout, file", input))
        }
    }
}

fn cli_parse_duration(input: &str) -> Result<Duration, Error> {
    parse_duration(input).context("Error parsing duration argument")
}

fn format_duration(duration: Duration) -> String {
    let seconds = duration.as_secs() % 60;
    let minutes = (duration.as_secs() / 60) % 60;
    let hours = (duration.as_secs() / 60) / 60;
    if hours > 0 {
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    } else {
        format!("{:02}:{:02}", minutes, seconds)
    }
}


#[derive(Debug, StructOpt)]
#[structopt(name = "countdown", about = "Countdown util to console or file")]
struct Opt {
    /// Activate debug mode
    #[structopt(short, long)]
    debug: bool,

    /// Where to write the output
    #[structopt(name="out-type", short = "o", long = "output", default_value = "stdout", help = "Output could be stdout or file")]
    out_type: OutType,

    /// File name: only required when `out-type` is set to `file`
    #[structopt(long, required_if("out-type", "file"), parse(from_os_str))]
    to: Option<PathBuf>,

    /// Duration between steps. Default 1s
    #[structopt(short = "s", long, default_value = "1s", parse(try_from_str = cli_parse_duration))]
    step: Duration,

    /// Countdown time. Example: 3m, 1h5m, 5m15s
    #[structopt(parse(try_from_str = cli_parse_duration))]
    time: Duration
}

enum CountdownOutput {
    STDOUT(Box<StdoutTerminal>),
    FILE(PathBuf),
}

impl TryFrom<&Opt> for CountdownOutput {
    type Error = Error;

    fn try_from(opt:& Opt) -> Result<Self, Self::Error> {
        Ok(match opt.out_type {
            OutType::STDOUT => CountdownOutput::STDOUT(stdout().context("Error getting reference on terminal")?),
            OutType::FILE => CountdownOutput::FILE(opt.to.clone().unwrap())
        })
    }
}


trait CountdownWriter {
    fn write(&mut self, buf: &str) -> Result<(), Error>;
}

impl CountdownWriter for CountdownOutput {
    fn write(&mut self, buf: &str) -> Result<(), Error> {
        match self {
            CountdownOutput::STDOUT(term) => {
                term.write(format!("\r{}", buf).as_bytes())?;
                term.flush()
            }.context("Error writing in terminal"),
            CountdownOutput::FILE(path) => {
                let mut file = File::create(path).context("Error opening output file")?;
                file.write_all(buf.as_bytes())?;
                file.flush()
            }.context("Error writing in file")
        }
    }
}

fn start(mut duration: Duration, step: &Duration, mut output: impl CountdownWriter) -> Result<(), Error> {
    while duration.as_secs() > 0 {
        output.write(&format_duration(duration.clone()).to_string())?;
        sleep(step.clone());
        duration -= step.clone();
    }
    output.write("\n")?;
    Ok(())
}

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();
    CombinedLogger::init( vec![TermLogger::new(if opt.debug { LevelFilter::Debug } else { LevelFilter::Warn } , Config::default(), TerminalMode::Mixed)])?;
    debug!("Cli arguments : {:?}", opt);

    debug!("Configure output");
    let output: CountdownOutput = TryFrom::try_from(&opt)?;

    debug!("Start countdown");
    start(opt.time, &opt.step, output)?;

    debug!("End countdown");
    Ok(())
}

