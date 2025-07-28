use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub keyboard: Option<Keyboard>,
    pub keysound: Option<Keysound>,
}

#[derive(Debug, Deserialize)]
pub struct Keyboard {
    pub path: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Keysound {
    pub path: Option<String>,
}
