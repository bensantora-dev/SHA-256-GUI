# SHA-256-GUI Testing Guide

This guide explains how to test the SHA-256-GUI application.

## Prerequisites

- Rust and Cargo installed
- A terminal/command line

## Quick Test

### Step 1: Build and Run

```bash
cd /home/ben/SHA-256-GUI
cargo run
```

The GUI window will open.

### Step 2: Test with the Included Test File

A test file has been created with a known hash for verification:

| Field | Value |
|-------|-------|
| **File Path** | `/home/ben/SHA-256-GUI/test_file.txt` |
| **Expected Hash** | `8c70acdc0c8cc7bb5387c3c1de7699275ede70438b4a425dafa4fdd54c608134` |

### Step 3: Verify in the GUI

1. **File path field**: Enter `/home/ben/SHA-256-GUI/test_file.txt`
2. **Expected Hash field**: Paste `8c70acdc0c8cc7bb5387c3c1de7699275ede70438b4a425dafa4fdd54c608134`
3. Click **Compute**
4. **Expected Result**: ✅ MATCH should appear

## Verify Hash Independently

To confirm the expected hash is correct, run:

```bash
sha256sum /home/ben/SHA-256-GUI/test_file.txt
```

Output should be:
```
8c70acdc0c8cc7bb5387c3c1de7699275ede70438b4a425dafa4fdd54c608134  test_file.txt
```

## Test with Your Own Files

1. Choose any file on your system
2. Get the expected hash: `sha256sum /path/to/your/file`
3. Enter the file path in the GUI
4. Paste the hash in the "Expected Hash" field
5. Click **Compute** and verify ✅ MATCH appears

## Troubleshooting

| Issue | Solution |
|-------|----------|
| "Error: No such file" | Check the file path is correct and absolute |
| ❌ MISMATCH | Verify you copied the full hash without extra spaces |
| Window doesn't appear | Ensure you have a display environment (X11/Wayland) |
