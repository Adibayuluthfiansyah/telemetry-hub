# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Device registration API: `POST /api/v1/devices` (HTTP 201, 409 on duplicate code).
- Device lookup API: `GET /api/v1/devices/{code}` (HTTP 200, 404).

No releases yet. This section will collect user-visible changes before the
first tag (`v0.1.0`).
