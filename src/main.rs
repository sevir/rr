use std::{env, path::PathBuf};
use rhai::{Engine, EvalAltResult};
use clap::Parser;

mod codevar;
mod codefile;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    // Change CODE env variable if you want
    #[clap(short, long, value_parser)]
    codevar: Option<String>,

    /// Number of times to greet
    #[clap(short, long, value_parser)]
    scriptfile: Option<PathBuf>,
}

const ENVCODE: &str = "CODE";

pub fn main() -> Result<(), Box<EvalAltResult>>
{
    let args = Args::parse(); // Clap parser

    let engine = Engine::new(); // Rhai parser

    // Execute code if is set in env variable
    let codevar = match args.codevar.as_deref(){
        Some(v) => v,
        None => ENVCODE // Default env key
    };

    // Try to load the code from the env variable
    if let Some(code) = codevar::get_code_from_env(codevar){
        // Execute the script
        engine.run(code.as_str())?;
    }else{
        // Try load the code of the script from a file
    }

    Ok(())
}