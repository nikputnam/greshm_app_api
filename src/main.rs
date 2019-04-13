#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket::Outcome;
use rocket::State;
//use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};
//use rocket::http::uri::Origin;
use rocket_contrib::databases::diesel;
use rocket_contrib::json::Json;

use std::sync::RwLock;

pub extern crate crypto;
//use crypto::sha2::Sha256;
extern crate jsonwebtoken as jwt;
//pub extern crate rustc_serialize;

//use self::crypto::digest::Digest;
//se self::crypto::sha2::Sha256;

pub mod authorize;
pub mod store;

#[database("sqlite_db")]
struct Conn(diesel::SqliteConnection);

#[derive(Serialize, Deserialize,Debug)]
pub struct Transaction {
    from: String,
    to: String,
    amount: f32,
}

impl Transaction {
        pub fn new(from: &String, to: &String, amount: &f32) -> Transaction {
            Transaction { from: from.to_string(), to: to.to_string(), amount: *amount } // this is fine, as we're in the same module
        }
    }

#[get("/")]
fn balance(store: State<store::Store>, key: authorize::AuthToken) -> String {
    format!("return the balance for {:?}\n", key )
}  
   
#[get("/")]
fn recent(store: State<store::Store>, key: authorize::AuthToken) -> String {
    format!("return recent transactions for {:?}\n", key )
}  
   
#[post("/", data = "<tx_json>"  )]
fn spend(store: State<store::Store>, tx_json: Json<Transaction>, _key: authorize::AuthToken ) -> Result<String,authorize::AuthTokenError> {

//    println!("{:?}",conn);
    //It's an error if the spend is not from the authorized user!
    if !(tx_json.from == _key.0 ) { return Err(authorize::AuthTokenError::UserMismatch) }

    let tx = Transaction::new( &tx_json.from, &tx_json.to, &tx_json.amount );
//    store.txs.push( tx ) ; 

    let mut ttx = store.txs.write().unwrap();
    println!("{:?}",ttx);
    println!("{:?}",*ttx);
    (*ttx).push(tx);

    Ok(format!("spend {:?}\n",tx_json))
}  
   
fn main() {

    let mut store = store::Store::new() ;

    rocket::ignite()
//        .attach(Conn::fairing())
        .manage(store)
        .mount("/auth", routes![authorize::auth])
        .mount("/balance", routes![balance])
        .mount("/recent", routes![recent])
        .mount("/spend", routes![spend])
        .launch();
}