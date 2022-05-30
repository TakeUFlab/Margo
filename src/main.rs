// // mod lexer;
mod lexer;
mod token;
mod types;

use chumsky::Parser as _;
use clap::{ArgEnum, Parser, Subcommand};
use lexer::file;
use std::fs;
use std::path::PathBuf;
use std::{error::Error, io::Write};
use types::File;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct CLi {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Build {
        input: PathBuf,

        #[clap(short, long)]
        output: Option<PathBuf>,

        #[clap(arg_enum)]
        target: BuildArg,
    },
}

#[derive(Debug, Clone, ArgEnum)]
enum BuildArg {
    Json,
    Rust,
    Html,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = CLi::parse();

    match cli.command {
        Command::Build {
            input,
            target,
            output,
        } => {
            let output = output.unwrap_or_else(|| input.with_extension("mgo.tmp"));
            let in_file = fs::read_to_string(input)?;
            let mut out_file = fs::File::create(output)?;
            let ast = file::parser().parse(in_file).unwrap();

            match target {
                BuildArg::Json => write!(&mut out_file, "{}", serde_json::to_string(&ast)?)?,
                BuildArg::Rust => write!(&mut out_file, "{:?}", &ast)?,
                BuildArg::Html => todo!(),
            }
        }
    }

    Ok(())
}
