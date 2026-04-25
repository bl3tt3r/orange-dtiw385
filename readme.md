# dtiw385

[![License](https://img.shields.io/badge/LICENSE-MIT-blue.svg)](./LICENSE)
![Rust Edition](https://img.shields.io/badge/edition-2024-orange)
![Made with Rust](https://img.shields.io/badge/Made%20with-Rust-000000?logo=rust)
![CI](https://github.com/bl3tt3r/dtiw385/actions/workflows/rust.yml/badge.svg)

This crate provides a Rust API to interact with [🟠 Orange](https://www.orange.fr/) DTIW385 decoders, allowing device discovery, querying device information, and sending asynchronous remote control commands over the network.

__This project is not affiliated with, endorsed by, or sponsored by Orange.__

---

## 📦 Installation

Add this to your `Cargo.toml`:

### 📦 From crates.io (recommended)

```toml
[dependencies]
dtiw385 = "0.1.2"
```

__Or run the following Cargo command in your project directory: `cargo add dtiw385`__

### 🔹 From Git (latest development version)

```toml
[dependencies]
dtiw385 = { git = "https://github.com/bl3tt3r/dtiw385", branch = "master" }
```

### 🔖 From a tagged release

```toml
[dependencies]
dtiw385 = { git = "https://github.com/bl3tt3r/dtiw385", tag = "v0.1.2" }
```

---

## 🧩 Features

| Feature        | Description                                           |
| -------------- | ----------------------------------------------------- |
| `serializable` | Enable serialization of `Decoder` and `ApiInfosData`. |

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

// Receiver of discovered decoders
let mut rx = Decoders::search(
    [192, 168, 1, 1]..=[192, 168, 1, 254],
    8080..=8080,
)
.find();
```

Each discovered decoder is sent through the receiver.

---

### 🎮 Interact with a decoder

#### 🎹 Keys

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
```

---

## ⚙️ Configuration

### 🔁 Concurrency

```rust
Decoders::search(ip_range, port_range)
    .with_concurrency(50);
```

* **Higher value** = faster scan ⚡
* **But more CPU and network usage 🔥**

---

### ⏱️ Timeout

```rust
Decoders::search(ip_range, port_range)
    .with_timeout(500);
```

* **Timeout** is in milliseconds
* **Lower** = faster failure
* **Higher** = more reliable but slower

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

License: MIT

---

<p align="center">
  Made with ❤️ and Rust 🦀
</p>