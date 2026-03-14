pub enum Mode {
    Press = 0,
    Hold = 1,
    Release = 2,
}

impl Mode {
    pub fn value(&self) -> u8 {
        match self {
            Mode::Press => 0,
            Mode::Hold => 1,
            Mode::Release => 2,
        }
    }
}
