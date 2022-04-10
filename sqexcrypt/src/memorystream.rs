pub struct MemoryStream(pub Vec<u8>);

impl MemoryStream {
    pub fn new() -> Self {
        MemoryStream(vec![])
    }

    pub fn merge<T>(&mut self, value: T)
        where T: IntoIterator<Item=u8>
    {
        self.0.append(&mut value.into_iter().collect::<Vec<u8>>())
    }

    pub fn push_u8(&mut self, value: u8) {
        self.0.push(value)
    }

    pub fn push_u16(&mut self, value: u16) {
        self.merge(value.to_le_bytes())
    }
}