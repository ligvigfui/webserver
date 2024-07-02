pub static DEBUG: DebugLevel = DebugLevel::HIGH;
pub static DEBUG_LEN: usize = 200;

#[derive(PartialEq, PartialOrd)]
pub enum DebugLevel {
    LOW,
    MEDIUM,
    HIGH,
    ULTRA,
}