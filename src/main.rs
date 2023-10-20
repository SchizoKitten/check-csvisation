mod api_token;
use crate::api_token::TOKEN;

use std::{io, error::Error};
use serde::{Deserialize, Serialize};



#[derive(Deserialize)]
#[derive(Debug)]
struct Data{
    data: Json
}

impl Data{
    fn unwrap(self) -> Check{
        return self.data.json;
    }
}

#[derive(Deserialize)]
#[derive(Debug)]
struct Json{
    json: Check
}

#[derive(Deserialize)]
#[derive(Debug)]
#[derive(Clone)]
#[serde(rename_all = "camelCase")]
struct Check{
    items: Vec<Item>,
    date_time: String,
    total_sum: i32,
}

#[derive(Deserialize)]
#[derive(Debug)]
#[derive(Clone)]
struct Item{
    sum: i32,
    name: String,
    quantity: i32,
}

#[derive(Serialize)]
#[derive(Debug)]
struct CsvRecord<'a>{
    date: &'a str,
    id: i32,
    full_account_name: String,
    description: String,
    memo: String,
    value: String,
}

impl CsvRecord<'_>{
    pub fn new(check: Item, id: i32, date: &str) -> CsvRecord{
        CsvRecord {
            date,
            id,
            full_account_name: get_account(&check.name),
            description: "".to_string(),
            memo: format!("{} x{}", check.name, check.quantity),
            value: format!("{}.{}", check.sum / 100, check.sum % 100),
        }
    }
}

fn get_account(_item_name: &str) -> String{
    "".to_string()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    let client = reqwest::Client::new();
    let mut wtr = csv::Writer::from_writer(io::stdout());
    let mut id_count = 1;
    loop{
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Input error");
        let input = match input.strip_prefix("QR-Code:"){
            None => match input.as_str(){
                "" => return Ok(()),
                _ => continue,
            },
            Some(x) => x.trim_end(),
        };
        let req = [("qrraw", input), ("token", TOKEN)];
        let check: Data = client
            .post("https://proverkacheka.com/api/v1/check/get")
            .form(&req)
            .send()
            .await?
            .json()
            .await?;
        let check = check.unwrap();

        let mut date = check.date_time;
        date.truncate(10);

        let mut records: Vec<CsvRecord> = Vec::new();
        let first_rec = CsvRecord{
            date: &date,
            id: id_count,
            full_account_name: "Карта 1".to_string(),
            description: "".to_string(),
            memo: "".to_string(),
            value: format!("-{}.{}", check.total_sum / 100, check.total_sum % 100),
        };
        records.push(first_rec);
        for item in check.items {
            records.push(CsvRecord::new(item, id_count, &date));
        }

        for record in records{
            wtr.serialize(record)?;
        }
        wtr.flush()?;
//        println!("{:?}", records);
        id_count += 1;
    }
}
