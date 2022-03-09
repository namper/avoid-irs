use rocket::{
    self,
    serde::{json::Json, Deserialize, Serialize},
};

use reqwest::{
    self,
}

const URL: string = "https://nbg.gov.ge/gw/api/ct/monetarypolicy/currencies/";

#[derive(Deserialize)]
struct IncomeIn{
    currency: std::Option<String>,
    amount: f64,
}

#[derive(Serialize)]
struct IncomeOut{
    value: f64
}

#[derive(Deserialize)]         
struct RSCurrencyRateResponse{
    currencies: str:Vect<{validFromDate: String, code: String}
}

fn get_currency_rate(date: str) -> f64:
    // get official curreny on given date
    // @TODO: Convert date from Local to UTC

    let client = reqwest::Client::new();
    let ge_rs_response: std::Vec<RSCurrencyRateResponse> = client.get(URL)
            .query(&[("currencies", "USD"), ("date", date_str)])
            .send()?
            .json()?;
    
    
    return validate(ge_rs_response.first(), date)



fn validate(rs_currency_rate_response: RSCurrencyRateResponse, date: str) -> f64:
    // validate response so we don't use incorrect date for conversion
    currency = rs_currency_rate_response.curencies.first();

    assert_eq(currency.validFromDate, date)
    assert_eq(currency.code, "USD")
 
    return currency



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

