use super::*;


enum CustomResult {
    Ok,
    Wc,
}

pub fn login(mut users: Arc<Vec<Mutex<User>>>, credentials: (String, String, String)) -> CustomResult {
    //credentials: (email, password, MAC)
    let (email, password, session_id) = credentials;
    for user in &mut users.iter() {
        let mut user = user.lock().unwrap();
        if user.email == email && user.password == password {
            return user.login(session_id);
        } else {
            return CustomResult::Wc
        }
    }
    CustomResult::Ok
}