use dotenv::dotenv;
use clap::Parser;

use rust_decimal::Decimal;
// use rust_decimal::prelude::*;

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


fn main() {
    dotenv().ok();
    let mut args = Args::parse();
    let exchange_rate_api_key = std::env::var("EXCHANGE_RATE_API_KEY").expect("EXCHANGE_RATE_API_KEY: must be set.");

    if args.amount.is_none() {
        print!("Enter amount: ");
        let mut amount_input = String::new();
        std::io::stdin().read_line(&mut amount_input).unwrap();
        args.amount = Some(Decimal::from(amount_input.trim()).expect("Invalid amount format!"));
    }
    //match args.to {
    //    Some(to) => {
    //        println!("Currency to: {}", to);
    //    }
    //    None => {
    //        println!("Currency to not set");
    //    }
    //}
    //match args.from {
    //    Some(from) => {
    //        println!("Currency from: {}", from);
    //    }
    //    None => {
    //        println!("Currency from not set");
    //    }
    //}
    println!("Args: {:?}", &args);
}
