# lagos_logger

A simple, pretty, and lightweight logger for Rust.

## Installation

To add this to your Cargo project with the default timestamp support, run:

```bash
cargo add lagos_logger
```

### Optional Timestamps (Features)

By default, `lagos_logger` includes timestamps in its output. If you want to disable timestamps (which removes the `chrono` dependency for faster compilation and a smaller binary), you can disable default features:

```bash
cargo add lagos_logger --no-default-features
```

## Usage

Import the `logger!` macro and use one of the built-in log types: `Info`, `Warn`, `Error`, `Running`, or `Success`. The log type is optional and defaults to `Info` if omitted. It works just like `println!` for variables and formatting.

```rust
use lagos_logger::{logger, Level::*};

fn main() {
    let port = 8080;

    // By default, omitting the log level defaults to Info:
    logger!("Loading configuration files...");
    logger!("Connected to database at: {}", "localhost");

    // Or explicitly use other log types!
    logger!(Running, "Starting server on port {}", port);
    logger!(Warn, "Missing optional config, using defaults");
    logger!(Success, "Server is fully operational!");

    // Variables work seamlessly
    let error_code = 500;
    logger!(Error, "Failed to connect: Code {}", error_code);
}
```

## Crate

https://crates.io/crates/lagos_logger
