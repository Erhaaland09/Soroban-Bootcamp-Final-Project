use crate::storage_types::DataKey;

use soroban_sdk::{Address,Env, symbol_short, Symbol,log};

pub fn is_admin(e:&Env) -> Address{
    let key = DataKey::Admin;
    e.storage().instance().get(&key).unwrap()
}

pub fn read_admin(e:&Env) -> Address{
    let key= DataKey::Admin;
    e.storage().instance().get(&key).unwrap()
}

pub fn set_admin(e:&Env, id:&Address){
    let key = DataKey::Admin;
    e.storage().instance().set(&key, id);
}

fn set_new_admin(e:&Env, admin:Address, new_admin:Address){
    let topics = (symbol_short!("set_new_admin"),admin);
    e.events().publish(topics, new_admin);
}