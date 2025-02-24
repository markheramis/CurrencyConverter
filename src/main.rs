use dotenv::dotenv;
use clap::Parser;

use rust_decimal::Decimal;
use rust_decimal::prelude::*;

use reqwest::Client;
use reqwest::Response;
use serde::Deserialize;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// The amount to convert
    #[arg(short, long)]
    pub amount: Option<Decimal>,

    /// The currency to convert from
    #[arg(short, long)]
    pub from: Option<String>,

    /// The currency to convert to
    #[arg(short, long)]
    pub to: Option<String>
}

#[derive(Deserialize, Debug)]
struct ExchangeRateResponse {
    result: String,
    base_code: String,
    target_code: String,
    conversion_rate: f64,
    time_last_update_utc: String,
    time_next_update_utc: String,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let mut args = Args::parse();
    let exchange_rate_api_key = std::env::var("EXCHANGE_RATE_API_KEY").expect("EXCHANGE_RATE_API_KEY: must be set.");

    if args.amount.is_none() {
        print!("Enter amount: ");
        std::io::Write::flush(&mut std::io::stdout()).expect("Failed to flush stdout");
        let mut amount_input = String::new();
        std::io::stdin().read_line(&mut amount_input).unwrap();
        args.amount = Some(Decimal::from_str(amount_input.trim()).expect("Invalid amount format!"));
    }
    if args.from.is_none() {
        print!("Enter currency to convert from: ");
        std::io::Write::flush(&mut std::io::stdout()).expect("Failed to flush stdout");
        let mut from_input = String::new();
        std::io::stdin().read_line(&mut from_input).unwrap();
        args.from = Some(from_input.trim().to_uppercase());
    }
    if args.to.is_none() {
        print!("Enter currency to convert into: ");
        std::io::Write::flush(&mut std::io::stdout()).expect("Failed to flush stdout");
        let mut to_input = String::new();
        std::io::stdin().read_line(&mut to_input).unwrap();
        args.to = Some(to_input.trim().to_uppercase());
    }
    let url = format!(
        "https://v6.exchangerate-api.com/v6/{}/pair/{}/{}",
        exchange_rate_api_key,
        args.from.clone().unwrap(),
        args.to.clone().unwrap()
    ); 

    let client: Client = reqwest::Client::new();

    let response: Response = client.get(url).send().await.expect("Failed to send request");
    let body: ExchangeRateResponse = response.json().await.expect("Failed to read response body");

    let amount = args.amount.unwrap();
    let conversion_rate = Decimal::from_f64(body.conversion_rate).unwrap();
    let from = args.from.clone().unwrap();
    let to = args.to.clone().unwrap();

    let converted_amount = amount * conversion_rate;
    println!("{} {} is {} {}", amount, from, converted_amount, to);
}
