use std::env;

pub fn get_code_from_env(codevar : &str) -> Option<String>
{
    env::var_os(codevar).map(|v| v.into_string().unwrap())
}