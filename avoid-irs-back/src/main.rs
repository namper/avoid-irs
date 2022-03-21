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
    currencies: std::vec::Vec<RsCurrencyResponse>
}


async fn get_currency_rate(date: String) -> f64{
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
            match response.json::<std::vec::Vec<RSCurrencyRateResponse>>().await{
                Ok(parsed_resp) => return validate(parsed_resp.first()),
                Err(error) => panic!("Error while parsing: {:?}", error)
            }
        }
        other => {
            panic!("Uh oh! Something unexpected happened: {:?}", other);
        }
    }
}

fn validate(rs_currency_rate_response: std::option::Option<&RSCurrencyRateResponse>) -> f64{
    // validate response so we don't use incorrect date, currenyc & more for conversion
    // ensure that conversion is done correctly and don't blindly trust goverment api 
    match rs_currency_rate_response {
        Some(currency_response) => {
            let currency = currency_response.currencies.first();

            match currency{
                Some(c) => {
                    assert_eq!(c.code, "USD"); 
                    return c.rate;
                }
                _ => panic!("Empty data")
            }

        }
        _ => panic!("Empty response")
    }
}

#[rocket::post("/api/monthly_tax", format = "json",  data="<income>")]
async fn monthly_tax(income: Json<IncomeIn>) -> Json<IncomeOut>{
    // montlhy tax computation

    let amount = income.amount.clone();
    let currency_rate = get_currency_rate(income.date.clone()).await;

    return Json(IncomeOut { value : amount * currency_rate });
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
