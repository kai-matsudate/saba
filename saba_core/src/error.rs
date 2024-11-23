use alloc::string::String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    Network(String),
    Unexpectedinput(String),
    InvalidUI(String),
    Other(String),
}
