use regex::Regex;
use rhai::{AST};

#[derive(Debug)]
#[allow(dead_code)]
pub struct Task{
    name: String,
    alias: Option<String>,
    desc: Option<String>
}

pub fn get_tasks(ast: &AST) -> Vec<Task>{
    let mut tasklist: Vec<Task> = Vec::new();

    for declared_function in ast.iter_functions(){
        if !declared_function.comments.is_empty(){
            let re_task = Regex::new(r"///\s*[T,t][A,a][S,s][K,k]\s*(.*)").unwrap();
            let re_desc = Regex::new(r"/// \s*(.*)\s*$").unwrap();

            let task_declaration = declared_function.comments.first().unwrap();
            
            if let Some(task_alias) = re_task.captures( *task_declaration ){
                //Check if desc is declared in the second line of comments
                if declared_function.comments.len()>1{
                    let desc_declaration = declared_function.comments.get(1).unwrap();
                    let desc = re_desc.captures(&*desc_declaration).unwrap();

                    tasklist.push(Task{
                        name: declared_function.name.to_string(),
                        alias: if task_alias.get(1).unwrap().as_str() == ""{
                            None
                        }else{
                            Some(task_alias.get(1).unwrap().as_str().to_string())
                        },
                        desc: Some(desc.get(1).unwrap().as_str().to_string())
                    }); 
                }else{
                    tasklist.push(Task{
                        name: declared_function.name.to_string(),
                        alias: if task_alias.get(1).unwrap().as_str() == ""{
                            None
                        }else{
                            Some(task_alias.get(1).unwrap().as_str().to_string())
                        },
                        desc: None
                    }); 
                }                
            }            
        }        
    }
    //Return the list
    tasklist
}

pub fn get_task_function_name(tasks: &Vec<Task>, find_string: &str) -> Result<String,&'static str>{
    let mut task_iterator = tasks.iter();
    while let Some(task) = task_iterator.next() {
        let mut found = false;
        if *find_string.to_string() == task.name {
            found = true;
        }
        match &task.alias {
            Some(alias)=>{
                if *find_string.to_string() == *alias {
                    found = true;
                }
            },
            _ => {found = false;}
        }

        if found{
            return Ok(task.name.clone());
        }
    }
    Err("Not found")
}