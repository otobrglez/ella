# ella

Ella is a real-time observability tool for your Prometheus-based metrics.

## Usage

```
Usage: ella [OPTIONS] <Metric URLs>...

Arguments:
  <Metric URLs>...

Options:
      --collect-interval <COLLECT_INTERVAL>  [env: COLLECT_INTERVAL=] [default: 2s]
      --retention-period <RETENTION_PERIOD>  [env: RETENTION_PERIOD=] [default: "1 hour"]
      --dump-period <DUMP_PERIOD>            [env: DUMP_PERIOD=] [default: "1 minute"]
  -h, --help                                 Print help
  -V, --version                              Print version
```

## Usage via Docker Image 🚀

This repository has pre-built [Docker images](https://github.com/otobrglez/ella/pkgs/container/ella). 
Recent `master` build can be obtained via the `ghcr.io` container registry.

```bash
docker run ghcr.io/otobrglez/ella:master \
  /usr/local/bin/ella http://0.0.0.0:8000/metrics
```

## Development 🏗

```bash
make clean && cargo build && cargo test -- --nocapture
```

### Building Docker Image

```bash
docker build -f Dockerfile -t pinkstack/ella -t pinkstack/ella:0.0.1 .

docker run -ti --rm \
  -p 8000:8000 \
  pinkstack/ella /usr/local/bin/ella \
  http://0.0.0.0:8000/metrics
```

## Author
- [Oto Brglez](https://github.com/otobrglez)
