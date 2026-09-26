//! # Godot-RS: Pure Rust Port and Architecture of Godot Engine
//!
//! A high-performance, modular pure Rust implementation of the Godot Engine architecture.
//!
//! ## Modules Overview
//!
//! - **`core`**: Variant system, Math primitives (Vector2/3/4, Basis, Transform2D/3D, Quaternion, Color, Rect2),
//!   StringName, NodePath, Object model, Signal/Callable system, and ClassDB reflection.
//! - **`scene`**: SceneTree, Node hierarchy, 2D/3D nodes, UI Control nodes, PackedScene, and Resource management.
//! - **`servers`**: RenderingServer, PhysicsServer2D, AudioServer, and Input event system.
//! - **`tscn`**: Godot 4 text scene parser (`.tscn`) and serializer.
//!
//! ## Example
//!
//! ```rust
//! use godot::prelude::*;
//!
//! // 1. Initialize engine reflection and servers
//! ClassDb::init();
//! init_servers();
//!
//! // 2. Create SceneTree and root nodes
//! let mut tree = SceneTree::new();
//! let root = tree.get_root();
//!
//! // 3. Build a player node
//! let mut player = Node::new("Player", "CharacterBody2D");
//! player.set_property("position", Variant::Vector2(Vector2::new(100.0, 200.0)));
//! player.set_property("velocity", Variant::Vector2(Vector2::new(150.0, 0.0)));
//! player.add_to_group("players");
//! let player_id = tree.add_node(player);
//! tree.add_child(root, player_id);
//!
//! // 4. Step physics simulation
//! tree.physics_step(1.0 / 60.0);
//!
//! let player_ref = tree.get_node(player_id).unwrap();
//! println!("Player positioned at {:?}", player_ref.get_property("position"));
//! ```

pub mod core;
pub mod prelude;
pub mod scene;
pub mod servers;
pub mod tscn;

pub use prelude::*;
