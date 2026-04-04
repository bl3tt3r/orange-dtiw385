# dtiw385

[![License](https://img.shields.io/badge/LICENCE-MIT-blue.svg)](./LICENSE)
![Rust Edition](https://img.shields.io/badge/edition-2024-orange)
![Made with Rust](https://img.shields.io/badge/Made%20with-Rust-000000?logo=rust)

Async Rust client to discover and control DTIW385 decoders over the network.

---

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
dtiw385 = "0.1"
````

---

## 🚀 Usage

The library provides two main ways to work with decoders.

### 🔌 Connect to a known decoder

If you already know the IP and port, you can create a decoder directly:

```rust
use dtiw385::Decoder;

let decoder = Decoder::new([192, 168, 1, 10], 8080);
```

You can then send commands to the device (button actions like press, hold, release).

---

### 🔍 Discover decoders on the network

Scan a range of IPs and ports to find available devices:

```rust
use dtiw385::Decoders;

let mut rx = Decoders::search(
    [192, 168, 1, 1]..=[192, 168, 1, 254],
    8080..=8080,
)
.find();
```

Each discovered decoder is sent through the receiver.

---

### 🎮 Interact with a decoder

#### 🎹 Keys

The library provides a `Key` enum with most common remote control buttons already mapped.

Each key is internally converted to a Linux input event code (`u16`) when sent to the decoder.

You can use these keys directly with:

- `press`
- `hold`
- `release`

---

##### 📋 Available keys

| Key           | Description        |
| ------------- | ------------------ |
| `PowerOnOff`  | Power toggle       |
| `Ok`          | Validate selection |
| `Up`          | Navigate up        |
| `Down`        | Navigate down      |
| `Left`        | Navigate left      |
| `Right`       | Navigate right     |
| `Back`        | Go back            |
| `Menu`        | Open menu          |
| `VolumeUp`    | Increase volume    |
| `VolumeDown`  | Decrease volume    |
| `Mute`        | Mute sound         |
| `ChannelUp`   | Next channel       |
| `ChannelDown` | Previous channel   |
| `Play`        | Play               |
| `Pause`       | Pause              |
| `Stop`        | Stop               |
| `Forward`     | Fast forward       |
| `Rewind`      | Rewind             |
| `N0`          | Number 0           |
| `N1`          | Number 1           |
| `N2`          | Number 2           |
| `N3`          | Number 3           |
| `N4`          | Number 4           |
| `N5`          | Number 5           |
| `N6`          | Number 6           |
| `N7`          | Number 7           |
| `N8`          | Number 8           |
| `N9`          | Number 9           |

#### 🧪 Example

```rust
use dtiw385::{Decoder, Key};

let decoder = Decoder::new([192, 168, 1, 10], 8080);

decoder.press(Key::Ok).await?;
decoder.hold(Key::VolumeUp).await?;
decoder.release(Key::VolumeUp).await?;

---

## ⚙️ Configuration

### 🔁 Concurrency

```rust
Decoders::search(ip_range, port_range)
    .with_concurrency(50);
```

* Higher value = faster scan ⚡
* But more CPU and network usage 🔥

---

### ⏱️ Timeout

```rust
Decoders::search(ip_range, port_range)
    .with_timeout(500);
```

* Timeout is in milliseconds
* Lower = faster failure
* Higher = more reliable but slower

---

## 📚 Examples

Examples are available in the `examples/` folder.

- [find_available_decoders](examples/find_available_decoders.rs)
- [get_decoder_infos](examples/get_decoder_infos.rs)
- [switch_decoder_power](examples/switch_decoder_power.rs)

Run one with:

```bash
cargo run --example get_decoder_infos
```

---

License : MIT

---

<p align="center">
  Made with ❤️ and Rust 🦀
</p>