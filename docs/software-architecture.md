-# Software Architecture

## Project

**Telemetry Hub**

A modular real-time telemetry platform built with Rust for collecting, processing, and visualizing telemetry data from virtual devices and future physical IoT devices.

---

# Vision

Build a modern telemetry platform that is modular, scalable, and hardware-independent during development.

---

# Scope

## Included (v1)

* Virtual Device Simulator
* Rust Backend
* REST API
* WebSocket
* Live Dashboard
* In-Memory State Management

## Excluded (v1)

* Authentication & Authorization
* Database Persistence
* MQTT
* Physical IoT Devices
* Kubernetes
* Cloud Deployment

---

# Architecture

```text
Virtual Device Simulator
            │
            ▼
     Rust Backend (Axum)
            │
     ┌──────┴──────┐
     │             │
 REST API     WebSocket
     │             │
     └──────┬──────┘
            ▼
     Next.js Dashboard
```

---

# Core Domains

## Device

Represents a telemetry source.

| Field  | Description       |
| ------ | ----------------- |
| id     | Unique identifier |
| name   | Device name       |
| type   | Device type       |
| status | Online/Offline    |

---

## Telemetry

Represents sensor data.

| Field       |
| ----------- |
| device_id   |
| temperature |
| humidity    |
| voltage     |
| current     |
| rpm         |
| timestamp   |

---

## Alert

Represents warning or critical events.

| Field     |
| --------- |
| device_id |
| severity  |
| message   |

---

# Technology Stack

## Backend

* Rust
* Tokio
* Axum
* Serde

## Frontend

* Next.js
* React
* Tailwind CSS
* Recharts

## Development

* Cargo Workspace
* Docker
* Git
* GitHub Actions

---

# Development Roadmap

## Phase 1

Project Setup

## Phase 2

Core Domain

## Phase 3

Backend API

## Phase 4

Virtual Device Simulator

## Phase 5

WebSocket

## Phase 6

Dashboard

## Phase 7

Database Integration (Optional)

## Phase 8

Physical Device Integration (Future)

---

# Design Principles

* Modular Architecture
* Separation of Concerns
* Clean Architecture
* Domain-First Design
* Extensibility
* Testability
* Hardware Independence
-
