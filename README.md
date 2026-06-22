# Home Digital Twin

Home Digital Twin is a Rust-powered 3D smart home model by Vineeth Velmurugan under **Vineeth Makes**. It represents rooms, devices, sensor readings, and automation checks as a browser-rendered digital twin.

The goal is to make home automation visible: a spatial view where lights, climate zones, cameras, locks, motion sensors, and routines can be inspected as part of one live system.

## What It Does

- Models a home layout with rooms, bounds, floors, and device positions
- Tracks device type, status, room assignment, and energy use
- Simulates room telemetry such as temperature, humidity, air quality, and occupancy
- Evaluates comfort, security, and energy automation events
- Exposes a JSON/WASM interface for a browser demo
- Renders an interactive Three.js floorplan with live summary metrics

## Stack

- Rust for the home state model, telemetry simulation, automation evaluation, tests, and WASM export
- `serde`/`serde_json` for structured snapshots
- `wasm-bindgen`/`wasm-pack` for browser integration
- Three.js for the 3D digital twin view
- GitHub Actions for Rust tests and clippy

## Run The Rust Checks

```sh
cargo test --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
```

## Run The Browser Demo

Build the Rust crate to WASM:

```sh
wasm-pack build crates/home-twin --target web --out-dir ../../web/pkg --features wasm
```

Serve the web folder:

```sh
cd web
python3 -m http.server 8081
```

Then open:

```text
http://localhost:8081
```

## Project Shape

```text
.
├── crates/home-twin      # Rust model, simulation, automation logic, WASM export
├── web                   # Three.js demo consuming the Rust snapshot
└── .github/workflows     # CI
```

## Roadmap

- Add a Home Assistant-compatible adapter behind the current mock state
- Persist snapshots for before/after automation comparisons
- Add multiple floors and camera zones
- Add screenshots or a short walkthrough video to the README
- Turn room/device selection into a control surface for real automations
