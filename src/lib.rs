use std::error::Error;
use std::fs;

pub fn process_file(config : &OptConfig) -> Result<(), Box<dyn Error>> {
    let contents : String = fs::read_to_string(config.filename)?;
    // println!("{}", contents);
    let result = search(&config.opt, &contents);
    if result.len() > 0 {
        println!("Find {} in text", config.opt);
        Ok(())
    }
    else {
        Err(format_args!("Can't find {}", config.opt).to_string().into())
    }
}


pub struct OptConfig<'a>{
    pub opt : &'a String,
    pub filename : &'a String
}


impl<'a> OptConfig<'a> {
    pub fn new(collect : &Vec<String>) -> Result<OptConfig, &'static str>{
        if collect.len() < 3 {
            return Err("length must large then 3");
        }

        return Ok(OptConfig{
            opt : &collect[1],
            filename : &collect[2]
        });
    }
}


pub fn search<'a>(query : &str, contents : &'a str) -> Vec<&'a str>{
    let mut result = Vec::new();
    for line in contents.lines(){
        if line.contains(query) == true {
            result.push(line);
        }
    }

    return result;
}



#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_search()
    {
        let query = "doc";
        let contents = "\
Rust Option :
Safe, fish, foot
Doc";
        let result : Vec<String> = vec![];
        assert_eq!(result, search(query, contents));
    }

    
}