use std::{sync::{Arc, Mutex}, fmt::format};
use crate::lib::IsHex;
use ripemd::{Ripemd160, Digest};
use webserver::now;

use crate::lib::{CustomResult, User};

fn decode_credentials(users: &Arc<Vec<Mutex<User>>>, credentials: String) -> (CustomResult, String) {
    //check if string is hex
    if credentials.is_not_hex() {
        return (CustomResult::Br, "nothing".to_string());
    }
    
    //return resutlt and email
    //if result is ok, email is the email
    //if result is wc, email is "nothing".as_string()
    
    
    for user in users.iter() {
        let user = user.lock().unwrap();
        let decoded = format!("{}:{}", user.email, user.password);
        if hash(decoded) == credentials {
            return (CustomResult::Ok, user.email.clone());
        }
        
    }
    (CustomResult::Wc, String::from("nothing"))
}
//


fn encode(result: CustomResult, email: &str) -> String {
    let encoded = format!("{}:{}", &result, email);
    hash(encoded)
}

fn hash(string: String) -> String {
    let mut hasher = Ripemd160::new();
    hasher.update(string.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
}
//------------------------------------------------------------------------------------------------

pub fn get_response(messege: &str, users: Arc<Vec<Mutex<User>>>) -> (String, String) {
    //returns (status, response)

    //separate credentials
    let credentials_start = match messege.find("Credentials: ") {
        Some(index) => index,
        None => {
            // the substring was not found
            println!("Credentials not found in the string");
            return ("400 Bad Request".to_string() , "Credentials not found in the string".to_string());
        }
    };



    // extract the part of the string after "credentials:"
    let credentials = &messege[credentials_start + 13..];
    // find the end of the line
    let line_end = match credentials.find('\r') {
        Some(index) => index,
        None => credentials.len(),
    };
    // extract the credentials string
    let credentials = &credentials[..line_end];
    
    //get hash of email, password from user
    //returns result and user 
    let (result, email) = decode_credentials(&users, credentials.to_owned());
    


    //extract sessionID
    let sessionID_start = match messege.find("SessionID: ") {
        Some(index) => index,
        None => {
            // the substring was not found
            println!("SessionID not found in the string");
            if result == CustomResult::Ok {
                return ("200 Ok".to_string(), encode(CustomResult::Ok, &email));
            }
            else {
                return ("200 Ok".to_string(), encode(CustomResult::Wc, "nothing"));
            }
            return ("400 Bad Request".to_string() , "Credentials not found in the string".to_string());
        }
    };
    // extract the part of the string after "SessionID:"
    let credentials = &messege[credentials_start + 13..];
    // find the end of the line
    let line_end = match credentials.find('\r') {
        Some(index) => index,
        None => credentials.len(),
    };
    // extract the credentials string
    let credentials = &credentials[..line_end];




    //if result is ok
    if result == CustomResult::Ok {
        //return 200 Ok hash(MAC)
        println!("{}: {}", email, CustomResult::Ok);
        return ("200 Ok".to_string(), encode(CustomResult::Ok, &email));
    }
    else {
        //return 200 Ok "wrong credentials"
        println!("{}: {}", email, CustomResult::Wc);
        return ("200 Ok".to_string(), encode(CustomResult::Wc, "nothing"));
    }

}

fn test_users() -> Arc<Vec<Mutex<User>>> {
    Arc::new(vec![
        Mutex::new(User::new(String::from("ligvigfui@gmail.com"), String::from("hali0123"))),
        Mutex::new(User::new(String::from("regő@regő.hu"), String::from("hali"))),
        Mutex::new(User::new(String::from("öüóőúűéáí@regő.hu"), String::from("hali")))
    ])
}



fn i32_to_hex_char(number: i32) -> String {
    match number {
        0 => String::from("0"),
        1 => String::from("1"),
        2 => String::from("2"),
        3 => String::from("3"),
        4 => String::from("4"),
        5 => String::from("5"),
        6 => String::from("6"),
        7 => String::from("7"),
        8 => String::from("8"),
        9 => String::from("9"),
        10 => String::from("a"),
        11 => String::from("b"),
        12 => String::from("c"),
        13 => String::from("d"),
        14 => String::from("e"),
        15 => String::from("f"),
        _ => panic!("number is not a hex number"),
    }
}


#[cfg(test)]
mod tests {

    #[test]
    fn test_decode_credentials(){
    }
    #[test]
    fn decode_MAC(){

    }
    #[test]
    fn can_login(){

    } 
    #[test]
    fn encode (){

    }

}