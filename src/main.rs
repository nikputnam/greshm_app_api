#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};
use rocket::http::uri::Origin;

use rocket_contrib::json::Json;

use std::time::{SystemTime, UNIX_EPOCH};

pub extern crate crypto;
//use crypto::sha2::Sha256;
extern crate jsonwebtoken as jwt;
//pub extern crate rustc_serialize;

//use self::crypto::digest::Digest;
use self::crypto::sha2::Sha256;

//////////  JWT validation

use jwt::{encode, decode, Header, Algorithm, Validation};

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    name: String,
    iat: u128
}

#[derive(Debug)]
struct ApiKey(String);

#[derive(Serialize, Deserialize,Debug)]
struct Login {
    username: String,
    password: String
}

#[derive(Serialize, Deserialize,Debug)]
struct Transaction {
    from: String,
    to: String,
    amount: f32,
}

/// Returns true if `key` is a valid API key string.
fn is_valid(key: &str) -> bool {
    key == "Bearer valid_api_key"
}

pub fn read_token(key: &str, request: &Request) -> Result<jwt::TokenData<Claims>, jwt::errors::Error> {

    // Changing one parameter
    let mut validation = Validation {leeway: 60, validate_exp: false, validate_nbf: false, ..Default::default()};

//    let token = Token::<Header, Registered>::parse(key);
    println!("{:?}",key);

    let token = decode::<Claims>(&key, "secret_key".as_ref(), &validation);
        println!("{:?}",token);
    token
/*
    match token {
        Err(e) => Err(e),
        Ok(claims) => Ok(claims.sub)
    }
    */
//        .map_err(|_| "Token not valid".to_string())?;
//    println!("{:?}",token);
 //   Ok(token.claims.sub) 
    //.ok_or("Claims not valid".to_string())
/*
    if token.verify(b"secret_key", Sha256::new()) {
        println!("token: {:?}",token);
        let uri: &Origin = request.uri();
        println!("request.uri: {:?}",uri);
        token.claims.sub.ok_or("Claims not valid".to_string())
    } else {
        Err("Token not valid".to_string())
    }
    */
}

#[derive(Debug)]
enum ApiKeyError {
    BadCount,
    Missing,
    Invalid,
}

//adapted from https://api.rocket.rs/v0.4/rocket/request/trait.FromRequest.html
// and from https://github.com/marcocastignoli/rust_rocket_api_authentication/blob/master/src/user/auth.rs 
impl<'a, 'r> FromRequest<'a, 'r> for ApiKey {
    type Error = ApiKeyError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let keys: Vec<_> = request.headers().get("Authorization").collect();
        println!("nkeys:  {}",keys.len());
        println!("key: {:?}",keys);
        //for k in &keys {
        //    println!("{}", k)
        //}
        //println!("{:?}",this)
        if keys.len() != 1 {
            return Outcome::Forward(());
        }

        match read_token(keys[0], request) {
//            Ok(claim) => Outcome::Success(ApiKey(claim.sub)),
            Ok(claim) => { 
                    println!("{:?}",claim.claims); 
                    Outcome::Success(ApiKey(claim.claims.sub.to_string())) 
                },
            Err(_) => Outcome::Forward(())
        }
    }
}

////////////////  JWT generation
    
#[post("/",format = "application/json", data = "<user>")]
fn auth(user: Json<Login>) -> String {
    let sub = format!("id_of_user({})",user.username);
    let name = user.username.clone();
    let now: u128 = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis();
    let my_claims = Claims { sub: sub, name: name,iat: now };
    let token = encode(&Header::default(), &my_claims, "secret_key".as_ref()).unwrap();
    format!("{}\n",token)
}   
    
#[get("/")]
fn balance(key: ApiKey) -> String {
    format!("return the balance for {:?}\n", key )
}  
   
#[get("/")]
fn recent(key: ApiKey) -> String {
    format!("return recent transactions for {:?}\n", key )
}  
   
#[post("/", data = "<tx>"  )]
fn spend(tx: Json<Transaction>, key: ApiKey) -> String {
//    format!("spend {:?}\n", tx )
    format!("spend {:?}\n",tx)
}  
   
fn main() {
    rocket::ignite()
        .mount("/auth", routes![auth])
        .mount("/balance", routes![balance])
        .mount("/recent", routes![recent])
        .mount("/spend", routes![spend])
        .launch();
}     
   