use std::sync::RwLock;

#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub balance: f32,
    pub balance_time: u128,
}

/*
pub struct Transaction {
    from: String,
    to: String,
    amount: f32
}
*/

pub struct Store {

    pub users: RwLock<Vec<User>>,
    pub txs: RwLock<Vec<super::Transaction>>,

}

impl Store {
        pub fn new() -> Store {
            Store { 
                users: RwLock::new( vec![] ), 
                txs: RwLock::new( vec![] )
             } 
        }
    }

