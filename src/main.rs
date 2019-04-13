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

#[derive(Serialize, Deserialize,Debug, Clone)]
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

#[derive(Serialize, Deserialize,Debug)]
pub struct Balance {
    pub balance: f32,
    pub balance_time: u128,
}

#[get("/")]
fn balance(store: State<store::Store>, key: authorize::AuthToken) -> Result<Json<Balance>,authorize::AuthTokenError>  {

    let mut users = store.users.read().unwrap();
    let mut b=0.0;
    let mut time=0;
    let mut found=false;
    for user in &(*users) {
        if ( user.username == key.0 ) {
            found=true;
            time = user.balance_time.clone();
            b    = user.balance.clone();
        }
    }
    if (!found) { return Err(authorize::AuthTokenError::UserMismatch)  }
    Ok(Json( Balance { balance: b, balance_time: time } ))

//    format!("return the balance for {:?}\n", key )
}  
   
#[get("/")]
fn dump(store: State<store::Store>) -> String {

    {
    let mut ttx = store.txs.read().unwrap();
    println!("txs: {:?}",ttx);
    }
    {
    let mut users = store.users.read().unwrap();
    println!("users: {:?}",users);
    }

    format!("Ok")
}  

#[get("/")]
fn recent(store: State<store::Store>, key: authorize::AuthToken) -> Json<Vec<Transaction>> {

   let mut ttx = store.txs.read().unwrap();
//    println!("{:?}",ttx);
//    println!("{:?}",*ttx);

    let mut result = Vec::new(); 

    for k in &(*ttx) {
        if !(( k.from == key.0 )||( k.to == key.0 ))  { continue; }
        result.push( Transaction::new(&k.from, &k.to, &k.amount) );
    }

    Json(result)
}  
   
#[post("/", data = "<tx_json>"  )]
fn spend(store: State<store::Store>, tx_json: Json<Transaction>, key: authorize::AuthToken ) 
                    -> Result<String,authorize::AuthTokenError> {

        println!("{:?}",tx_json);
    //It's an error if the spend is not from the authorized user!
    if !(tx_json.from == key.0 ) { return Err(authorize::AuthTokenError::UserMismatch) }

    let tx = Transaction::new( &tx_json.from, &tx_json.to, &tx_json.amount );
    {
        let mut ttx = store.txs.write().unwrap();
        (*ttx).push(tx.clone());
    }

    {
            let mut users = store.users.write().unwrap();
            for user in  &mut(*users) {
                if ( user.username == tx.from ) {
                    user.balance = user.balance - tx.amount;
                }
                if ( user.username == tx.to ) {
                    user.balance = user.balance + tx.amount;
                }
            }
    }

    Ok(format!("spend {:?}\n",tx_json))
}  
   
fn main() {

    let mut store = store::Store::new() ;

    rocket::ignite()
//        .attach(Conn::fairing())
        .manage(store)
        .mount("/auth", routes![authorize::auth])        // log in
        .mount("/signup", routes![authorize::signup])  // create a new account
        .mount("/balance", routes![balance])  // get user balance 
        .mount("/recent", routes![recent])  // get user's transactions
        .mount("/spend", routes![spend])  // submit a TX
        .mount("/dump", routes![dump])  // debug:  triggers dump of datastore to the server console log
        .launch();
}