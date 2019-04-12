#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};
use rocket::http::uri::Origin;

use rocket_contrib::json::Json;

pub extern crate crypto;
//use crypto::sha2::Sha256;
pub extern crate jwt;
//pub extern crate rustc_serialize;

//use self::crypto::digest::Digest;
use self::crypto::sha2::Sha256;

//////////  JWT validation

use self::jwt::{
    Header,
    Registered,
    Token,
};

#[derive(Debug)]
struct ApiKey(String);

#[derive(Serialize, Deserialize,Debug)]
struct Login {
    username: String,
    password: String
}

/// Returns true if `key` is a valid API key string.
fn is_valid(key: &str) -> bool {
    key == "Bearer valid_api_key"
}

pub fn read_token(key: &str, request: &Request) -> Result<String, String> {
    let token = Token::<Header, Registered>::parse(key)
        .map_err(|_| "Unable to parse key".to_string())?;
    if token.verify(b"secret_key", Sha256::new()) {
        println!("token: {:?}",token);
        let uri: &Origin = request.uri();
        println!("request.uri: {:?}",uri);
        token.claims.sub.ok_or("Claims not valid".to_string())
    } else {
        Err("Token not valid".to_string())
    }
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
        //for k in &keys {
        //    println!("{}", k)
        //}
        //println!("{:?}",this)
        if keys.len() != 1 {
            return Outcome::Forward(());
        }
        match read_token(keys[0], request) {
            Ok(claim) => Outcome::Success(ApiKey(claim)),
            Err(_) => Outcome::Forward(())
        }
    }
}

////////////////  JWT generation

#[post("/",format = "application/json", data = "<user>")]
fn auth(user: Json<Login>) -> String {
    format!("Hello {:?}\n",user)
}

#[get("/<name>/<age>")]
fn user(name: String, age: u8, key: ApiKey) -> String {
    format!("Hello, {} year old named {}!  {:?}\n", age, name, key )
}

fn main() {
    rocket::ignite()
        .mount("/auth", routes![auth])
        .mount("/", routes![user])
        .launch();
}
