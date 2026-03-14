pub enum Operation {
    SendKey,
    ReadInfos,
    ChangeChannel,
}

impl Operation {
    pub fn value(&self) -> u8 {
        match self {
            Operation::SendKey => 1,
            Operation::ReadInfos => 10,
            Operation::ChangeChannel => 9,
        }
    }
}
