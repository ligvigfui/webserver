use std::sync::{Arc, Mutex};
use ripemd::{Ripemd160, Digest};

use crate::*;

pub fn handle_neptun_login<'a>(request: Request, users: &'a Arc<Vec<Mutex<User>>>) -> (&'a str, String) {
    let email = match handle_neptun_login_inner(request, users) {
        Ok(x) => x,
        Err(response) => return (CODES[&400], response.to_string()),
    };

    // send response with email, mac and count & update last login time
    println!("{}: {} logged in" , readable_time() , email);
    let response = response(users, email);
    (CODES[&200] , response.to_string())
}

fn handle_neptun_login_inner<'a>(request: Request, users: &'a Arc<Vec<Mutex<User>>>) -> Result<String,&'a str> {
    // get credentials
    let credentials = match request.get_header("Credentials") {
        Some(x) => x.to_string(),
        None => {
            println!("{}: No credentials found in GET request", readable_time());
            return Err("Error 1: No credentials found in GET request\nTry updating the client or contact me at ligvigfui@gmail.com");}
    };
    //check if credentials are correct length
    if credentials.len() != 40 {
        println!("{}: Credentials are not correct length" , readable_time());
        return Err("Error 2: Credentials are not correct length\nTry updating the client or contact me at ligvigfui@gmail.com")
    }
    // check if credentials are hex
    if credentials.is_not_hex() {
        println!("{}: Credentials are not hex" , readable_time());
        return Err("Error 3: Credentials are not hex\nTry updating the client or contact me at ligvigfui@gmail.com")
    }

    // get email from credentials
    let email = match get_user_email(users, credentials, true) {
        Some(x) => x,
        None => {
            println!("{}: User does not exist" , readable_time());
            return Err("Error 4: User does not exist\nMeet me in room 211 or write to ligvigfui@gmail.com")
        }
    };
    
    // get mac from Id:
    let id = match request.get_header("Id") {
        Some(x) => x.to_string(),
        None => return Ok(email),
    };
    // check if id is correct length
    if id.len() != 240 {
        println!("{}: Id is not correct length" , readable_time());
        return Err("Error 5: Id is not correct length\nTry updating the client or contact me at ligvigfui@gmail.com")
    }
    // check if id is hex
    if id.is_not_hex() {
        println!("{}: Id is not hex" , readable_time());
        return Err("Error 6: Id is not hex\nTry updating the client or contact me at ligvigfui@gmail.com")
    }
    
    // get mac from id = hash(mac)
    let mac = match get_mac_from_id(id) {
        Some(x) => x,
        None => {
            println!("{}: Id is not valid" , readable_time());
            return Err("Error 7: Id is not valid\nTry updating the client or contact me at ligvigfui@gmail.com")
        }
    };

    // set user mac to this if this is the first time logging in
    match set_mac(users, &email, mac) {
        Ok(_) => return Ok(email),
        Err(e) => {
            println!("{}", e);
            return Err(e)
        }
    };
}

fn response<'a>(users: &'a Arc<Vec<Mutex<User>>>, email: String) -> String {
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
    //a before 5 sec a = continue count
    //a after 5 sec a = reset count
    //a before 5 sec b = error
    //a after 5 sec b = reset count

    for user in users.iter() {
        let mut user = user.lock().unwrap();
        if user.email != email {
            continue;
        }
        if user.time + 5 > now() {
            if &user.MAC == &mac {
                user.count += 2;
                return Ok(());
            }
            return Err("Error 8: User already logged in with these credentials")
        }
        user.time = now();
        user.MAC = mac;
        user.count = 1;
        return Ok(());
    }
    Err("Error 4: User does not exist\nMeet me in room 211 or write to ligvigfui@gmail.com") // this should never happen
}

fn get_mac_from_id(id: String) -> Option<String> {
    let mut mac = String::new();
    for i in 0..6 {
        assert!(i*40+40 <= id.len());
        mac += &get_mac_2_chars(&id[i*40..i*40+40])?;
        mac += ":"; // add ':' between mac chars
    }
    // remove last ':'
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

fn get_user_email(users: &Arc<Vec<Mutex<User>>>, credentials: String , first_time: bool) -> Option<String> {
    // returns email
    for user in users.iter() {
        let user = user.lock().unwrap();
        let decoded;
        if first_time {
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
    fn users() -> Arc<Vec<Mutex<User>>> {
        Arc::new(vec![Mutex::new(User::new("ligvigfui@fsda.capok".to_owned(), "password".to_string()))])
    }

    #[test]
    fn login_base() {
        let users= users();
        set_mac(&users, "ligvigfui@fsda.capok", "00:00:00:00:00:00".to_string()).unwrap();
        assert!(set_mac(&users, "ligvigfui@fsda.capok", "00:00:00:00:00:00".to_string()).is_ok());
    }

    #[test]
    fn login_multiple(){
        let users = users();
        set_mac(&users, "ligvigfui@fsda.capok", "00:00:00:00:00:00".to_string()).unwrap();
        thread::sleep(Duration::from_secs(3));
        assert!(set_mac(&users, "ligvigfui@fsda.capok", "00:00:00:00:00:01".to_string()).is_err());
    }

    #[test]
    fn login_delayed() {
        let users = users();
        set_mac(&users, "ligvigfui@fsda.capok", "00:00:00:00:00:00".to_string()).unwrap();
        thread::sleep(Duration::from_secs(6));
        assert!(set_mac(&users, "ligvigfui@fsda.capok", "00:00:00:00:00:01".to_string()).is_ok());
    }
}
