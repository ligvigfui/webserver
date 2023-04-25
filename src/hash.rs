use std::{sync::{Arc, Mutex}};
use crate::lib::{IsHex, extract_anything};
use ripemd::{Ripemd160, Digest};
use webserver::{now, readable_time};

use crate::lib::{User};

pub fn handle_neptun_login_first(messege: &str, users: &Arc<Vec<Mutex<User>>>) -> (String, String) {
    // returns (status, response)
    // get credentials
    let credentials = match extract_anything(messege, "Credentials: ") {
        Some(x) => x,
        None => {
            println!("{}: No credentials found in GET request", readable_time());
            return ("404 Bad Request".to_owned(), "Error 1: No credentials found in GET request\nTry updating the client or contact me at ligvigfui@gmail.com".to_owned());}
    };
    //check if credentials are correct length
    if credentials.len() != 40 {
        println!("{}: Credentials are not correct length" , readable_time());
        return ("404 Bad Request".to_owned(), "Error 2: Credentials are not correct length\nTry updating the client or contact me at ligvigfui@gmail.com".to_owned())
    }
    // check if credentials are hex
    if credentials.is_not_hex() {
        println!("{}: Credentials are not hex" , readable_time());
        return ("404 Bad Request".to_owned(), "Error 3: Credentials are not hex\nTry updating the client or contact me at ligvigfui@gmail.com".to_owned())
    }

    // get mac from Id:
    let id = match extract_anything(messege, "Id: ") {
        Some(x) => x,
        None => {
            println!("{}: No Id found in GET request" , readable_time());
            return ("404 Bad Request".to_owned(), "Error 4: No Id found in GET request\nPlease contact me at ligvigfui@gmail.com".to_owned())}
    };
    // check if id is correct length
    if id.len() != 240 {
        println!("{}: Id is not correct length" , readable_time());
        return ("404 Bad Request".to_owned(), "Error 5: Id is not correct length\nTry updating the client or contact me at ligvigfui@gmail.com".to_owned())
    }
    // check if id is hex
    if id.is_not_hex() {
        println!("{}: Id is not hex" , readable_time());
        return ("404 Bad Request".to_owned(), "Error 6: Id is not hex\nTry updating the client or contact me at ligvigfui@gmail.com".to_owned())
    }

    // get email from credentials
    let email = match get_user_email(users, credentials, true) {
        Some(x) => x,
        None => {
            println!("{}: User does not exist" , readable_time());
            return ("200 Ok".to_owned(), "Error 7: User does not exist\nMeet me in room 211 or write to ligvigfui@gmail.com".to_owned())}
    };

    // get mac from id = hash(mac)
    let mac = match get_mac_from_id(id) {
        Some(x) => x,
        None => {
            println!("{}: Id does not exist" , readable_time());
            return ("200 Ok".to_owned(), "Error 8: Id does not exist".to_owned())}
    };
    
    // set user mac to this
    match set_mac(users, &email, mac) {
        Ok(_) => (),
        Err(e) => {
            println!("{}", e);
            return ("200 Ok".to_owned(), "Error 9: Could not set mac".to_owned())}
    };
    
    

    // send response with email, mac and count & update last login time
    println!("{}: {} logged in" , readable_time() , email);
    let response = response(users, email);
    ("200 OK".to_owned() , response)
}

pub fn handle_neptun_login_other(messege: &str, users: &Arc<Vec<Mutex<User>>>) -> (String, String) {
    // returns (status, response)
    // get credentials
    let credentials = match extract_anything(messege, "Credentials: ") {
        Some(x) => x,
        None => {
            println!("{}: No credentials found in GET request", readable_time());
            return ("404 Bad Request".to_owned(), "Error 1: No credentials found in GET request\nTry updating the client or contact me at ligvigfui@gmail.com".to_owned());}
    };
    //check if credentials are correct length
    if credentials.len() != 40 {
        println!("{}: Credentials are not correct length" , readable_time());
        return ("404 Bad Request".to_owned(), "Error 2: Credentials are not correct length\nTry updating the client or contact me at ligvigfui@gmail.com".to_owned())
    }
    // check if credentials are hex
    if credentials.is_not_hex() {
        println!("{}: Credentials are not hex" , readable_time());
        return ("404 Bad Request".to_owned(), "Error 3: Credentials are not hex\nTry updating the client or contact me at ligvigfui@gmail.com".to_owned())
    }

    // get email from credentials
    let email = match get_user_email(users, credentials, false) {
        Some(x) => x,
        None => {
            println!("{}: User does not exist" , readable_time());
            return ("200 Ok".to_owned(), "Error 7: User does not exist\nMeet me in room 211 or write to ligvigfui@gmail.com".to_owned())}
    };

    // send response with email, mac and count & update last login time
    println!("{}: {} logged in" , readable_time() , email);
    let response = response(users, email);
    ("200 OK".to_owned() , response)
}

