use Auth_service::{Credentials,authenticate};
fn main(){
    let cred = Credentials{
        username: "letgen".to_string(),
        password: "password".to_string(),
    };
    authenticate(cred);
}