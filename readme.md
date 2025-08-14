<div align="center">

# VC:MP Announce For Rust

</div>

## Build
```bash
cargo build --release
```

## Usage
### Generate default config and run
```bash
cargo run --release -- --default-config config.toml
```

### Run with config
```bash
cargo run --release -- --config config.toml
```

## Run In VCMP Server
### Build
```bash
cargo build --release
```

### Copy to server & Edit your `server.cfg`
```cfg
plugins vcmp-announce
```


