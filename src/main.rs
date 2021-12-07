extern crate reqwest;

use serde_json::Value;
use std::env;
use std::process;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("No arguements provided. Terminating...\n(Usage: define 'word')");
        process::exit(1);
    } else if args.len() > 2 {
        println!("Too many arguements. Terminating...\n(Usage: define 'word')");
        process::exit(1);
    } else {
        let query_string = format!(
            "https://mashape-community-urban-dictionary.p.rapidapi.com/define?term={}",
            args[1]
        );

        let client = reqwest::Client::new();
        let res: Value = client
            .get(&query_string)
            .header("x-rapidapi-host", "<hostname/>")
            .header("x-rapidapi-key", "<api-key/>")
            .send()
            .await?
            .json()
            .await?;

        let list_of_definitions = res["list"].as_array();
        let iterable_array = list_of_definitions.unwrap();

        let mut i = 1;
        for item in iterable_array {
            println!("Definition {}: \n{}\n", i, item["definition"]);
            i = i + 1;
        }

        Ok(())
    }
}
