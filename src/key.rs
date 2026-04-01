pub enum Key {
    // Power
    PowerOnOff,

    // Navigation
    Ok,
    Up,
    Down,
    Left,
    Right,
    Back,
    Menu,

    // Volume
    VolumeUp,
    VolumeDown,
    Mute,

    // Channel
    ChannelUp,
    ChannelDown,

    // Playback
    Play,
    Pause,
    Stop,
    Forward,
    Rewind,

    // Numbers
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

impl From<Key> for u16 {
    fn from(val: Key) -> Self {
        match val {
            Key::PowerOnOff => 116,
            Key::Ok => 352,
            Key::Up => 103,
            Key::Down => 108,
            Key::Left => 105,
            Key::Right => 106,
            Key::Back => 158,
            Key::Menu => 139,
            Key::VolumeUp => 115,
            Key::VolumeDown => 114,
            Key::Mute => 113,
            Key::ChannelUp => 402,
            Key::ChannelDown => 403,
            Key::Play => 164,
            Key::Pause => 119,
            Key::Stop => 128,
            Key::Forward => 159,
            Key::Rewind => 168,
            Key::N0 => 512,
            Key::N1 => 513,
            Key::N2 => 514,
            Key::N3 => 515,
            Key::N4 => 516,
            Key::N5 => 517,
            Key::N6 => 518,
            Key::N7 => 519,
            Key::N8 => 520,
            Key::N9 => 521,
        }
    }
}
