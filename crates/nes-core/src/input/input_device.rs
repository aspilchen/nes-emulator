pub trait InputDevice {
    fn read(&mut self) -> u8;
    fn write(&mut self, value: u8);
    fn strobe(&mut self);
}
