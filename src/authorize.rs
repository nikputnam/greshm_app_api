

use rocket::Outcome;
use rocket::request::{self, Request, FromRequest};
use std::time::{SystemTime, UNIX_EPOCH};
use rocket_contrib::json::Json;
use rocket::State;
use rocket::response::status;


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
pub struct AuthToken(pub String);

#[derive(Debug)]
pub struct SpenderMatch(String);

#[derive(Serialize, Deserialize,Debug)]
pub struct Login {
    username: String,
    password: String
}

#[derive(Serialize, Deserialize,Debug)]
pub struct Signup {
    username: String,
    password: String
}

pub fn read_token(key: &str) -> Result<jwt::TokenData<Claims>, jwt::errors::Error> {
    let validation = Validation {
        leeway: 60, 
        validate_exp: false, 
        validate_nbf: false, 
        ..Default::default()};

    let token = decode::<Claims>(&key, "secret_key".as_ref(), &validation);
        println!("{:?}",token);
    token

}

#[derive(Debug)]
pub enum AuthTokenError {
    BadCount,
    Missing,
    Invalid,
    UserMismatch,
}

//adapted from https://api.rocket.rs/v0.4/rocket/request/trait.FromRequest.html
// and from https://github.com/marcocastignoli/rust_rocket_api_authentication/blob/master/src/user/auth.rs 
impl<'a, 'r> FromRequest<'a, 'r> for AuthToken {
    type Error = AuthTokenError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let keys: Vec<_> = request.headers().get("Authorization").collect();

        if keys.len() != 1 {
            return Outcome::Forward(());
        }

        match read_token(keys[0]) {
            Ok(claim) => { 
                    println!("{:?}",claim.claims); 
                    Outcome::Success(AuthToken(claim.claims.sub.to_string())) 
                },
            Err(_) => Outcome::Forward(())
        }
    }
}



////////////////  JWT generation
#[post("/",format = "application/json", data = "<data>")]
pub fn auth(store: State<super::store::Store>, data: Json<Login>) -> Result<String, String> {

    if let Ok(_) = store.valid_password( &data.username, &data.password  ) {
        let sub = format!("{}",data.username);  //TODO: replace this with an internal ID
        let name = data.username.clone();
        let now: u128 = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis();
        let my_claims = Claims { sub: sub, name: name,iat: now };
        let token = encode(&Header::default(), &my_claims, "secret_key".as_ref()).unwrap();
        Ok(format!("{}\n",token))
    } else {
        Err("Authentication failed".to_string())
    }
}   


////////////////  User signup
#[post("/",format = "application/json", data = "<user>")]
pub fn signup(store: State<super::store::Store>, user: Json<Signup>) -> Result<String, status::BadRequest<String> > {

    if store.user_exists(&user.username) { 
        //return Err( "Username already registered".to_string() ) 
        return Err( status::BadRequest(Some("Username already registered.".to_string())));
    }   // User already exists
    
    let new_user = super::store::User { 
        id: 0 ,
        username: user.username.clone(),
        email: user.username.clone(),
        password: user.password.clone(),
        balance: 0.0,
        balance_time: SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis() ,
     } ;

//    let mut users = store.users.write().unwrap();
    match store.add_user(new_user)  {
        Ok(a) => Ok(a),
        Err(e) => Err(status::BadRequest(Some(e.clone())))
    }
  //  (*users).push(new_user);
    //Ok(format!("new user"))
} 




