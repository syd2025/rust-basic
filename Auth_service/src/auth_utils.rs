pub fn login(creds: models::Credentials){
    crate::database::get_user();
}

fn logout(){
    println!("Logging out");
}

pub mod models;