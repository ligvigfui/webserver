
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