extern crate serde_json;

extern crate hyper;

use std::io::Read;
use self::hyper::{Client};

#[derive(Serialize, Deserialize)]
struct Entry {
    status: u8,
    total: i64,
}

fn get_dict_size(dict: &str) -> i64 {
    let offset = 1;
    let limit = 1;
    let res = get_content(
        format!(
            "http://api.pearson.com/v2/dictionaries/{}/entries?offset={}&limit={}",
            dict, offset, limit
            ).as_str()
        );
    let entry: Entry = res_to_entry(res);
    return entry.total;
}

fn res_to_entry(res: hyper::Result<String>) -> Entry {
    return serde_json::from_str(res.unwrap().as_str()).unwrap();
}

// TODO: Get size of dictionary and choose a random offset to request
pub fn get_random() -> String {
    println!("Getting random english word from dictionary ...");

    let res = get_content("http://api.pearson.com/v2/dictionaries/lasde/entries?offset=21176&limit=1");
    let entry: Entry = res_to_entry(res);
    if entry.status == 200 {
        println!("Request successful!");
        return "imaginatively".to_string().to_uppercase();
    }
    println!("Request unsuccessful, returned code {}", entry.status);
    println!("Falling back to static string");
    return "unimaginatively".to_string().to_uppercase();
}

fn get_content(url: &str) -> hyper::Result<String> {
    let client = Client::new();
    let mut response = client.get(url).send()?;
    let mut buf = String::new();
    response.read_to_string(&mut buf)?;
    Ok(buf)
}
