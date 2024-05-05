use soroban_sdk::{contracttype, Address};

#[derive(Clone)]
#[contraacttype]
pub struct HealthData{
    pub file_no: u64,
    pub patient_name: String,
    pub disease: String,
    pub gender: String,
    pub blood_grp: String,
    pub dr_name: String,
}


#[derive(Clone)]
#[contraacttype]
pub struct DataKey{
    Admin,
    PatientData(HealthData),
}