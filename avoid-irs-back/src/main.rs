use rocket::{
    self,
    serde::{json::Json, Deserialize, Serialize},
};

use reqwest::{
    self,
};

use std::vec::Vec;
use std::option::Option;

const URL: &str = "https://nbg.gov.ge/gw/api/ct/monetarypolicy/currencies/";

#[derive(Deserialize)]
struct IncomeIn{
    date: String,
    currency: String,
    amount: f64,
}

#[derive(Serialize)]
struct IncomeOut{
    value: f64
}

#[derive(Deserialize)]
struct RsCurrencyResponse{
    code: String,
    rate: f64,
}

#[derive(Deserialize)]         
struct RSCurrencyRateResponse{
    currencies: Vec<RsCurrencyResponse>
}


async fn get_currency_rate(date: String) -> Option<f64>{
    // get official curreny on given date
    // @TODO: Convert date from Local to UTC

    let client = reqwest::Client::new();
    let response = client.get(URL)
            .query(&[("currencies", "USD"), ("date", &date)])
            .send()
            .await
            .unwrap();
    
    
    match response.status(){
        reqwest::StatusCode::OK => {
            match response.json::<Vec<RSCurrencyRateResponse>>().await{
                Ok(responses) => {
                    match responses.first(){
                        Some(response) => {
                            return validate_response_currency(response);
                        },
                        _ => None
                    }
                },
                Err(_) => None
            }
        }
        _ => None 
    }
}

fn validate_response_currency(response: &RSCurrencyRateResponse) -> Option<f64>{
   /*  validate response so we don't use incorrect date, curreny & more for conversion
    *  ensure that conversion is done correctly and don't blindly trust goverment api
    */
    match response.currencies.first(){
        Some(c) => {
            if c.code != "USD" {return None};
            return Some(c.rate);
        },
        _ => None
    }
}

#[rocket::post("/api/monthly_tax", format = "json",  data="<income>")]
async fn monthly_tax(income: Json<IncomeIn>) -> Json<IncomeOut>{
    // montlhy tax computation

    let amount = income.amount.clone();
    match get_currency_rate(income.date.clone()).await{
        Some(currency_rate) => Json(IncomeOut { value : amount * currency_rate }),
        _ => Json(IncomeOut { value : 0.0}) 
    }
}


#[rocket::main]
async fn main(){
    if let Err(err) = rocket::build()
        .mount("/", rocket::routes![monthly_tax])
        .launch()
        .await
    {
       println!("Rocket Rust couldn't take off successfully!");
       drop(err); // Drop initiates Rocket-formatted panic
    }
}
