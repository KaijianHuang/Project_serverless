use actix_web::{get, web::ServiceConfig};
use shuttle_actix_web::ShuttleActixWeb;

#[get("/")]
async fn hello_world() -> &'static str {
    "Hello! select a fact you want to know about, and add it to the end of your url: /ostrich, /seahorses, /elephants, /clownfish !"
}
#[get("/ostrich")]
async fn ostrich() -> &'static str {
    "The feathery flightless muscular legs of ostrich are powerful enough to kill man and lion alike."
}
#[get("/seahorses")]
async fn seahorses() -> &'static str {
    "Seahorses are monogamous and mate for life."
}
#[get("/elephants")]
async fn elephants() -> &'static str {
    "Most elephants weigh less than the tongue of a blue whale."
}
#[get("/clownfish")]
async fn clownfish() -> &'static str {
    "All clownfish are born male, and will only change sex to become a dominant female."
}
#[shuttle_runtime::main]
async fn actix_web(
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello_world).service(ostrich).service(seahorses).service(elephants).service(clownfish);
    };

    Ok(config.into())
}