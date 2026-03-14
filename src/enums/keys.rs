pub trait Key {
    fn get_code(&self) -> u16;
}

pub mod key {

    use super::Key;

    // -------- Power --------

    pub enum Power {
        OnOff,
    }

    impl Key for Power {
        fn get_code(&self) -> u16 {
            match self {
                Power::OnOff => 116,
            }
        }
    }

    // -------- Navigation --------

    pub enum Navigation {
        Ok,
        Up,
        Down,
        Left,
        Right,
        Back,
        Menu,
    }

    impl Key for Navigation {
        fn get_code(&self) -> u16 {
            match self {
                Navigation::Ok => 352,
                Navigation::Up => 103,
                Navigation::Down => 108,
                Navigation::Left => 105,
                Navigation::Right => 106,
                Navigation::Back => 158,
                Navigation::Menu => 139,
            }
        }
    }

    // -------- Volume --------

    pub enum Volume {
        Up,
        Down,
        Mute,
    }

    impl Key for Volume {
        fn get_code(&self) -> u16 {
            match self {
                Volume::Up => 115,
                Volume::Down => 114,
                Volume::Mute => 113,
            }
        }
    }

    // -------- Chaine --------

    pub enum Channel {
        Up,
        Down,
    }

    impl Key for Channel {
        fn get_code(&self) -> u16 {
            match self {
                Channel::Up => 402,
                Channel::Down => 403,
            }
        }
    }

    // -------- Playback --------

    pub enum Playback {
        Play,
        Pause,
        Stop,
        Forward,
        Rewind,
    }

    impl Key for Playback {
        fn get_code(&self) -> u16 {
            match self {
                Playback::Play => 164,
                Playback::Pause => 119,
                Playback::Stop => 128,
                Playback::Forward => 159,
                Playback::Rewind => 168,
            }
        }
    }

    // -------- Numbers --------

    pub enum Number {
        N0,
        N1,
        N2,
        N3,
        N4,
        N5,
        N6,
        N7,
        N8,
        N9,
    }

    impl Key for Number {
        fn get_code(&self) -> u16 {
            match self {
                Number::N0 => 512,
                Number::N1 => 513,
                Number::N2 => 514,
                Number::N3 => 515,
                Number::N4 => 516,
                Number::N5 => 517,
                Number::N6 => 518,
                Number::N7 => 519,
                Number::N8 => 520,
                Number::N9 => 521,
            }
        }
    }
}
