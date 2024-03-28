# ella

Ella is PoC for calling statically compiled Go libraries from Rust.

## Development

```bash
make clean && cargo build && cargo test -- --nocapture
```

## Docker

```bash
docker build -f Dockerfile -t pinkstack/ella -t pinkstack/ella:0.0.1 .
docker run -ti --rm pinkstack/ella
```

## Author
- [Oto Brglez](https://github.com/otobrglez)
