use rocket::{
    self,
    serde::{json::Json, Deserialize, Serialize},
};


#[derive(Deserialize)]
struct IncomeIn{
    currency: String,
    amount: f64
}

#[derive(Serialize)]
struct IncomeOut{
    value: f64
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

