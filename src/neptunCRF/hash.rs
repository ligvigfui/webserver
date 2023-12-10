use ripemd::{Ripemd160, Digest};

use crate::*;

pub fn handle_neptun_login(stream: &mut TcpStream, request: &Request, users: Arc<Vec<Mutex<User>>>) {
    if DEBUG >= DebugLevel::HIGH {
        println!("Request: {:?}", request);
    }
    let (code, response) = match handle_neptun_login_inner(request, &users) {
        Ok(user) => {
            println!("{}: {} logged in" , readable_time() , user.lock().unwrap().email);
            let response = response(user);
            (200, response)
        },
        Err(response) => (400, response.to_string())
    };
    default_handle(stream, &CODE[&code], None, &response)
}

fn handle_neptun_login_inner<'a>(request: &Request, users: &'a Arc<Vec<Mutex<User>>>) -> Result<&'a Mutex<User>,&'a str> {
    // get credentials
    let credentials = match request.headers.get("Credentials") {
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

    // get user from credentials
    let user = match get_user(users, credentials, request.headers.get("Id").is_some()) {
        Some(x) => x,
        None => {
            println!("{}: User does not exist" , readable_time());
            return Err("Error 4: User does not exist\nMeet me in room 211 or write to ligvigfui@gmail.com")
        }
    };
    
    // get mac from Id:
    // if successful: this is the first time this user is logging in
    let id = match request.headers.get("Id") {
        Some(x) => x.to_string(),
        None => return Ok(user),
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
    match set_mac(&user, mac) {
        Ok(_) => return Ok(user),
        Err(e) => {
            println!("{}", e);
            return Err(e)
        }
    };
}

fn response<'a>(users: &'a Mutex<User>) -> String {
    let mut user = users.lock().unwrap();
    let response = format!("tKn.8M{}:{}:{}", user.email, user.MAC, user.count);
    user.count += 2;
    user.time = now();
    hash(response)
}

fn set_mac<'a>(user: &Mutex<User>, mac: String) -> Result<(), &'a str> {
    //a before 5 sec b = error
    //else reset count
    let mut user = user.lock().unwrap();

    if user.time + 5 > now() && &user.MAC != &mac {
        return Err("Error 8: A user already logged in with these credentials\nIf you think you didn't give your credentials to anyone else, meet me in room 211 or write to ligvigfui@gmail.com for a new password\nIf this is not the case then don't try to cheat the system! ty :)")
    }
    user.time = now();
    user.MAC = mac;
    user.count = 1;
    return Ok(())
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

fn get_user(users: &Arc<Vec<Mutex<User>>>, credentials: String , first_time: bool) -> Option<&Mutex<User>> {
    users.iter().find(|user| {
        let user = user.lock().unwrap();
        let decoded;
        if first_time {
            decoded = format!("vcdjZVbvLFB1{}:{}:{}", user.email, user.password, 0);
        }
        else {
            decoded = format!("vcdjZVbvLFB1{}:{}:{}:{}", user.email, user.password, user.MAC, user.count-1);
        }
        hash(decoded) == credentials
    })
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
    fn user() -> Mutex<User> {
        Mutex::new(User::new("ligvigfui@fsda.capok".to_owned(), "password".to_string()))
    }

    #[test]
    fn login_base() {
        let user = user();
        set_mac(&user, "00:00:00:00:00:00".to_string()).unwrap();
        assert!(set_mac(&user, "00:00:00:00:00:00".to_string()).is_ok());
    }

    #[test]
    fn normal_login() {
        let user = user();
        set_mac(&user, "00:00:00:00:00:00".to_string()).unwrap();
        thread::sleep(Duration::from_secs(1));
        assert!(set_mac(&user, "00:00:00:00:00:00".to_string()).is_ok());
        thread::sleep(Duration::from_secs(1));
        assert!(set_mac(&user, "00:00:00:00:00:00".to_string()).is_ok());
        thread::sleep(Duration::from_secs(1));
        assert!(set_mac(&user, "00:00:00:00:00:00".to_string()).is_ok());
    }

    #[test]
    fn login_multiple(){
        let user = user();
        set_mac(&user, "00:00:00:00:00:00".to_string()).unwrap();
        thread::sleep(Duration::from_secs(3));
        assert!(set_mac(&user, "00:00:00:00:00:01".to_string()).is_err());
    }

    #[test]
    fn login_delayed() {
        let user = user();
        set_mac(&user, "00:00:00:00:00:00".to_string()).unwrap();
        thread::sleep(Duration::from_secs(6));
        assert!(set_mac(&user, "00:00:00:00:00:01".to_string()).is_ok());
    }
}
