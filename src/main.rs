use std::path::PathBuf;
use rhai::{Engine, EvalAltResult, Scope};
use clap::Parser;

mod codevar;
mod codefile;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// OPTIONAL: Change CODE env variable if you want
    #[clap(short, long, value_parser)]
    codevar: Option<String>,

    /// OPTIONAL: Script file for execute it
    #[clap(short, long, value_parser)]
    scriptfile: Option<PathBuf>,

    /// OPTIONAL: Runs a specific task declared instead runs the code only
    #[clap(value_parser)]
    task: Option<String>,
}

const ENVCODE: &str = "CODE";
const FILECODE: &str = ".rr.rhai";

pub fn main() -> Result<(), Box<EvalAltResult>>
{
    let args = Args::parse(); // Clap parser

    let mut engine = Engine::new(); // Rhai parser

    // Execute code if is set in env variable
    let codevar = match args.codevar.as_deref(){
        Some(v) => v,
        None => ENVCODE // Default env key
    };

    let scriptfile = match args.scriptfile.as_deref(){
        Some(v) => v.as_os_str().to_str().unwrap(),
        None => FILECODE
    };

/*     // Tasks support
    let mut taskslist: Vec<String> = Vec::new();

    // Add function for tasks declarations
    fn add_task(task_name: &str, function_name: &str) {
        taskslist.push(task_name.to_string());
    }
    fn add_task_with_desc(){

    }
    engine.register_fn("task", add_task).register_fn("task", add_task_with_desc); */

    // Try to load the code from the env variable
    if let Some(code) = codevar::get_code_from_env(codevar){
        // Execute the script
        let ast = engine.compile(code.as_str())?;

        match args.task.as_deref(){
            Some(task_name) => {
                // Execute function by name
                let mut scope = Scope::new();
                engine.call_fn_raw(&mut scope, &ast, true, true, task_name, None, [])?;
            },
            None => {
                // Runs the code without task
                engine.eval_ast(&ast)?;
            }
        }
    }else if let Some(path_code) = codefile::get_code_from_file(scriptfile){
        // Try load the code of the script from a file
        let ast = engine.compile_file(path_code)?;

        match args.task.as_deref(){
            Some(task_name) => {
                // Execute function by name
                let mut scope = Scope::new();
                engine.call_fn_raw(&mut scope, &ast, true, true, task_name, None, [])?;
            },
            None => {
                // Runs the code without task
                engine.eval_ast(&ast)?;
            }
        }
    }

    Ok(())
}