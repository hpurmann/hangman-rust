extern crate serde_json;

extern crate hyper;
extern crate rand;
extern crate spinner;

use std::io::Read;
use self::hyper::{Client};
use self::rand::Rng;
use self::spinner::{SpinnerBuilder};

#[derive(Serialize, Deserialize)]
struct Entry {
    status: u32,
    total: u64,
    results: Vec<Result>,
}

#[derive(Serialize, Deserialize)]
struct Result {
    headword: String,
}

fn get_content(url: &str) -> hyper::Result<String> {
    let client = Client::new();
    let mut response = client.get(url).send()?;
    let mut buf = String::new();
    response.read_to_string(&mut buf)?;
    Ok(buf)
}

fn query(dict: &str, offset: u64, limit: i8) -> Entry {
    let res = get_content(
        format!(
            "http://api.pearson.com/v2/dictionaries/{}/entries?offset={}&limit={}",
            dict, offset, limit
            ).as_str()
        );
    return serde_json::from_str(res.unwrap().as_str()).unwrap();
}

pub fn get_random() -> String {
    let dict = "lasde";
    let sp = SpinnerBuilder::new("Getting random english word from dictionary.".into()).start();

    let entry1 = query(dict, 1, 1);
    let size = entry1.total;
    let offset = rand::thread_rng().gen_range(0, size);

    let entry2 = query(dict, offset, 1);
    sp.message("Done!".into());
    sp.close();

    if entry2.status == 200 {
        let solution: String = entry2.results.into_iter().nth(0).unwrap().headword;
        return solution.to_uppercase();
    }
    println!("Request unsuccessful, returned code {}", entry2.status);
    println!("Falling back to static string");
    return "unimaginatively".to_string().to_uppercase();
}
