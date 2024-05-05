use soroban_sdk::{Bytes,Env};
use soroban_token_sdk::{TokenMetadata, TokenUtils};

pub fn read_name(e:&Env) -> Bytes{
    let util = TokenUtils::new(e);
    util.get_metadata_unchecked().unwrap().name
}

pub read_symbol(e:&Env) -> Bytes{
    let util = TokenUtils::new(e);
    util.get_metadata_unchecked().unwrap().symbol
}