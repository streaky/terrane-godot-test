# Terrane Godot N-body experiment

This project exercises a Terrane-authored CPU N-body simulation through the Rust `godot` GDExtension bindings. The simulation, fixed-step controller, render preparation, frame model, and Godot builtin value construction are Terrane code; the maintained Rust module is limited to Godot's macro-defined extension entrypoint and host-mandated `Node2D` lifecycle/drawing ceremony.

[Watch a 10-second recording of the simulation on YouTube.](https://www.youtube.com/watch?v=kqYydz2LnyI)

## Architecture

```text
Terrane state (position, velocity, mass)
        ↓
Terrane symplectic-Euler CPU integrator
        ↓
Terrane immutable frame snapshot
        ↓
Terrane-projected Godot values and minimal host bridge
        ↓
Godot Node2D visualization
```

The Terrane source is split by namespace:

- `nbody/model` owns simulation state;
- `nbody/integrator` owns the CPU backend;
- `nbody/setup` owns initial conditions;
- `nbody/snapshot` owns the visualization-facing frame;
- `nbody/godot` owns fixed-step scheduling and render-ready Godot values;
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

The helper asks Terrane to build the package as a `cdylib`, installs the
resulting library at `godot/bin/libterrane_nbody.so`, and writes Godot's
generated extension list so direct game runs load `terrane_nbody.gdextension`
without requiring an editor scan first. The current packaging surface
intentionally targets Linux x86-64; supporting another platform requires
matching library entries and installation names in both files.

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

Some installations name the executable `godot4` instead. Godot supplies
render-frame deltas to the host callback; the Terrane `nbody/godot` controller
clamps and accumulates them, advances the simulation in fixed 1/120-second
steps, and applies the 20× simulation-time scale. This keeps the trajectory
independent of render-frame cadence while making its deliberately small
world-space velocities visible. The CPU integrator evaluates each unordered
body pair once and applies equal-and-opposite force contributions. Each rendered
frame asks Terrane for render-ready positions, radii, and mass-derived color
tiers, then the host callback issues the Godot drawing calls. The overlay reports
measured simulation steps per second, Godot's rendered video frames per second,
and the simulation-time multiplier.

## Current integration boundary

Terrane imports generated and reexported Godot API through the ordinary
`/deps/godot` projection. The drawing path constructs `Vector2` positions and
mass-derived `Color` values in Terrane. The acceptance fixture checks generated
`Node2D` identity and its projected `get_position` class method, plus projected
`Vector2` construction and methods.

The authored drawing body is tied to specific current declines rather than a
general Godot exception: GDExtension registration requires Rust attribute and
derive macros, lifecycle entry uses the Rust `INode2D` trait, and projected
foreign resources cannot yet be stored in Terrane collections. The render plan
narrows its `float64` coordinates and radii explicitly to `float32` in Terrane,
so the maintained Rust module performs only that host ceremony and final
`draw_circle` calls from Terrane-prepared values. Terrane owns pairwise
simulation, fixed-step scheduling, frame preparation, radius and color policy,
and projected Godot value construction. The generated crate's Cargo compilation
checks this boundary; renaming the Terrane declarations or changing compiler
member visibility fails there rather than during Terrane semantic checking.

The initial conditions are a useful drawing and integration stress case, not a
carefully tuned stable orbital system. A later physical model should choose and
test its own conserved quantities and equilibrium conditions.