fn response(users: &Arc<Vec<Mutex<User>>>, email: String) -> String {
    for user in users.iter() {
        let mut user = user.lock().unwrap();
        if user.email == email {
            let response = format!("tKn.8M{}:{}:{}", user.email, user.MAC, user.count);
            user.count += 2;
            user.time = now();
            return hash(response);
        }
    }
    "".to_owned()
}

fn set_mac<'a>(users: &'a Arc<Vec<Mutex<User>>>, email: &str, mac: String) -> Result<(), &'a str> {
    for user in users.iter() {
        let mut user = user.lock().unwrap();
        if user.email == email && user.time + 5 > now(){
            user.MAC = mac;
            user.count = 1;
            return Ok(());
        }
    }
    Err("User already logged in with these credentials")
}

fn get_mac_from_id(id: String) -> Option<String> {
    let mut mac = String::new();
    for i in 0..6 {
        assert!(i*40+40 <= id.len());
        mac += &get_mac_2_chars(&id[i*40..i*40+40])?;
        mac += ":"; // add : between mac chars
    }
    // remove last :
    mac.pop();
    Some(mac)
}

fn get_mac_2_chars(id_slice: &str) -> Option<String> {
    for int1 in 0..16 {
        for int2 in 0..16 {
            let guess = format!("{}1pN?Qx\\jQvh{}", i32_to_hex_char(int1), i32_to_hex_char(int2));
            let hash = hash(guess);
            if hash == id_slice {
                return Some(format!("{}{}", i32_to_hex_char(int1), i32_to_hex_char(int2)));
            }
        }
    }
    None
}

fn get_user_email(users: &Arc<Vec<Mutex<User>>>, credentials: String , zero_count: bool) -> Option<String> {
    // returns email
    for user in users.iter() {
        let user = user.lock().unwrap();
        let decoded;
        if zero_count {
            decoded = format!("vcdjZVbvLFB1{}:{}:{}", user.email, user.password, 0);
        }
        else {
            decoded = format!("vcdjZVbvLFB1{}:{}:{}", user.email, user.password, user.count-1);
        }
        if hash(decoded) == credentials {
            return Some(user.email.clone());
        }
    }
    None
}

fn hash(string: String) -> String {
    let mut hasher = Ripemd160::new();
    hasher.update(string.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
}




fn i32_to_hex_char(number: i32) -> char {
    match number {
        0 => '0',
        1 => '1',
        2 => '2',
        3 => '3',
        4 => '4',
        5 => '5',
        6 => '6',
        7 => '7',
        8 => '8',
        9 => '9',
        10 => 'a',
        11 => 'b',
        12 => 'c',
        13 => 'd',
        14 => 'e',
        15 => 'f',
        _ => panic!("number is not a hex number"),
    }
}


#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

    use super::*;
    #[test]
    fn extract_anything_test(){
        let str1 = "élnfskbvkaéjds va s\ré md";
        assert_eq!(extract_anything(str1 , "nf").unwrap(),"skbvkaéjds va s".to_string());
    }
    #[test]
    fn login(){
        let users = Arc::new(vec![Mutex::new(User::new("ligvigfui@fsda.capok".to_owned(), "password".to_owned()))]);
        set_mac(&users, "ligvigfui@fsda.capok", "00:00:00:00:00:00".to_owned()).unwrap();
        thread::sleep(Duration::from_secs(3));
        assert!(set_mac(&users, "ligvigfui@fsda.capok", "00:00:00:00:00:01".to_owned()).is_err());
        assert!(set_mac(&users, "ligvigfui@fsda.capok", "00:00:00:00:00:00".to_owned()).is_ok());
        thread::sleep(Duration::from_secs(6));
        assert!(set_mac(&users, "ligvigfui@fsda.capok", "00:00:00:00:00:01".to_owned()).is_ok());
    }
}
