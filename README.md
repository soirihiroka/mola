# Mola

Mola (named after the Mola mola) is a work-in-progress realtime 3D motion-capture for vtubers.

This repository contains native display code and a web frontend for capturing and visualizing motion data.

The project aims to provide low-latency capture, simple tooling for facial/hand/pose tracking, and realtime rendering for prototyping and demos.

## Quick overview

- Rust realtime rendering with wgpu in `display/`
- Web motion capture in `mocap/web/`

## Getting started

Prerequisites

- Rust toolchain with cargo, and all the dependencies of Bevy.
- Node.js (v16+) and npm or pnpm for the web frontend

Run the native display client (local development)

### Display

#### Build

Change into the `display` folder and build:

```bash
cd display
cargo build
```

#### Run

```bash
cargo run --release
```

#### Hot Reload

Alternatively, you can run make to debug it with hot reload.

### Motion Capture

Run the web frontend (mocap capture + UI)

Change into the web folder and install dependencies:

```bash
cd mocap/web
npm install
```

Start the dev server:

```bash
npm run dev
```

Open the app in your browser (usually <http://localhost:3000/>).

If the web motion capture client isn't run on the same host as the display client, you may need to use some proxy to enable HTTPS.

## Repository layout

- `display/` — Rust realtime renderer and plugins
  - `src/` — main application code (camera, scene, character control, materials, api)
  - `shaders/`, `assets/` — shaders, fonts, models and other assets
  - `Cargo.toml` — Rust crate manifest

- `mocap/web/` — web frontend (Vite + React/TS)
  - `src/` — web app source
  - `package.json` — frontend scripts and dependencies

- top-level files: `README.md`, `package.json` (workspace helper), project metadata

## Features and current status

- Realtime 3D display using wgpu (in `display/`)
- Web UI for capture/demo (in `mocap/web/`)

This project will not likely to be actively worked on.

## License

This project is licensed under the MIT License.
