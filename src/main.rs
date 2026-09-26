use clap::{Parser, Subcommand};
use colored::*;
use godot::prelude::*;
use std::fs;
use std::path::Path;
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(name = "godot")]
#[command(version = "0.1.0")]
#[command(about = "Godot-RS: Pure Rust port and fork of the Godot Engine architecture", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Run in headless mode (no graphical window)
    #[arg(long, default_value_t = false)]
    headless: bool,

    /// Scene to run (if no subcommand provided)
    scene: Option<String>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Run a Godot .tscn scene file
    Run {
        /// Path to .tscn scene
        scene: String,

        /// Number of simulation ticks (default: 60)
        #[arg(short, long, default_value_t = 60)]
        ticks: u32,

        /// Delta time per tick in seconds (default: 0.016666)
        #[arg(short, long, default_value_t = 0.016666)]
        delta: f32,
    },

    /// Parse and display AST of a .tscn scene
    Parse {
        /// Path to .tscn scene
        scene: String,
    },

    /// Initialize a new Godot project template
    New {
        /// Project directory name
        name: String,
    },

    /// Benchmark Variant system, Vector Math, and SceneTree processing
    Bench,

    /// Display Godot-RS engine architecture and ClassDB information
    Info,

    /// Run built-in interactive terminal game loop demo
    Demo,
}

fn main() {
    ClassDb::init();
    init_servers();

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Run { scene, ticks, delta }) => {
            run_scene(&scene, ticks, delta);
        }
        Some(Commands::Parse { scene }) => {
            parse_scene(&scene);
        }
        Some(Commands::New { name }) => {
            create_new_project(&name);
        }
        Some(Commands::Bench) => {
            run_benchmarks();
        }
        Some(Commands::Info) => {
            show_info();
        }
        Some(Commands::Demo) => {
            run_demo();
        }
        None => {
            if let Some(scene) = cli.scene {
                run_scene(&scene, 60, 0.016666);
            } else {
                show_info();
            }
        }
    }
}

fn show_info() {
    println!("{}", "=========================================================".bright_cyan());
    println!("{}", "               GODOT ENGINE - RUST FORK (godot-rs)        ".bright_green().bold());
    println!("{}", "=========================================================".bright_cyan());
    println!("  {} Pure Rust architecture port of Godot 4", "Version:".bright_yellow());
    println!("  {} bhubbard / Brandon Hubbard", "Author:".bright_yellow());
    println!("  {} MIT OR Apache-2.0", "License:".bright_yellow());
    println!();
    println!("{}", "Registered ClassDB Core Classes:".bright_cyan().bold());
    let classes = [
        "Object", "RefCounted", "Resource", "PackedScene",
        "Node", "CanvasItem", "Node2D", "Sprite2D", "Camera2D",
        "CharacterBody2D", "CollisionShape2D", "Area2D",
        "Control", "Label", "Button", "ColorRect",
        "Node3D", "VisualInstance3D", "MeshInstance3D", "Camera3D",
        "DirectionalLight3D", "CharacterBody3D", "Viewport", "Window",
    ];
    for chunk in classes.chunks(4) {
        println!("    {}", chunk.join(", ").white());
    }
    println!();
    println!("{}", "Subsystems & Servers Initialized:".bright_cyan().bold());
    println!("  ✓ RenderingServer (Canvas 2D / Visual Draw Pipeline)");
    println!("  ✓ PhysicsServer2D (Raycast & AABB collision queries)");
    println!("  ✓ AudioServer (Audio buses & attenuation)");
    println!("  ✓ Input (Action mapping, axis queries, input events)");
    println!("  ✓ TscnParser / TscnSerializer (Godot 4 .tscn engine parser)");
    println!();
    println!("Run {} for commands, or {} to see an interactive simulation.", "godot --help".green(), "godot demo".green());
}

fn run_scene(path: &str, ticks: u32, delta: f32) {
    println!("{} Loading scene: {}", "●".green(), path);
    match TscnParser::parse_file(path) {
        Ok(packed) => {
            let mut tree = SceneTree::new();
            let root = tree.get_root();
            if let Some(scene_root) = packed.instantiate(&mut tree) {
                tree.add_child(root, scene_root);
                tree.set_current_scene(scene_root);

                println!("{} Scene successfully instantiated. Node hierarchy:", "✓".green());
                println!("{}", tree.print_tree_pretty());

                println!("{} Executing {} simulation frames at delta={:.4}s...", "▶".yellow(), ticks, delta);
                let start = Instant::now();
                for _ in 0..ticks {
                    tree.step(delta);
                    tree.physics_step(delta);
                }
                let elapsed = start.elapsed();
                println!("{} Completed in {:?} ({:.2} µs/frame)", "✓".green(), elapsed, elapsed.as_micros() as f64 / ticks as f64);
            } else {
                eprintln!("{} Failed to instantiate root node from scene", "✗".red());
            }
        }
        Err(e) => {
            eprintln!("{} Error parsing scene {}: {}", "✗".red(), path, e);
        }
    }
}

fn parse_scene(path: &str) {
    println!("{} Parsing .tscn file: {}", "●".cyan(), path);
    match TscnParser::parse_file(path) {
        Ok(packed) => {
            println!("{} Successfully parsed {} nodes:", "✓".green(), packed.nodes.len());
            for (idx, node) in packed.nodes.iter().enumerate() {
                let parent = node.parent_path.as_deref().unwrap_or("<root>");
                println!("  [{}] Node \"{}\" ({}) parent: {}", idx, node.name.bright_green(), node.type_name.bright_yellow(), parent);
                for (k, v) in &node.properties {
                    println!("       • {} = {}", k.cyan(), v);
                }
                if !node.groups.is_empty() {
                    println!("       • groups: {:?}", node.groups);
                }
            }
        }
        Err(e) => {
            eprintln!("{} Error: {}", "✗".red(), e);
        }
    }
}

