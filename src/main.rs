#[macro_use] extern crate rocket;

#[get("/")]
fn hello(){

}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
    .mount("/", routes![hello])
    .launch()
    .await;
}
