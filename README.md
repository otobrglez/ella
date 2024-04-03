# ella

Ella is a real-time observability tool for your Prometheus-based metrics.

## Development

```bash
make clean && cargo build && cargo test -- --nocapture
```

## Docker

```bash
docker build -f Dockerfile -t pinkstack/ella -t pinkstack/ella:0.0.1 .

docker run -ti --rm \
  -p 8000:8000 \
  pinkstack/ella /usr/local/bin/ella \
  http://0.0.0.0:8000/metrics
```

## Author
- [Oto Brglez](https://github.com/otobrglez)
