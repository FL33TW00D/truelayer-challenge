# Truelayer Challenge 

## How to Run

### Running using Docker:

Install Docker for your OS: https://docs.docker.com/get-docker/

Clone repository to your machine.

Whilst inside the directory:

`docker build --tag truelayer --file Dockerfile .` 

`docker run -p 8000:8000 truelayer`

### Running using Rust

Install Rust: https://www.rust-lang.org/tools/Install

Clone the repository to your machine

Whilst inside the directory

`cargo run --release`

### Testing functionality

`curl http://localhost:8000/pokemon/mewtwo`
`curl http://localhost:8000/pokemon/translated/mewtwo`

If tests fail because of the following message:

`Too many open files`

Run the following command to increase the number of allowed files:

`ulimit -n 8192`


## Production

If putting this API into production, I would have done the following:

- Caching on Redis or similar to avoid rate limits from dependent APIs and break dependency chains (therefore maximising uptime).
- Use the tracing crate and ship the spans to something like HoneyComb/Jaeger for full observability.
- CI/CD - Need to scan for vulnerabilities and measure code coverage (with something like cargo-tarpaulin)
- Deploy to a n > 1 nodes to allow failover.
- Use something like cargo-chef to reduce docker build times - improving feedback time and maximising developer productivity.
- Set up Development, UAT and Production instances and a procedure for promotion between each instance.
- Export metrics to a dashboard (e.g Prometheus) with alerts when HTTP 4xx or 5xx hit a certain threshold.
