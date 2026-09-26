# Godot-RS (`godot`)

[![Crates.io](https://img.shields.io/badge/crates.io-v0.1.0-orange.svg)](https://crates.io)
[![Documentation](https://docs.rs/godot-rs/badge.svg)](https://docs.rs/godot-rs)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)

A pure, high-performance Rust port and architecture fork of [Godot Engine](https://github.com/godotengine/godot).

`godot-rs` implements Godot 4's foundational architecture directly in idiomatic Rust: the dynamic `Variant` container, 2D/3D spatial math, the `GodotObject` / `ClassDB` reflection database, the hierarchical `SceneTree` and node lifecycle pipeline, low-level servers (`RenderingServer`, `PhysicsServer2D`, `AudioServer`, `Input`), and full parsing / round-trip serialization of Godot 4 `.tscn` text scene files.

---

## Architecture

```
godot-rs/
├── src/
│   ├── core/
│   │   ├── math/          # Vector2/3/4, Basis, Transform2D/3D, Quaternion, Color, Rect2, AABB, Plane
│   │   ├── variant/       # 35 Variant types, arithmetic operations, type conversions, Callable, Signal
│   │   ├── object/        # ObjectId, GodotObject, ClassDB reflection, signal connections & notifications
│   │   ├── string/        # StringName (interned string ID), NodePath parser & resolution
│   │   └── io/            # project.godot configuration file parser
│   ├── scene/
│   │   ├── node.rs        # Node base class, process modes, groups, 2D/3D/UI specialized data
│   │   ├── scene_tree.rs  # SceneTree runner, root Viewport, tree hierarchy traversal, process/physics step
│   │   └── resources/     # Resource base class, PackedScene instantiation & scene state
│   ├── servers/           # RenderingServer (draw lists), PhysicsServer2D (raycast & bodies), AudioServer, Input
│   ├── tscn/              # Godot 4 .tscn scene lexer, parser, and round-trip serializer
│   ├── lib.rs             # Public API
│   ├── main.rs            # Godot CLI tool (`godot`)
│   └── prelude.rs         # Re-exported types for rapid development
└── tests/                 # Unit tests for math, variant, scene tree, and .tscn parsing
```

---

## Features

- **Pure Rust**: No C++ compilation, no Godot binary dependency, runs anywhere Rust compiles (native macOS/Linux/Windows and WebAssembly).
- **Core Variant System**: Full implementation of Godot's universal dynamically-typed `Variant` enum with arithmetic operators, equality, and conversions.
- **Godot Math**: 1:1 API with Godot's vector, matrix, and transform math (`Vector2`, `Vector3`, `Transform2D`, `Transform3D`, `Basis`, `Quaternion`, `Color`, `Rect2`, `Aabb`, `Plane`).
- **SceneTree & Node Hierarchy**: Complete scene tree with parent-child ownership, relative & absolute `NodePath` queries (`^"Player/Sprite2D"`, `^"../Enemy"`), groups, and pause modes (`Inherit`, `Pausable`, `WhenPaused`, `Always`, `Disabled`).
- **Godot 4 `.tscn` Parser**: Directly parses `.tscn` files into a `PackedScene` resource and instantiates them into live nodes in the `SceneTree`.
- **Built-in CLI**: Run scenes, inspect ASTs, scaffold new projects, execute benchmarks, and run interactive terminal simulations.

---

## Quick Start

### Library Usage

Add `godot-rs` to your `Cargo.toml`:

```toml
[dependencies]
godot = { package = "godot-rs", version = "0.1.0" }
```

```rust
use godot::prelude::*;

fn main() {
    // 1. Initialize reflection and subsystems
    ClassDb::init();
    init_servers();

    // 2. Initialize SceneTree
    let mut tree = SceneTree::new();
    let root = tree.get_root();

    // 3. Create nodes
    let mut player = Node::new("Player", "CharacterBody2D");
    player.set_property("position", Variant::Vector2(Vector2::new(100.0, 150.0)));
    player.set_property("velocity", Variant::Vector2(Vector2::new(60.0, -10.0)));
    player.add_to_group("players");

    let player_id = tree.add_node(player);
    tree.add_child(root, player_id);

    // 4. Step physics simulation
    for _ in 0..60 {
        tree.step(1.0 / 60.0);
        tree.physics_step(1.0 / 60.0);
    }

    println!("{}", tree.print_tree_pretty());
}
```

---

## CLI Usage

Install the CLI tool:

```bash
cargo install --path .
```

### 1. Show Engine Information
```bash
godot info
```

### 2. Scaffold a New Project
```bash
godot new MyGame
cd MyGame
```

### 3. Run a Scene
```bash
godot run MyGame/main.tscn --ticks 120 --delta 0.016666
```

### 4. Parse and Inspect a `.tscn` File
```bash
godot parse MyGame/main.tscn
```

### 5. Run Performance Benchmarks
```bash
godot bench
```

### 6. Interactive Terminal Simulation Demo
```bash
godot demo
```

---

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
