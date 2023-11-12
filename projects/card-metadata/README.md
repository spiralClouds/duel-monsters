# This project is a rust library
You don't have to run this but this is how a rust lib is created
```bash
cargo init --lib
```

# basic building and running the project
build
```bash
cargo build
```
run
```bash
cargo run
```
check for errors without producing an executable
```bash
cargo check
```

# build without warnings
`RUSTFLAGS=-Awarnings cargo build --timings`

# test algorithms with 
`cargo test`

# test algorithms without flags
`RUSTFLAGS=-Awarnings cargo test`

# How was this made in the firstplace?
Used YGOpro api guide: https://ygoprodeck.com/api-guide/
```bash
https://db.ygoprodeck.com/api/v7/cardinfo.php
```

# To build structs based on that json
https://transform.tools/json-to-rust-serde
paste in the json result  from ygopro