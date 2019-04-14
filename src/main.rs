#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
//#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
use std::time::{SystemTime, UNIX_EPOCH};

use rocket::State;
//use rocket_contrib::databases::diesel;
use rocket_contrib::json::Json;

pub extern crate crypto;
extern crate jsonwebtoken as jwt;

pub mod authorize;
pub mod store;

//#[database("sqlite_db")]
//struct Conn(diesel::SqliteConnection);


#[derive(Serialize, Deserialize,Debug, Clone)]
pub struct RawTransaction {
    from: String,
    to: String,
    amount: f32
}

#[derive(Serialize, Deserialize,Debug, Clone)]
pub struct Transaction {
    from: String,
    to: String,
    amount: f32,
    timestamp: u128,
}

impl Transaction {
        pub fn new(from: &String, to: &String, amount: &f32) -> Transaction {
            Transaction { 
                from: from.to_string(), 
                to: to.to_string(), 
                amount: *amount ,
                timestamp: SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis()
                } // this is fine, as we're in the same module
        }
    }

#[derive(Serialize, Deserialize,Debug)]
pub struct Balance {
    pub balance: f32,
    pub balance_time: u128,
}

#[get("/")]
fn balance(store: State<store::Store>, key: authorize::AuthToken) -> Result<Json<Balance>,String>  {

    match store.get_balance_for_username(&(key.0)) {
        Ok(bal) => Ok(Json(bal)),
        Err(e)  => Err(e)
    }
}  
   
#[get("/")]
fn dump(store: State<store::Store>) -> String {

    {
    let ttx = store.txs.read().unwrap();
    println!("txs: {:?}",ttx);
    }
    {
    let users = store.users.read().unwrap();
    println!("users: {:?}",users);
    }

    format!("Ok")
}  

#[get("/")]
fn recent(store: State<store::Store>, key: authorize::AuthToken) -> Json<Vec<Transaction>> {

   let ttx = store.txs.read().unwrap();

    let mut result = Vec::new(); 

    for k in &(*ttx) {
        if !(( k.from == key.0 )||( k.to == key.0 ))  { continue; }
        result.push( k.clone() );
    }
    Json(result)
}  
   
#[post("/", data = "<tx_json>"  )]
fn spend(store: State<store::Store>, tx_json: Json<RawTransaction>, key: authorize::AuthToken ) 
                    -> Result<String,String> {

        println!("{:?}",tx_json);
    //It's an error if the spend is not from the authorized user!
    if ! (tx_json.from == key.0)  { return Err("Unauthorized spend transaction\n".to_string()) }

    let tx = Transaction::new( &tx_json.from, &tx_json.to, &tx_json.amount );
    store.add_spend(tx)

}  
   
fn main() {

    let store = store::Store::new() ;

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
