// This is a seperate library file that contains the logic of th CLI-Parser.

use std::fs;
use std::error::Error;
use std::env;

// The run function is used for 
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    // Using the custom set enviornement condition to let the function determine wether to choose case_sensitive condtion or not
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    // After Defining the conditon this loop seraches for the query in each line as it loops through.
    for line in results {
        println!("{}", line)
    }

    Ok(())
}

// Defining the Custom Function for our usage.
pub struct Config{
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

//  Implementing the error function defining and printing out a clean and well definined error for the user.
impl Config{
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments!");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}

// Using a loop to seach thorught lines of text and saving it in the 'results' vector.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> { // Using Lifetimes "'a"
    let mut results = Vec::new();
    
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

// This function is same as the search function but is not case_sensitive and can be used by explicitly acttivating this enviornment.
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

// Defining the test cases for "Test Driven Development".
// In which we first define a test to failure and then write and define function to pass the test.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}