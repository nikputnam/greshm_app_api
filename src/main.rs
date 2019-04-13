#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
//#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket::Outcome;
//use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};
//use rocket::http::uri::Origin;

use rocket_contrib::json::Json;


pub extern crate crypto;
//use crypto::sha2::Sha256;
extern crate jsonwebtoken as jwt;
//pub extern crate rustc_serialize;

//use self::crypto::digest::Digest;
//se self::crypto::sha2::Sha256;

pub mod authorize;

//use crate::authorize;

#[derive(Serialize, Deserialize,Debug)]
struct Transaction {
    from: String,
    to: String,
    amount: f32,
}

#[get("/")]
fn balance(key: authorize::AuthToken) -> String {
    format!("return the balance for {:?}\n", key )
}  
   
#[get("/")]
fn recent(key: authorize::AuthToken) -> String {
    format!("return recent transactions for {:?}\n", key )
}  
   
#[post("/", data = "<tx>"  )]
fn spend(tx: Json<Transaction>, _key: authorize::AuthToken ) -> Result<String,authorize::AuthTokenError> {

    //It's an error if the spend is not from the authorized user!
    if !(tx.from == _key.0 ) { return Err(authorize::AuthTokenError::UserMismatch) }

    Ok(format!("spend {:?}\n",tx))
}  
   
fn main() {
    rocket::ignite()
        .mount("/auth", routes![authorize::auth])
        .mount("/balance", routes![balance])
        .mount("/recent", routes![recent])
        .mount("/spend", routes![spend])
        .launch();
}