use std::env;

pub fn get_code_from_env(codevar : &str) -> Option<String>
{
    match env::var_os(codevar) {
        Some(v) => Some(v.into_string().unwrap()),
        None => {
            None
        }
    }
}