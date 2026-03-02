# SHA-256-GUI Tool

A simple GUI application written in Rust using Slint to compute and verify SHA-256 checksums of files.

## Features
- **Compute Hash**: Calculate SHA-256 hash for any local file.
- **Verify Hash**: Compare the computed hash against an expected hash with a visual Match/Mismatch indicator.
- **Dark Mode**: Clean, dark-themed user interface.

## Prerequisites
- Rust and Cargo installed.

## How to Run
1. Clone the repository.
2. Run the application:
   ```bash
   cargo run
   ```
## Build from Source
```bash
git clone https://github.com/bensantora/SHA-256-GUI
cd SHA-256-GUI
cargo run
```
## Usage
1. Enter the full path to the file in the "File path" field.
2. (Optional) Paste the official hash in the "Expected Hash" field for verification.
3. Click **Compute**.
4. The result and verification status will be displayed.

## License
MIT License

##Author
- Ben Santora
