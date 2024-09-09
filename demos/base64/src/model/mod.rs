use base64::{engine::general_purpose, Engine};

#[derive(Debug, Clone)]
pub struct Model {
    focus: i32,
    decode: String,
    encode: String,
}

impl Model {
    pub fn default() -> Self {
        Self {
            focus: 0,
            decode: String::from("Normal text"),
            encode: String::from("Base64 text"),
        }
    }
    pub fn decode(&self) -> &String {
        &self.decode
    }
    pub fn encode(&self) -> &String {
        &self.encode
    }
    pub fn set_encode(&mut self, value: String) {
        self.encode = value;
        self.decode = match general_purpose::STANDARD.decode(&self.encode) {
            Ok(decode) => String::from_utf8(decode).unwrap(),
            Err(error) => format!("{}", error),
        };
        self.focus = 2;
    }
    pub fn focus(&self) -> i32 {
        self.focus
    }
    pub fn set_decode(&mut self, value: String) {
        self.decode = value;
        self.encode = general_purpose::STANDARD.encode(&self.decode);
        self.focus = 0;
    }
}
