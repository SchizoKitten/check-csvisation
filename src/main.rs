use std::io;
use serde::{Deserialize, Serialize};
//use check_csvisation::{answer, categorise, csv, parse};

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

const INPUT: [(&str, &str); 2] = [("qrraw", "t=20230925T1626&s=369.00&fn=9961440300825334&i=32808&fp=634850959&n=1"), ("token", "23978.O0NclvxcsqOu9thkz")];

#[tokio::main]
async fn main() -> Result<(), reqwest::Error>{
    let client = reqwest::Client::new();
    loop{
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Input error");
        let req = [("qrraw", input.strip_prefix("QR-Code:").unwrap().trim_end()), ("token", "23978.O0NclvxcsqOu9thkz")];
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
