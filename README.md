# Rust Microservices Reference Project

This project is a reference implementation of common features used in microservices, built with the [Axum](https://github.com/tokio-rs/axum) framework in Rust. The goal is to provide a reusable and extensible foundation for future projects.

## Features

- [ ] Support SQL DB (PostgreSQL)
    - [x] Migration of the DB schema
    - [ ] Fetch data using pagination
    - [ ] Fetch nested entities
    - [ ] Transactions
    - [ ] Work with JSONB type
- [ ] Translate errors to valid JSON responses
- [x] Write logs in common format
- [ ] Use cache
    - [ ] In-mem
    - [ ] Redis
- [ ] Support OAuth2
- [ ] Support Kafka. Consuming and producing events.
- [ ] Expose service metrics in Prometheus format
- [ ] Show how to fetch configuration from a secret manager
- [ ] Integrate with Sentry
- [ ] Support localization

## Getting Started

### Prerequisites

- Rust (latest stable version)
- PostgreSQL. The default configuration uses to locally running database with name `sample`.
