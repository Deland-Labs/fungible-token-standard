pub trait StableState: Sized {
    fn encode(&self) -> Vec<u8>;
    fn decode(bytes: Vec<u8>) -> Result<Self, String>;
}
