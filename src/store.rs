use std::sync::RwLock;
//use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};
extern crate futures; 
extern crate foundationdb;
use foundationdb::transaction::RangeOptionBuilder;

use futures::future::*;
//use crate::store::foundationdb::tuple::Encode;
use foundationdb::tuple::{Decode, Encode};

//use rand::prelude::*;
use rand::Rng;

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
     income_rate: RwLock<f32>,
     network: foundationdb::network::Network,
     handle: std::thread::JoinHandle<()>,
     db: foundationdb::Database 
}

fn from_slice(bytes: &[u8]) -> [u8; 16] {
    let mut array = [0; 16];
    let bytes = &bytes[..array.len()]; // panics if not enough data
    array.copy_from_slice(bytes); 
    array
}

impl Store {
    pub fn new() -> Store {

        let n = foundationdb::init().expect("failed to initialize Fdb client");
        let h = std::thread::spawn(move || { 
            let error = n.run();
            if let Err(error) = error { panic!("fdb_run_network: {}",error)}

         });
        n.wait();

        let config_path="docker.cluster";   
        let d = foundationdb::Cluster::new(config_path)
            .and_then(|cluster| cluster.create_database())
            .wait().expect("failed to create Cluster");

       Store { 
            users:     RwLock::new( vec![] ), 
            txs:       RwLock::new( vec![] ),
            income_rate: RwLock::new( 0.00001 ),
            network: n,
            handle: h,
            db: d
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
        let rate = self.income_rate.read().unwrap();
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

        Ok( super::Balance { balance: b, balance_time: time, income_rate: *rate } )

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

// from the database


        let timestamp = SystemTime::now();
        let now: u128 = timestamp.duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis();
        let nowb = now.to_be_bytes().to_vec() ; //. to_str(); 

        //let trx = self.db.create_trx().expect("failed to create transaction");
        //let result = trx.get(b"xxx",false).wait().expect("failed to read back random bytes");
        //let value: &[u8] = result.value().expect("couldn't unpack results.");
        //let recv_range = RangeOptionBuilder::from(("greshm", "recv", &user )).build();

        for ranget in [ "send", "recv"].iter() { 
            let range = RangeOptionBuilder::from(("greshm", ranget, &user )).build();
            //println!("range: {:?}", range);

            for key_value in self.db.create_trx()
                .unwrap()
                .get_range(range, 1_024)
                .wait()
                .expect("get_range failed")
                .key_values()
                .into_iter()
            {
            //println!("k {:?}", key_value.key());
            //println!("v {:?}", key_value.value());

                let (_, sr, from, timestamp_bytes, to) = <(String, String, String, Vec<u8>, String)>::try_from(key_value.key()).unwrap();
                let timestamp = u128::from_be_bytes(from_slice(&timestamp_bytes));
                let (amount,) = <(f32,)>::try_from(key_value.value()).unwrap();
                //let amount = <String>::try_from(key_value.value()).unwrap();
//                println!("{} {} {}", from, sr, to);

            println!("got a tx: {} {} {} {} {} {:?}", ranget, from, to, amount, timestamp, timestamp_bytes)
            }
        }

// from the in-memory store
        let ttx = self.txs.read().unwrap();
        let mut result = Vec::new(); 

        for k in &(*ttx) {
            if !(( k.from == user )||( k.to == user ))  { continue; }
            result.push( k.clone() );
            println!("mem tx: {:?}", k);
        }

        Ok(result)
    }

    pub fn database_healthy(&self) -> bool {

        let dbtrx = self.db.create_trx().expect("failed to create transaction");
        dbtrx.set_option(  foundationdb::options::TransactionOption::Timeout(3000) );

        let random_bytes = rand::thread_rng().gen::<[u8; 32]>();

        println!("random bytes {:?}", random_bytes);


        dbtrx.set(b"xxx" , &random_bytes ); // errors will be returned in the future result

        dbtrx.commit()
            .wait()
            .expect("failed to set random bytes");

        let trx = self.db.create_trx().expect("failed to create transaction");
        let result = trx.get(b"xxx",false).wait().expect("failed to read back random bytes");
        let value: &[u8] = result.value().expect("couldn't unpack results.");
        println!("retrieved bytes {:?}", value);

        value == random_bytes
    //true

    }

    pub fn add_spend( &self, tx: super::Transaction) -> Result<String,String> {

        println!("###add spend ###");

        if ! self.user_exists(&tx.to) { return Err("Recipient unknown\n".to_string()) }

        let mut ttx   = self.txs.write().unwrap();       //get both locks so they stay in sync.
        let mut users = self.users.write().unwrap();
        let income_rate = self.income_rate.read().unwrap();

        let timestamp = SystemTime::now();
        let now: u128 = timestamp.duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis();

        //Check for sufficient funds
        for user in  &mut(*users) {
            if  user.username == tx.from  {
                let time_delta : f32 = (now - user.balance_time) as f32;
                if (user.balance + time_delta * (*income_rate) ) < tx.amount {
                    println!("###insufficient funds###");

                    return Err("insufficient funds\n".to_string())
                }
            }
        }
        println!("###funds ok###");

        for user in  &mut(*users) {
            if  user.username == tx.from  {
                user.balance = user.balance - tx.amount;
            }
            if user.username == tx.to  {
                user.balance = user.balance + tx.amount;
            }
        }

        let dbtrx = self.db.create_trx().expect("failed to create transaction");

        let nowb = now.to_be_bytes().to_vec() ; //. to_str(); 
        let txc = tx.clone();  // because the next line takes partial ownership of the strings or something 
//        let key = &( b"greshm", b"send", &nowb , &txc.from , &txc.to ).to_vec();
        let key1 = &( "greshm", "send", &txc.from,  &nowb, &txc.to ).to_vec();
        let key2 = &( "greshm", "recv", &txc.to,  &nowb, &txc.from ).to_vec();
        dbtrx.set(key1 , &txc.amount.to_vec() ); // errors will be returned in the future result
        dbtrx.set(key2 , &txc.amount.to_vec() ); // errors will be returned in the future result

        println!("store: {:?} => {}", key1,  txc.amount);


        dbtrx.commit()
            .wait()
            .expect("failed to commit transaction to db");

        (*ttx).push(tx);
        Ok("added spend".to_string())
    }



}

