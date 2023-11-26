use std::time;

impl IsHex for String {
    /// Checks if a string is hex
    /// # Returns 
    /// true if the string is hex
    /// 
    /// false if the string is not hex
    fn is_hex(&self) -> bool {
        for c in self.chars() {
            if !c.is_digit(16) {
                return false;
            }
        }
        true
    }
}

pub trait IsHex {
    fn is_hex(&self) -> bool;
    fn is_not_hex(&self) -> bool {
        !self.is_hex()
    }
}

/// Returns the current time in seconds since the Unix epoch.
pub fn now() -> u64 {
    time::SystemTime::now().duration_since(time::UNIX_EPOCH).expect("Time went backwards").as_secs()
}


/// Returns a string with the current time in the format "YYYY-MM-DD HH:MM:SS"
/// with a two hour offset.
pub fn readable_time() -> String {
    let current_date_time = chrono::NaiveDateTime::from_timestamp_opt(now() as i64, 0)
        .expect("Invalid timestamp");
    //get the current timezone offset
    let offset = chrono::Local::now().offset().local_minus_utc() as i64;
    let local_date_time = current_date_time + chrono::Duration::seconds(offset);
    local_date_time.format("%Y-%m-%d %H:%M:%S").to_string()
}

#[cfg(test)]
mod time_tests {
    use super::*;

    #[test]
    fn time_test() {
        let time = now();
        assert!(time > 0);
    }
}