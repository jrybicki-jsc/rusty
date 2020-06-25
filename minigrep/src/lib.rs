use std::fs;
use std::error::Error;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    //println!("Text:\n{}", contents);
   
    let results = if config.case_sensitive {
        search(&config.query, &contents)

    } else {
        search_insensitive(&config.query, &contents)
    };

    for line in results { 
        println!(">{}", line);
    }

    Ok(())
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

pub fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
         }
    }
    result
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
 
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get filename"),
        };

        let case_sensitive = env::var("CASE_INSENSITVE").is_err();


        Ok(Config { query, filename, case_sensitive })
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

   #[test]
   fn insesitive() {
      let query = "dUct";
      let contents = "Rust\nsafe, fast, productive\nAnd some more\nLines of text";

      assert_eq!(vec!["safe, fast, productive"], search_insensitive(query, contents));
   }

}


