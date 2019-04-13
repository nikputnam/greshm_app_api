use std::sync::RwLock;


pub struct User {
    id: i32,
    name: String,
    email: String,
    password: String
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

