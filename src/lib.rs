use std::fs::File;
use std::error::Error;
use std::io::prelude::*;
use std::string::ToString;

pub struct Config<T> {
    pub query: T,
    pub file: T,
    pub case_insensitive: bool
}

impl<T: PartialEq + ToString> Config<T> {
    pub fn new(args: &[T]) -> Result<Config<&T>, &'static str> {
        if args.len() < 2 {
            return Err("Not enough args");
        }
        Ok(Config {
            query: &args[0], 
            file: &args[1],
            case_insensitive: args.len() == 3 && args[2].to_string()[..] == *"-i"
        })
    }
}

pub fn run<T>(config: Config<T>) -> Result<(), Box<Error>> 
where T: std::convert::AsRef<std::path::Path> {
    let mut f = File::open(config.file)?;
    let mut contents = String::new();

    f.read_to_string(&mut contents).expect("can't read file");
    for line in search(&config.query.as_ref().to_str().unwrap(), &contents, &config.case_insensitive) {
        println!("{}",  line);
    }
    Ok(())
}


fn search<'a>(query: &str, contents: &'a str, case_insensitive: &bool) -> Vec<&'a str>  {
    let mut results = Vec::new();
    let query = if *case_insensitive {
        query.to_lowercase()
    } else {
        query.to_string()
    };

    let query = &query[..];
    for line in contents.lines() {
        let line_temp = if *case_insensitive {
            line.to_lowercase()
        } else {
            line.to_string()
        };
        if line_temp.contains(query) {
            results.push(line);
        }
    }
    
    results
}

#[cfg(test)]

mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "HA";
        let contents = "KAMEHAMEHA";

        assert_eq!(vec!["KAMEHAMEHA"], search(query, contents, &false));
    }

    #[test]
    fn case_sensitive() {
        let query = "HA";
        let contents = "KAMEHAMEHA";
        assert_eq!(vec!["KAMEHAMEHA"], search(query, contents, &false));
    }

    #[test]
    fn case_insensitive() {
        let query = "ha";
        let contents = "KAMEHAMEHA";
        assert_eq!(vec!["KAMEHAMEHA"], search(query, contents, &true));
    } 
}
