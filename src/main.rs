mod api_token;
use crate::api_token::TOKEN;

use std::io;
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
#[serde(rename_all = "camelCase")]
struct Check{
    items: Vec<Item>,
    date_time: String,
    total_sum: i32,
}

#[derive(Deserialize)]
#[derive(Debug)]
struct Item{
    sum: i32,
    name: String,
    price: i32,
    quantity: i32,
}


#[tokio::main]
async fn main() -> Result<(), reqwest::Error>{
    let client = reqwest::Client::new();
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
        let req_answer: Data = client
            .post("https://proverkacheka.com/api/v1/check/get")
            .form(&req)
            .send()
            .await?
            .json()
            .await?;
//        let info = parse(answer);
//        let final_info = categorise(info);
//        let csv_out = csv(final_info);
//        let output = csv_out;
        println!("{:?}", req_answer.unwrap());
    }
}
