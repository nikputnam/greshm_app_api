use std::sync::RwLock;
//use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub balance: f32,
    pub balance_time: u128,
}

pub struct Store {

     users: RwLock<Vec<User>>,
     txs: RwLock<Vec<super::Transaction>>,
     mint_rate: RwLock<f32>,

}

impl Store {
    pub fn new() -> Store {
        Store { 
            users:     RwLock::new( vec![] ), 
            txs:       RwLock::new( vec![] ),
            mint_rate: RwLock::new( 0.001 )

            } 
    }

    pub fn user_exists( &self, username: &String) -> bool {
        let users = self.users.read().unwrap();
        for user in &(*users) {
            if  user.username == *username  {
                return true
            }
        }
        return false
    }

    pub fn get_balance_for_username( &self, username: &String) -> Result<super::Balance,String> {
        println!("get balance for {}", username);
        let users = self.users.read().unwrap();
        let mut b=0.0;
        let mut time=0;
        let mut found=false;
        for user in &(*users) {
            if  user.username == *username  {
                found=true;
                time = user.balance_time.clone();
                b    = user.balance.clone();
            }
        }
//        if !found { return Err(super::authorize::AuthTokenError::UserMismatch)  }
        if !found {println!("not found"); return Err("No such user".to_string()) ;} ;
        println!("found");

        Ok( super::Balance { balance: b, balance_time: time } )

    }

    pub fn valid_password( &self, username: &String, password: &String) -> Result<String,String> {

    //validity = store.valid_password( data.username, data.password  );

        let users = self.users.read().unwrap();

        let mut matched = false;

        for u in &(*users) {
            if ( u.username == *username ) && ( u.password == *password ) { matched = true; }
        }

        if !matched { return Err( "Unknown user/pw pair".to_string() ) } 
        return Ok("password ok".to_string())
    }


    pub fn add_user( &self, new_user: User) -> Result<String,String> {
        let mut users = self.users.write().unwrap();
  //  store.add_user(new_user);
        (*users).push(new_user);
        Ok("added user".to_string())
    }

    pub fn get_recent_txs( &self, user: String) -> Result< Vec<super::Transaction> ,String> {

        let ttx = self.txs.read().unwrap();
        let mut result = Vec::new(); 

        for k in &(*ttx) {
            if !(( k.from == user )||( k.to == user ))  { continue; }
            result.push( k.clone() );
        }
        Ok(result)
    }

    pub fn add_spend( &self, tx: super::Transaction) -> Result<String,String> {

        if ! self.user_exists(&tx.to) { return Err("Recipient unknown\n".to_string()) }

        let mut ttx   = self.txs.write().unwrap();       //get both locks so they stay in sync.
        let mut users = self.users.write().unwrap();
        let mint_rate = self.mint_rate.read().unwrap();

        let now: u128 = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis();

        //Check for sufficient funds
        for user in  &mut(*users) {
            if  user.username == tx.from  {
                let time_delta : f32 = (now - user.balance_time) as f32;
                if (user.balance + time_delta * (*mint_rate) ) < tx.amount {
                    return Err("insufficient funds\n".to_string())
                }
            }
        }

        for user in  &mut(*users) {
            if  user.username == tx.from  {
                user.balance = user.balance - tx.amount;
            }
            if user.username == tx.to  {
                user.balance = user.balance + tx.amount;
            }
        }
        (*ttx).push(tx);

        Ok("added spend".to_string())
    }



}

