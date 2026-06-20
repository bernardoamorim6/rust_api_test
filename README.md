# Rust API Test (`rust-api-test`)

A production-ready blueprint for a high-performance REST API built with Rust, leveraging **Axum** for routing, **Tokio** as the asynchronous runtime, and **Diesel ORM** with **PostgreSQL**.

The entire development lifecycle is fully reproducible using a **Nix Flake**, with the database managed via **Docker Compose**.

---

## Tech Stack and Architecture

* **Language:** Rust
* **Web Framework:** [Axum](https://github.com/tokio-rs/axum)
* **Async Runtime:** [Tokio](https://tokio.rs/)
* **ORM / Query Builder:** [Diesel](https://diesel.rs/) (with `libpq` PostgreSQL backend)
* **Database:** PostgreSQL (via Docker Compose)
* **Environment:** Nix Flakes + `direnv`

---

## Getting Started (Development Setup)

### 1. Prerequisites
Ensure you have **Nix** (with flakes enabled) and **Docker / Docker Compose** installed on your host system.

### 2. Enter the Environment
If you have `direnv` installed, simply `cd` into the project directory. 

Alternatively, manually spin up the Nix development shell:
```bash
nix develop