/// Remote control keys supported by the decoder.
///
/// Pass a `Key` variant to [`Decoder::press`](crate::Decoder::press),
/// [`Decoder::hold`](crate::Decoder::hold), or
/// [`Decoder::release`](crate::Decoder::release).
///
/// Each variant maps to the Linux input event code sent over the HTTP API.
/// Raw `u16` codes can also be used directly if a key is not listed here.
///
/// # Example
///
/// ```no_run
/// use dtiw385::{Decoders, key::Key};
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let decoder = Decoders::connect([192, 168, 1, 10], 8080);
///
/// // Short press on the OK key
/// decoder.press(Key::Ok).await?;
///
/// // Use a raw code for unlisted keys
/// decoder.press(116u16).await?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Copy)]
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
