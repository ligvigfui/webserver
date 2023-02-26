use std::{time::{self}, sync::{Arc, Mutex}};
use crate::lib::IsHex;
use ripemd::{Ripemd160, Digest};
use webserver::now;

use crate::lib::{CustomResult, User};

fn decode(users: &Arc<Vec<Mutex<User>>>, credentials: String) -> (CustomResult, String) {
    //check if string is hex
    if credentials.is_not_hex() {
        return (CustomResult::Wc, "nothing".to_string());
    }
    
    //return resutlt and email
    //if result is ok, email is the email
    //if result is we, email is "nothing".as_string()
    // code format from client to server: 1LzI; + pw[5..] + Q695' + email.split('@')[0] Ufy6R8 + sec + pw[0..5] + zD*G;vx& + email.split('@')[1] amp;jVP
    
    
    let time = now();
    for i in 0..10 {
        let seconds = (time - i).to_string();
        for user in users.iter() {
            let user = user.lock().unwrap();
            let mut decoded = String::from("1LzI;");
            decoded.push_str(&user.password[5..]);
            decoded.push_str("Q695'");
            decoded.push_str(&user.email.split('@').collect::<Vec<&str>>()[0]);
            decoded.push_str("Ufy6R8");
            decoded.push_str(&seconds);
            decoded.push_str(&user.password[0..5]);
            decoded.push_str("zD*G;vx&");
            decoded.push_str(&user.email.split('@').collect::<Vec<&str>>()[1]);
            decoded.push_str("amp;jVP");
            if hash(decoded) == credentials {
                return (CustomResult::Ok, user.email.clone());
            }
        }
    }
    (CustomResult::Wc, String::from("nothing"))
}
//


fn encode(result: CustomResult, email: &str) -> String {
    // code format from server to client: r + customresult + ^vS + sec[0..4] + 5By| + sec[4..-4] + hlt + sec[-4..] + U9'8@ + email
    let mut encoded = String::from("r");
    encoded.push_str(&result.to_string());
    encoded.push_str("^vS");
    let seconds = now();
    encoded.push_str(&seconds.to_string()[0..4]);
    encoded.push_str("5By|");
    encoded.push_str(&seconds.to_string()[4..seconds.to_string().len() - 4]);
    encoded.push_str("hlt");
    encoded.push_str(&seconds.to_string()[seconds.to_string().len() - 4..]);
    encoded.push_str("U9'8@");
    encoded.push_str(email);
    if result == CustomResult::Rl {println!("encoded: {}", encoded); }
    hash(encoded)
}

