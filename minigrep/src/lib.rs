use std::fs;
use std::error::Error;


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    //println!("Text:\n{}", contents);
    
    for line in search(&config.query, &contents) { 
        println!(">{}", line);
    }

    Ok(())
}


pub struct Config {
    pub query: String,
    pub filename: String,
}

pub fn search<'a>(query: &str, contents:&'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}


#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn one_result() {
      let query = "duct";
      let contents = "Rust\nsafe, fast, productive\nAnd some more\nLines of text";

      assert_eq!(vec!["safe, fast, productive"], search(query, contents));
   }

}


