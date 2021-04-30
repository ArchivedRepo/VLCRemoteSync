pub enum Identity {
    Host = 0,
    Client = 1,
}

impl Into<u8> for Identity {
    fn into(self) -> u8 {
        self as u8
    }
}