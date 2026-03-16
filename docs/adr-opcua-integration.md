# ADR: OPC UA Integration for Machine Connectivity

**Status:** Proposed  
**Date:** March 2026  
**Context:** Howick factory, Si Racha, Thailand

---

## Context

Currently the Howick FRAMA machine receives job files via USB stick or local network file share.
This is manual, error-prone, and provides no feedback from the machine back to the design system.

The CEO (Prin) is open to replacing this with a smarter integration layer.

## Decision

Replace the USB dongle workflow with a small compute module running an OPC UA server,
sitting physically adjacent to the Howick machine and connected to the factory LAN.

## Architecture

```
plat-trunk (browser / CF Worker)
        │
        │ Automerge CRDT sync
        ▼
  Edge Agent (Rust binary on mini compute module)
  ┌─────────────────────────────────┐
  │  async-opcua SERVER             │  ← exposes machine state to network
  │  async-opcua CLIENT             │  ← connects to machine if it has OPC UA
  │  howick-rs                      │  ← generates CSV from plat-trunk geometry
  │  File watcher / job submitter   │  ← drops CSV to machine input folder
  └─────────────────────────────────┘
        │
        │ CSV file drop (existing interface)
        │ OPC UA (future, if Howick adds support)
        ▼
  Howick FRAMA Machine
```

## Hardware Options for the Edge Agent

| Option | Pros | Cons |
|--------|------|------|
| Raspberry Pi 5 | Cheap (~$80), small, silent | ARM, may need cross-compile setup |
| Intel NUC | x86, easy to develop on | Larger, more expensive |
| Mac Mini M4 | Powerful, runs macOS + Rust natively | Expensive, overkill for v1 |
| Beelink Mini PC | x86, cheap (~$150), Windows or Linux | Less known hardware |

Recommended for v1: **Raspberry Pi 5** with Raspberry Pi OS.  
Cross-compile from MacBook via `cross` or `cargo-cross`.

## OPC UA Crate

`async-opcua` — https://github.com/FreeOpcUa/async-opcua  
- Pure Rust, no C dependencies
- Full client + server in one crate
- Tokio async, MPL-2.0
- Active development (150+ commits in 2025)
- Contributors: Adam Lock (original author), Einar Omang, Sander van Harmelen

```toml
[dependencies]
async-opcua = { version = "0.18", features = ["client", "server"] }
```

## What the OPC UA Server Exposes

Proposed address space nodes:

```
/Howick/
  Machine/
    Status          (Running | Idle | Error | Offline)
    CurrentJob      (frameset name string)
    PiecesProduced  (uint32, resets per job)
    CoilRemaining   (float, metres — if sensor available)
    LastError       (string)
  Jobs/
    Queue/          (list of pending framesets)
    Completed/      (list of completed framesets with timestamps)
  Control/
    SubmitJob()     (method: accepts CSV bytes, queues job)
    CancelJob()     (method: cancels current job)
```

## Relation to plat-trunk

The edge agent bridges plat-trunk's Automerge CRDT sync layer and the physical machine:

1. Designer completes a frameset in plat-trunk browser
2. Automerge sync delivers the frameset data to the edge agent
3. Edge agent uses `howick-rs` to generate the CSV
4. CSV is submitted to the machine (file drop initially, OPC UA method call later)
5. Machine status flows back via OPC UA → edge agent → Automerge → browser

This closes the loop: **design → manufacture → status** all visible in plat-trunk.

## Companion Repo

https://github.com/joeblew999/opcua-howick (to be created)

Will contain:
- The edge agent binary
- OPC UA address space definition
- Howick machine file watcher
- Cross-compilation setup for Raspberry Pi
