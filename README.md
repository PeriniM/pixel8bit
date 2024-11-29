# 👾 pixel8bit

[![Crates.io](https://img.shields.io/crates/v/pixel8bit)](https://crates.io/crates/pixel8bit)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://img.shields.io/github/actions/workflow/status/PeriniM/pixel8bit/ci.yml?branch=main)](https://github.com/PeriniM/pixel8bit/actions)
[![Crates.io Downloads](https://img.shields.io/crates/d/pixel8bit)](https://crates.io/crates/pixel8bit)

**pixel8bit** is a Rust library and command-line tool that applies 8-bit pixelation effects to images, with optional symmetry detection and mirroring.

## ✨ Features

- 8-bit Pixelation: Transform images to emulate classic 8-bit graphics.
- Symmetry Detection: Automatically detect horizontal and vertical symmetry axes.
- Mirroring: Mirror images around detected symmetry axes for artistic effects.

## 📦 Installation

To install **pixel8bit**, ensure you have [Rust and Cargo](https://www.rust-lang.org/tools/install) installed. Then, run:

```
cargo install pixel8bit
```

## 🚀 Usage

After installation, you can use the pixel8bit command-line tool as follows:

```
pixel8bit <input_path> <output_path> <block_size> <is_mirrored>
```

- <input_path>: Path to the input image file.
- <output_path>: Path to save the processed image.
- <block_size>: Size of the pixel blocks (e.g., 16).
- <is_mirrored>: true or false to enable or disable mirroring.

Example:

To pixelate an image with a block size of 16 pixels and enable mirroring:

```
pixel8bit input.jpg output.jpg 16 true
```

## 🛠️ Library Integration

To use **pixel8bit** as a library in your Rust project, add the following to your Cargo.toml:

[dependencies]
pixel8bit = "0.1.0"

Then, in your code:

```rust
use pixel8bit::pixelate::apply_pixelation;
use pixel8bit::symmetry::{detect_symmetry, mirror_image};
```

## 🖼️ Pixelation Examples


| Block Size | Result |
|------------|---------|
| Original | ![Original Image](docs/assets/original.png) |
| 8x8 | ![Pixelated Image (8x8)](docs/assets/output_8.png) |
| 16x16 | ![Pixelated Image (16x16)](docs/assets/output_16.png) |
| 32x32 | ![Pixelated Image (32x32)](docs/assets/output_32.png) |
| 64x64 | ![Pixelated Image (64x64)](docs/assets/output_64.png) |


## 🤝 Contributing

Contributions are welcome! Please fork the repository and submit a pull request.

## 📄 License

This project is licensed under the MIT License.
