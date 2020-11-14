// use std::fmt;
use std::path::{Path, PathBuf};
// use std::str::FromStr;
use crate::asm::parser::parse_from_file;
use crate::passes::layout::place;
use structopt::StructOpt;
use crate::util::file::write_to_file;

#[derive(Clone, Debug, StructOpt)]
#[structopt(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS")
)]
pub struct Opt {
    // Input file
    #[structopt(parse(from_os_str))]
    pub input: PathBuf,

    // Backend
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    pub output: Option<PathBuf>,
}

impl Opt {
    pub fn input(&self) -> &Path {
        &self.input
    }

    pub fn output(&self) -> Option<&PathBuf> {
        self.output.as_ref()
    }
}

#[derive(Clone, Debug)]
pub struct Optimize {
    pub opts: Opt,
}

impl Default for Optimize {
    fn default() -> Optimize {
        Optimize {
            opts: Opt::from_args(),
        }
    }
}

impl Optimize {
    pub fn new(opts: Opt) -> Optimize {
        Optimize { opts }
    }

    pub fn opts(&self) -> &Opt {
        &self.opts
    }

    fn write_output(&self, contents: &str) {
        if let Some(output) = self.opts().output() {
            write_to_file(output, contents);
        } else {
            println!("{}", contents);
        }
    }

    pub fn run(&self) {
        let prog = parse_from_file(self.opts().input());
        let placed = place::place_basic(&prog);
        self.write_output(&placed.to_string());
    }
}
