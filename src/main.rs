use std::env;
use rhai::{Engine, EvalAltResult};

pub fn main() -> Result<(), Box<EvalAltResult>>
{
    let engine = Engine::new();
    let code = match env::var_os("CODE") {
        Some(v) => v.into_string().unwrap(),
        None => "print(\"nocode\")".to_string()
    };

    engine.run(code.as_str())?;

    Ok(())
}