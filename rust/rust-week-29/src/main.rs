use chrono::{ Utc, Local };
use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    let var = env::var("KEY");
    
    match var {
        Ok(str) => println!("{}", str),
        Err(_e) => println!("Error in reading env")
    }

    // if faster stoppage is needed

    

    let utc = Utc::now();
    let local = Local::now();
    println!("Hello, world!{}{}", utc, local);


}
