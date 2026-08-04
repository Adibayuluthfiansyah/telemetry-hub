# Telemetry Hub

A modern real-time telemetry platform built with Rust for receiving, processing, and visualizing telemetry data from virtual devices and future IoT hardware.

---

## Overview

Telemetry Hub is a learning-oriented yet production-inspired project designed to explore modern backend development with Rust.

Instead of depending on physical hardware, the project starts with a Virtual Device Simulator that continuously generates telemetry data. This allows the backend and dashboard to be fully developed before integrating real IoT devices such as ESP32 or Arduino.

---

## Features

* Real-time telemetry processing
* Virtual device simulator
* REST API
* WebSocket streaming
* Live dashboard
* Modular architecture
* Hardware-independent development

---

## Tech Stack

### Backend

* Rust
* Tokio
* Axum
* Serde

### Frontend

* Next.js
* React
* Tailwind CSS
* Recharts

---

## Project Structure

```text
telemetry-hub/
│
├── apps/
│   ├── server/
│   └── simulator/
│
├── crates/
│
├── frontend/
│
├── docs/
│   └── software-architecture.md
│
├── docker/
│
├── Cargo.toml
└── README.md
```

---

## Development Roadmap

* [ ] Project Setup
* [ ] Cargo Workspace
* [ ] Core Domain
* [ ] Backend API
* [ ] Virtual Device Simulator
* [ ] WebSocket
* [ ] Dashboard
* [ ] Database Support
* [ ] Physical Device Integration

---

## Project Status

🚧 Under Development

---

## License

MIT License

