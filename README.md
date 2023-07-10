# Dicom Reader

Dicom Reader is a small rust library that uses dicom_core and serde to read a dicom file, parse the dicom properties and return them as JSON.

## Building

run the command below to build

```bash
cargo build
```

## Releasing

Run the command below to release the .dll

```bash
cargo build --release
```

the .dll file should be available at `target/release/dicom_parser.dll`

## Testing

Run the command below to run all the tests

```bash
cargo test
```