fn create_new_project(name: &str) {
    let project_dir = Path::new(name);
    if project_dir.exists() {
        eprintln!("{} Directory {} already exists!", "✗".red(), name);
        return;
    }

    if let Err(e) = fs::create_dir_all(project_dir) {
        eprintln!("{} Failed to create directory: {}", "✗".red(), e);
        return;
    }

    let project_godot = format!(
r#"; Engine configuration file.
; It's best edited using the editor UI and not directly,
; but since the parameters are simple, they can be easily edited.

[application]

config/name="{}"
run/main_scene="res://main.tscn"
config/features=PackedStringArray("4.3", "Forward Plus")

[display]

window/size/viewport_width=1152
window/size/viewport_height=648
"#,
        name
    );

    let main_tscn = r#"[gd_scene format=3]

[node name="Main" type="Node2D"]
position = Vector2(0, 0)

[node name="Player" type="CharacterBody2D" parent="."]
position = Vector2(576, 324)
velocity = Vector2(100, -50)
groups = ["players"]

[node name="Sprite2D" type="Sprite2D" parent="Player"]
texture = "res://icon.svg"
position = Vector2(0, 0)

[node name="CollisionShape2D" type="CollisionShape2D" parent="Player"]

[node name="ScoreLabel" type="Label" parent="."]
text = "Score: 0"
position = Vector2(50, 50)
"#;

    let _ = fs::write(project_dir.join("project.godot"), project_godot);
    let _ = fs::write(project_dir.join("main.tscn"), main_tscn);

    println!("{} Created new Godot project in {}", "✓".green(), name.bold());
    println!("  Files created:");
    println!("    • {}/project.godot", name);
    println!("    • {}/main.tscn", name);
    println!();
    println!("Run it with: {} run {}/main.tscn", "godot".green(), name);
}

fn run_benchmarks() {
    println!("{}", "Starting Godot-RS Performance Benchmarks...".bright_cyan().bold());

    // 1. Math benchmark
    let count = 1_000_000;
    let t0 = Instant::now();
    let mut v = Vector2::new(1.0, 2.0);
    for i in 0..count {
        v = v.rotated(0.01) + Vector2::new(0.5, -0.5);
        if i % 10 == 0 {
            v = v.normalized();
        }
    }
    let d_math = t0.elapsed();
    println!("  ✓ Vector2 transforms ({} ops): {:?} ({:.1} ns/op)", count, d_math, (d_math.as_nanos() as f64) / count as f64);

    // 2. Variant creation and arithmetic
    let t1 = Instant::now();
    let mut var = Variant::Int(0);
    for _ in 0..count {
        var = var + Variant::Int(1);
    }
    let d_var = t1.elapsed();
    println!("  ✓ Variant arithmetic ({} ops): {:?} ({:.1} ns/op)", count, d_var, (d_var.as_nanos() as f64) / count as f64);

    // 3. SceneTree processing benchmark
    let mut tree = SceneTree::new();
    let root = tree.get_root();
    for i in 0..1_000 {
        let mut node = Node::new(format!("Entity_{}", i), "CharacterBody2D");
        node.set_property("velocity", Variant::Vector2(Vector2::new(10.0, 20.0)));
        let nid = tree.add_node(node);
        tree.add_child(root, nid);
    }

    let steps = 1_000;
    let t2 = Instant::now();
    for _ in 0..steps {
        tree.step(0.016);
        tree.physics_step(0.016);
    }
    let d_tree = t2.elapsed();
    println!("  ✓ 1,000 Node SceneTree Step ({} frames): {:?} ({:.2} µs/frame)", steps, d_tree, (d_tree.as_micros() as f64) / steps as f64);
    println!("{}", "Benchmarks completed successfully!".bright_green());
}

fn run_demo() {
    println!("{}", "=== Godot-RS Interactive Terminal Simulation Demo ===".bright_green().bold());
    let mut tree = SceneTree::new();
    let root = tree.get_root();

    let mut player = Node::new("Player", "CharacterBody2D");
    player.set_property("position", Variant::Vector2(Vector2::new(10.0, 10.0)));
    player.set_property("velocity", Variant::Vector2(Vector2::new(25.0, 50.0)));
    player.add_to_group("actors");
    let player_id = tree.add_node(player);
    tree.add_child(root, player_id);

    let camera = Node::new("Camera2D", "Camera2D");
    let camera_id = tree.add_node(camera);
    tree.add_child(player_id, camera_id);

    let ui = Node::new("HUD", "Control");
    let ui_id = tree.add_node(ui);
    tree.add_child(root, ui_id);

    let mut label = Node::new("HealthLabel", "Label");
    label.set_property("text", Variant::String("HP: 100".to_string()));
    let label_id = tree.add_node(label);
    tree.add_child(ui_id, label_id);

    println!("Initial SceneTree:");
    println!("{}", tree.print_tree_pretty());

    println!("Stepping physics simulation for 10 frames (gravity & velocity)...");
    for frame in 1..=10 {
        tree.step(0.1);
        tree.physics_step(0.1);
        let p_node = tree.get_node(player_id).unwrap();
        if let NodeData::CharacterBody2D(ref cb) = p_node.data {
            println!("  [Frame {:02}] Player pos: ({:.1}, {:.1}) vel: ({:.1}, {:.1}) floor: {}",
                frame, cb.node_2d.position.x, cb.node_2d.position.y, cb.velocity.x, cb.velocity.y, cb.on_floor);
        }
    }
    println!("{}", "Simulation run finished!".green().bold());
}
