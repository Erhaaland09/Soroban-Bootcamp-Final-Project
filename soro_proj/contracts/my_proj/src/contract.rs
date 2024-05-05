use crate::admin::{is_admin,set_admin};
use crate::metadata::{read_name, read_symbol};

use soroban_sdk::{contractimpl, Address,Env,String,log,symbol_short};
use soroban_token_sdk::TokenMetadata;

const PAT_STATE:SYmbol = symbol_short!("PATIENT")

pub PatientTrait{
    fn initialize(e:Env, admin:Address);

    fn read_data(e:Env, id: Address) -> HealthData;

}



#[contract]
pub struct HealthDapp;

#[contractimpl]
impl PatientTrait for HealthDapp{
    fn initialize(e:Env, admin:Address){
        if is_admin(&e){
            panic!("alreay created admin and initialized Dapp...")
        }
        set_admin(&e, &admin);
    }

    fn set_admin(e:Env, new_admin:Address){
        let admin = read_admin(&e);
        admin.require_auth();
        set_admin(&e, &new_admin);
        admin::set_new_admin(&e, admin, new_admin);
    }

    fn read_patient_data(e:Env ) -> PatientTrait{
        env.storage().instance().get(&)
    }
}
