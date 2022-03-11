use rocket::{
    self,
    serde::{json::Json, Deserialize, Serialize},
};

use reqwest::{
    self,
};

const URL: &str = "https://nbg.gov.ge/gw/api/ct/monetarypolicy/currencies/";

#[derive(Deserialize)]
struct IncomeIn{
    currency: String,
    amount: f64,
}

#[derive(Serialize)]
struct IncomeOut{
    value: f64
}

#[derive(Deserialize)]
struct RsCurrencyResponse{
    validFromDate: String,
    code: String,
    rate: f64,
}

#[derive(Deserialize)]         
struct RSCurrencyRateResponse{
    currencies: std::vec::Vec<RsCurrencyResponse>
}


fn get_currency_rate(date: String) -> std::option::Option<f64>{
    // get official curreny on given date
    // @TODO: Convert date from Local to UTC

    let client = reqwest::Client::new();
    let ge_rs_response: std::vec::Vec<RSCurrencyRateResponse> = client.get(URL)
            .query(&[("currencies", "USD"), ("date", &date)])
            .send()?
            .json()?;
    
    
    return validate(ge_rs_response.first(), date);
}

fn validate(rs_currency_rate_response: std::option::Option<RsCurrencyResponse>, date: String) -> std::option::Option<f64>{
    // validate response so we don't use incorrect date, currenyc & more for conversion
    // ensure that conversion is done correctly and don't blindly trust goverment api 
    match rs_currency_rate_response {
        Some(currency_response) => {
            assert_eq!(currency_response.code, "USD"); 
            return Some(currency_response.rate);

        }
        None => return None;
    }

#[rocket::post("/api/monthly_tax", format = "json",  data="<income>")]
fn monthly_tax(income: Json<IncomeIn>) -> Json<IncomeOut>{
    // montlhy tax computation
    return Json(IncomeOut { value : income.amount });
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

