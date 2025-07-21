use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub keyboard: Keyboard,
    pub keysound: Keysound,
}

#[derive(Debug, Deserialize)]
pub struct Keyboard {
    pub path: String,
}

#[derive(Debug, Deserialize)]
pub struct Keysound {
    pub path: String,
}
