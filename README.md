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
- `nbody` contains the package entrypoint used as a compile-time integration check.

A CUDA backend is deliberately not stubbed yet. It is a later backend of the same state-to-snapshot model, and adding a placeholder now would not validate any CUDA behavior.

## Requirements

- Terrane built from this repository;
- Rust toolchain compatible with `godot` 0.5.5;
- Godot 4.6 or newer when running the visualization.

## Build

From this directory:

```bash
./build-extension.sh
```

The helper asks Terrane to build the package as a `cdylib`, then installs the
resulting library at `godot/bin/libterrane_nbody.so`, the stable path referenced
by `terrane_nbody.gdextension`. The current packaging surface intentionally
targets Linux x86-64; supporting another platform requires matching library
entries and installation names in both files.

For compiler and simulation checks:

```bash
../../target/debug/terrane check .
../../target/debug/terrane test .
```

Because this package emits a dynamic library, its Terrane `main` declaration is
a compile-time package entrypoint rather than a runnable program. The unit test
executes the same 600-step CPU simulation and checks known positions.

## Run

```bash
godot --path godot
```

For a display-independent smoke run:

```bash
godot --headless --path godot --quit-after 120
```

Some installations name the executable `godot4` instead. Godot accumulates
render-frame time and advances the simulation in fixed 1/120-second steps, so
the trajectory is independent of render-frame cadence. Each rendered frame
requests a Terrane snapshot and draws every body as a circle.

## Current integration gaps

The standard dependency projector was run first, but `godot` currently yields no
projectable members because its public engine surface is macro-generated and
reexport-heavy. In addition, GDExtension registration requires Rust attribute
and derive macros. Both limitations are recorded in
`terrane-integration-adapters`; the project therefore uses one maintained Rust
module for only that non-projectable boundary.

That bridge intentionally names lowered Terrane functions, types, and public
fields directly. The generated crate's Cargo compilation checks this boundary;
renaming the Terrane declarations or changing compiler member visibility will
fail there rather than during Terrane semantic checking.

The initial conditions are a useful drawing and integration stress case, not a
carefully tuned stable orbital system. A later physical model should choose and
test its own conserved quantities and equilibrium conditions.
