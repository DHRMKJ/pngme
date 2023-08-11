# My implementation of the PNGme Project - Rust

![PNGme Logo](https://upload.wikimedia.org/wikipedia/commons/d/d5/Rust_programming_language_black_logo.svg)

**PNGme** is a Rust-based command-line utility that allows you to hide text-based messages within PNG image files, utilizing the unused space in the image's metadata. This project demonstrates the power of Rust for system-level programming and showcases how to manipulate image files while maintaining data integrity. With PNGme, you can encode and decode secret messages in PNG images without visibly altering the image itself.

## Table of Contents

1. [Features](#features)
2. [Getting Started](#getting-started)
    - [Installation](#installation)
    - [Usage](#usage)
3. [Usage Examples](#usage-examples)
4. [Contributing](#contributing)
5. [License](#license)

## Features

- **Text Hiding**: Encode any text message into a PNG image without visibly altering the image appearance.
- **Decoding**: Extract hidden messages from PNG images encoded with PNGme.
- **Metadata Utilization**: Utilizes unused space in the PNG image's metadata to store hidden messages.
- **Command-line Interface**: Easy-to-use command-line interface for encoding and decoding messages.
- **Further Development**: If bigger files are zipped they can be encoded into pngs, if you implement it feel free to send a pr. 

## Getting Started

### Installation

Before you start, ensure that you have [Rust](https://www.rust-lang.org/tools/install) installed on your system.

1. Clone the PNGme repository:

    ```bash
    git clone https://github.com/Dh-rm-k/pngme.git
    ```

2. Navigate to the project directory:

    ```bash
    cd pngme
    ```

3. Build the project:

    ```bash
    cargo build --release
    ```

### Usage

PNGme provides two main functionalities: encoding and decoding messages in PNG images.

- To encode a message in an image:

    ```bash
    cargo run encode --path <input-image.png> --message "Your secret message"
    ```

- To decode a message from an image:

    ```bash
    cargo run decode --path <input-image.png>
    ```

## Usage Examples

- Encoding a message:

    ```bash
    cargo run encode --path input.png --message "This is a secret message."
    ```

- Decoding a message:

    ```bash
    cargo run decode --path output.png
    ```

## Contributing

We welcome contributions to PNGme! If you'd like to contribute, please follow these steps:

1. Fork the repository.
2. Create a new branch for your feature or bug fix: `git checkout -b feature/your-feature-name`.
3. Make your changes and commit them with descriptive commit messages.
4. Push your changes to your branch.
5. Submit a pull request to the main repository.

Please ensure that your code follows the project's coding standards and practices. Additionally, write tests for new features or changes you make.

## License

PNGme is released under the [MIT License](LICENSE).

---

Feel free to reach out to us via GitHub Issues or by emailing dhrmkjdv@gmail.com We appreciate your interest and contributions to PNGme!