fn hash(string: String) -> String {
    let mut hasher = Ripemd160::new();
    hasher.update(string.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
}

pub fn get_response(messege: &str, users: Arc<Vec<Mutex<User>>>) -> (String, String) {
    //returns (status, response)

    //exctract credentials from messege
    //decode credentials
    //if result::ok
    //  extract session_id_hash from messege
    //  if session_id_hash is 40 hex characters
    //      session_id = session_id_short
    //      if Some(session_id)
    //          update / insert session_id into user with email
    //      if None
    //          return 200 Ok "need long session id"
    //  if session_id_hash is 480 hex characters
    //      session_id = session_id_long
    //      if Some(session_id)
    //         insert session_id into user with email
    //      if None
    //          return 400 Bad Request
    //  else return 400 Bad Request
    //else return 200 Ok "wrong credentials"

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
    
    //get hash of time, email, password and MAC address from user
    //returns result and user 
    let (result, email) = decode(&users, credentials.to_owned());
    
    //if result is ok
    if result == CustomResult::Ok {
        //separate session_id_hash
        let session_id_hash_start = match messege.find("SessionId: ") {
            Some(index) => index,
            None => {
                // the substring was not found
                println!("SessionIdHash not found in the string");
                return ("400 Bad Request".to_string() , "SessionId not found in the string".to_string());
            }
        };
        // extract the part of the string after "SessionIdHash:"
        let session_id_hash = &messege[session_id_hash_start + 11..];
        // find the end of the line
        let line_end = match session_id_hash.find('\r') {
            Some(index) => index,
            None => session_id_hash.len(),
        };
        // extract the session_id_hash
        let session_id_hash = &session_id_hash[..line_end];
        // make it a string
        let session_id_hash = session_id_hash.to_string();
        // throw error if session_id_hash is not hex
        if session_id_hash.is_not_hex() {
            println!("SessionId is not hex");
            return ("400 Bad Request".to_string(), "SessionId is not hex".to_string());
        }
        
        //if session_id_hash is 40 hex characters
        if session_id_hash.len() == 40 {

            //session_id = session_id_short
            let session_id = send_session_id_short(&users, &email, &session_id_hash);
            //if Some(session_id)
            match session_id {
                Some(session_id) => {
                    println!("found short session id: \"{}\"", session_id);
                    //update / insert session_id into user with email
                    let mut user = users.iter().find(|user| user.lock().unwrap().email == email).unwrap().lock().unwrap();
                    let result = user.login(session_id);
                    match result {
                        CustomResult::Ok => {
                            println!("{}: {}", email, CustomResult::Ok);
                            return ("200 Ok".to_string(), encode(CustomResult::Ok, &email));
                        },
                        CustomResult::Mu => {
                            println!("{}: {}", email, CustomResult::Mu);
                            return ("200 Ok".to_string(), encode(CustomResult::Mu, &email));
                        },
                        _ => {
                            println!("{}: {}", email, CustomResult::Br);
                            return ("400 Bad Request".to_string(), encode(CustomResult::Br, &email));
                        }
                    }
                },
                None => {
                    println!("short session id not found");
                    //return 200 Ok "need long session id"
                    println!("{}: {}", email, CustomResult::Rl);
                    return ("200 Ok".to_string(), encode(CustomResult::Rl, &email));
                }
            }
        }
        //if session_id_hash is 480 hex characters
        else if session_id_hash.len() == 480 {
            //session_id = session_id_long
            let session_id = send_session_id_long(session_id_hash);
            //if Some(session_id)
            match session_id {
                Some(session_id) => {
                    println!("found long session id: \"{}\"", session_id);
                    //insert session_id into user with email
                    let mut user = users.iter().find(|user| user.lock().unwrap().email == email).unwrap().lock().unwrap();
                    match user.login(session_id) {
                        CustomResult::Ok => {
                            println!("{}: {}", email, CustomResult::Ok);
                            return ("200 Ok".to_string(), encode(CustomResult::Ok, &email));
                        },
                        CustomResult::Mu => {
                            println!("{}: {}", email, CustomResult::Mu);
                            return ("200 Ok".to_string(), encode(CustomResult::Mu, &email));
                        },
                        _ => {
                            println!("{}: {}", email, CustomResult::Br);
                            return ("400 Bad Request".to_string(), encode(CustomResult::Br, &email));
                        }
                    }
                },
                None => {
                    println!("long session id not found");
                    //return 400 Bad Request
                    println!("{}: {}", email, CustomResult::Br);
                    return ("400 Bad Request".to_string(), encode(CustomResult::Br, "nothing"));
                }
            }
        }
        else {
            //return 400 Bad Request
            println!("{}: {}", email, CustomResult::Br);
            return ("400 Bad Request".to_string(), encode(CustomResult::Br, "nothing"));
        }


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

fn send_session_id_long(session_id: String) -> Option<String> {
    //check if string is hex
    if session_id.is_not_hex() {
        return None;
    }
    let mut MAC_address = String::new();
    let time = now();
    //loop throw time
    for i in 0..10 {
        let secs = time - i;
        //loop throw MAC address chars
        for mac_address_char in 0..12 {

            if (MAC_address.len()+1) % 3 == 0 {
                MAC_address.push_str(":");
            }

            let hash_part = session_id[0+mac_address_char*40..40+mac_address_char*40].to_string();
            let (before, after) = session_id_long_helper(secs, mac_address_char);
            let mut missed = 0;
            //loop throw possible chars
            for char in 0..16 {
                let char1 = i32_to_hex_char(char);
                let str1 = before.clone() + &char1 + &after;
                let hash = hash(str1);
                if hash == hash_part {
                    MAC_address.push_str(&char1);
                    break;
                }
                missed += 1;
            }
            if missed == 16 {
                continue;
            }            
            if MAC_address.len() == 17 {
                return Some(MAC_address);
            }
        }
    }
    None
}

fn send_session_id_short(users: &Arc<Vec<Mutex<User>>>, email: &String, session_id: &String) -> Option<String> {
    //check if string is hex
    if session_id.is_not_hex() {
        return None;
    }
    //get user from users and email
    let user = match users.iter().find(|user| user.lock().unwrap().email == *email) {
        Some(user) => user,
        None => return None,
    };
    let sessions = user.lock().unwrap().sessions();
    //loop throw time
    let time = now();
    for i in 0..10 {
        //loop throw sessions
        for session in &sessions {
            // u@CV + MAC + @|yvSL + time + IB7wCVM
            let string2 = "u@CV".to_string() + &session.id + "@|yvSL" + &(time-i).to_string() + "IB7wCVM";
            if &hash(string2) == session_id {
                return Some(session.id.clone());
            }
        }
    }
    None
}

fn session_id_long_helper(time: i32, chars_number: usize) -> (String, String) {
    //pick wich char_number we look for
    match chars_number {
        0 => {
            //sk + Time() + djY/gt;&amp;"uYWf + mac_address[0] + R
            let before = String::from("sk") + &time.to_string() + "djY/gt;&amp;\"uYWf";
            let after = String::from("R");
            (before, after)
        },
        1 => {
            //^m4U + mac_address + fn.v0LEyE + Time() + IA
            let before = String::from("^m4U");
            let after = String::from("fn.v0LEyE") + &time.to_string() + "IA";
            (before, after)
        },
        2 => {
            //tK^8T82 + Time() + VIN. + mac_address + xlQ|
            let before = String::from("tK^8T82") + &time.to_string() + "VIN.";
            let after = String::from("xlQ|");
            (before, after)
        },
        3 => {
            //aI& + mac_address + ampg^I/'I4c + Time() + eUC
            let before = String::from("aI&");
            let after = String::from("ampg^I/\'I4c") + &time.to_string() + "eUC";
            (before, after)
        },
        4 => {
            //8"9r.w + Time() + kRVp + mac_address + Cj-Z@
            let before = String::from("8\"9r.w") + &time.to_string() + "kRVp";
            let after = String::from("Cj-Z@");
            (before, after)
        },
        5 => {
            //Q2-C + mac_address + G7t&K3qO?;" + Time()
            let before = String::from("Q2-C");
            let after = String::from("G7t&K3qO?;\"") + &time.to_string();
            (before, after)
        },
        6 => {
            //x?RT + Time() + NPQI" + mac_address + d5Z9vJ
            let before = String::from("x?RT") + &time.to_string() + "NPQI\"";
            let after = String::from("d5Z9vJ");
            (before, after)
        },
        7 => {
            // " + Time() + Aodlq;URW + mac_address + 0Cb
            let before = String::from("\"") + &time.to_string() + "Aodlq;URW";
            let after = String::from("0Cb");
            (before, after)
        },
        8 => {
            //g&gt + mac_address + ;/X + Time() + OVb2*xl
            let before = String::from("g&gt");
            let after = String::from(";/X") + &time.to_string() + "OVb2*xl";
            (before, after)
        },
        9 => {
            //SAgV_RFZM + Time()  + mac_address + 4/Y4
            let before = String::from("SAgV_RFZM") + &time.to_string();
            let after = String::from("4/Y4");
            (before, after)
        },
        10 => {
            //;HvDz + Time() + oRGiN + mac_address + f.;|N
            let before = String::from(";HvDz") + &time.to_string() + "oRGiN";
            let after = String::from("f.;|N");
            (before, after)
        },
        11 => {
            //# + Time() + Xq + mac_address + dadfm&amp;uU
            let before = String::from("#") + &time.to_string() + "Xq";
            let after = String::from("dadfm&amp;uU");
            (before, after)
        },
        _ => panic!("chars_number is not a valid number"),
    }
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
    use std::{thread::sleep};
    use super::*;

    #[test]
    fn encode_test(){
        let users = test_users();
        let string = encode(CustomResult::Ok, "ligvigfui@gmail.com");
        println!("{}", string);
        assert_eq!(string, "rOk^vS5By|hltU9'8@ligvigfui");
    }


    #[test]
    fn digest_test(){
        let input = b"hello world";
        let mut hasher = Ripemd160::new();
        hasher.update(input);
        let result = hasher.finalize();
        assert_eq!(format!("{:x}", result), "98c615784ccb5fe5936fbc0cbe9dfdb408d92f0f");
    }

    #[test]
    fn decode_test(){
        let users = test_users();
        let (result, string) = decode(&Arc::clone(&users), "a".to_owned());
        assert_eq!(result, CustomResult::Wc);
        assert_eq!(string, "nothing");
        let (result, string) = decode(&users, "c1e5947a3dd4a4fb79598439a8e996bded3b6820".to_owned());
        assert_eq!(result, CustomResult::Ok);
        assert_eq!(string, "ligvigfui@gmail.com");
    }

    #[test]
    fn try_login(){
        let users = test_users();
        assert_eq!(users[0].lock().unwrap().login("hali0123".to_owned()), CustomResult::Ok);
        assert_eq!(users[0].lock().unwrap().login("hali".to_owned()), CustomResult::Mu);
        assert_eq!(users[0].lock().unwrap().login("hali0123".to_owned()), CustomResult::Ok);
        sleep(time::Duration::from_secs(5));
        assert_eq!(users[0].lock().unwrap().login("hali".to_owned()), CustomResult::Ok);
        assert_eq!(users[0].lock().unwrap().login("hali0123".to_owned()), CustomResult::Mu);
    }
    #[test]
    fn test_session_id_long(){
        assert_eq!(Some("00:15:5d:e1:47:ac".to_owned()), send_session_id_long("ee231cc46cd2bdab147fa04c37d0a25be4d7550fc062e0b325ce9d13a7623ff81ef298b2f19839a76e8381c3afed4eb35af382a1eae5b6e1b96f6c29284561c3e77abe6e99a61846a08d97a40bfacf3a903a5718d444f60332cd2b8a99b87b4c16d51fdbc0602f9363f37ade930be2f16aa5cae8681f3f477adc08615a13a175ff3cfbe0d7f24c2fbe7d1866b8a5a51e4c167f6e07f3f689b590ade9c902c45ff3c1ed59a1e4b7e7ae613b2f860a904679d04e755d52f6ff80606570ce3987ea2fc740642b80b393b6451821490a74f3c5cba534a6c7bc1edf2863fe49dc25cdaaa6bc6f77daa3d22e72cb5c5706d1af".to_string()))
    }
    #[test]
    fn test_session_id_short(){
        let users = test_users();
        send_session_id_short(&users, &"ligvigfui@gmail.com".to_owned(), &"91640fbeb4db4045062bc39ee1e2a3f986f8034f".to_owned());
    }
}