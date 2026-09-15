# Terrane Godot N-body experiment

This project exercises a Terrane-authored CPU N-body simulation through the Rust `godot` GDExtension bindings. The simulation and frame model are Terrane code; the maintained Rust module is limited to Godot's macro-defined extension entrypoint and `Node2D` lifecycle/drawing bridge.

## Architecture

```text
Terrane state (position, velocity, mass)
        ↓
Terrane symplectic-Euler CPU integrator
        ↓
Terrane immutable frame snapshot
        ↓
minimal maintained Rust Godot binding
        ↓
Godot Node2D visualization
```

The Terrane source is split by namespace:

- `nbody/model` owns simulation state;
- `nbody/integrator` owns the CPU backend;
- `nbody/setup` owns initial conditions;
- `nbody/snapshot` owns the visualization-facing frame;
- `nbody` contains only the smoke entrypoint.

A CUDA backend is deliberately not stubbed yet. It is a later backend of the same state-to-snapshot model, and adding a placeholder now would not validate any CUDA behavior.

## Requirements

- Terrane built from this repository;
- Rust toolchain compatible with `godot` 0.5.5;
- Godot 4.2 or newer when running the visualization.

## Build

From this directory:

```bash
./build-extension.sh
```

The helper asks Terrane to build the package as a `cdylib`, then installs the resulting library at `godot/bin/libterrane_nbody.so`, the stable path referenced by `terrane_nbody.gdextension`.

For a compiler-only check:

```bash
../../target/debug/terrane check .
```

## Run

```bash
godot4 --path godot
```

Godot calls the Rust binding once per frame. The binding advances the Terrane CPU simulation, requests a Terrane frame snapshot, and draws each body as a circle.

## Current integration gaps

The CPU integrator currently builds next-frame scalar arrays instead of updating
the existing arrays in place. Terrane conservatively retains owner provenance
from indexed reads and rejected the read-then-write loop with `T0059`. That is a
general ownership/optimization question for Terrane rather than a Godot adapter
concern; this experiment leaves the visible value semantics correct and records
the extra per-frame allocation here for later compiler work.

The standard dependency projector was run first, but `godot` currently yields no projectable members because its public engine surface is macro-generated and reexport-heavy. In addition, GDExtension registration requires Rust attribute and derive macros. Both limitations are recorded in `terrane-integration-adapters`; the project therefore uses one maintained Rust module for only that non-projectable boundary.
