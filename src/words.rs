extern crate serde_json;

extern crate hyper;
extern crate rand;
extern crate spinner;

use std::error::Error;
use std::io::Read;
use self::hyper::{Client};
use self::rand::Rng;
use self::spinner::{SpinnerBuilder};

#[derive(Serialize, Deserialize)]
struct Data {
    status: u32,
    total: u64,
    results: Vec<Entry>,
}

#[derive(Serialize, Deserialize)]
struct Entry {
    headword: String,
}

fn get_content(url: &str) -> hyper::Result<String> {
    let client = Client::new();
    let mut response = client.get(url).send()?;
    let mut buf = String::new();
    response.read_to_string(&mut buf)?;
    Ok(buf)
}

fn query(dict: &str, offset: u64, limit: i8) -> Result<Data, Box<Error>> {
    let res = try!(get_content(
        format!(
            "http://api.pearson.com/v2/dictionaries/{}/entries?offset={}&limit={}",
            dict, offset, limit
            ).as_str()
        ));
    let data: Data = try!(serde_json::from_str(res.as_str()));
    Ok(data)
}

fn get_static_answers() -> Vec<String> {
    return vec![
        "unimaginatively".into(),
        "interestingly".into(),
        "random".into(),
        "unfair".into(),
        "computer".into(),
    ];
}

pub fn get_random() -> String {
    let dict = "lasde";
    let sp = SpinnerBuilder::new("Getting random english word from dictionary.".into()).start();
    let static_answers = get_static_answers();

    let query1 = query(dict, 1, 1);
    let size = match query1 {
        Ok(v) => v.total,
        Err(_) => static_answers.len() as u64,
    };
    let offset = rand::thread_rng().gen_range(0, size);

    let query2 = query(dict, offset, 1);
    let solution: String = match query2 {
        Ok(v) => {
            sp.message("Done!".into());
            sp.close();
            let solution: String = v.results.into_iter().nth(0).unwrap().headword;
            solution.to_uppercase()
        },
        Err(e) => {
            println!("Error: {}", e);
            sp.message("Request unsuccessful (no internet connection?)".into());
            println!("Falling back to static string");
            sp.close();
            static_answers.get(offset as usize).unwrap().to_uppercase()
        },
    };
    return solution;
}
