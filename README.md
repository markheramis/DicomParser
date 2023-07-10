# Dicom Reader

Dicom Reader is a small [Rust](https://www.rust-lang.org/) utility/library program that uses [dicom_core](https://docs.rs/dicom-core/latest/dicom_core/) and [serde](https://docs.rs/serde/latest/serde/) to read a [dicom](https://en.wikipedia.org/wiki/DICOM) file, parse the dicom properties and return them as JSON.

## Building

run the command below to build

```bash
cargo build
```

the .dll file should be available at `target/debug/dicom_parser.dll`

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