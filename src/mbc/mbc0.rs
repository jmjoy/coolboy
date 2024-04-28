use super::MBC;

pub struct MBC0 {
    data: Vec<u8>,
}

impl MBC0 {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }
}

impl MBC for MBC0 {
    fn read(&self, addr: u16) -> u8 {
        self.data[addr as usize]
    }

    fn write(&mut self, _addr: u16, _value: u8) {}
}
