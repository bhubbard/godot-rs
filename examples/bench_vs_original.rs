use std::time::Instant;
use godot::prelude::*;

fn main() {
    println!("============================================================");
    println!("     godot-rs (Rust) vs Godot Engine C++ / GDExtension Bench");
    println!("============================================================");

    ClassDb::init();
    init_servers();

    // 1. Variant Arithmetic & Construction Micro-Benchmark
    println!("\n--- 1. Variant Dynamic Value & Arithmetic Throughput ---");
    {
        let iterations = 10_000_000;
        let start = Instant::now();
        let mut sum = Variant::Int(0);

        for i in 0..iterations {
            let v = Variant::Int((i % 100) as i64);
            sum = sum + v;
        }

        let elapsed = start.elapsed();
        let ns_per_op = elapsed.as_nanos() as f64 / iterations as f64;
        let ops_per_sec = iterations as f64 / elapsed.as_secs_f64();

        println!(
            "Variant Ops: {} | Time: {:.2?} | Latency: {:.2} ns/op | {:>10.0} ops/s | Final: {:?}",
            iterations, elapsed, ns_per_op, ops_per_sec, sum
        );
    }

    // 2. SceneTree Physics Step & Node Hierarchy Traversal
    println!("\n--- 2. SceneTree Physics Step & Entity Updating ---");
    for &node_count in &[100, 1000, 5000] {
        let mut tree = SceneTree::new();
        let root = tree.get_root();

        let main_node = Node::new("Main", "Node2D");
        let main_id = tree.add_node(main_node);
        tree.add_child(root, main_id);

        for i in 0..node_count {
            let mut cb = Node::new(format!("Actor_{}", i), "CharacterBody2D");
            cb.set_property("velocity", Variant::Vector2(Vector2::new(10.0, 5.0)));
            cb.add_to_group("actors");
            let id = tree.add_node(cb);
            tree.add_child(main_id, id);
        }

        let steps = 1000;
        let start = Instant::now();
        for _ in 0..steps {
            tree.physics_step(0.0166);
        }
        let elapsed = start.elapsed();
        let step_latency = elapsed / steps as u32;
        let steps_per_sec = steps as f64 / elapsed.as_secs_f64();
        let node_updates_per_sec = (steps * node_count) as f64 / elapsed.as_secs_f64();

        println!(
            "Nodes: {:>5} | Step Latency: {:>8.2?} | {:>8.0} steps/s | {:>10.0} node-updates/s",
            node_count, step_latency, steps_per_sec, node_updates_per_sec
        );
    }

    // 3. NodePath Resolution & Hierarchy Lookup
    println!("\n--- 3. NodePath Resolution & Hierarchy Lookup ---");
    {
        let mut tree = SceneTree::new();
        let root = tree.get_root();
        let main_id = tree.add_node(Node::new("World", "Node2D"));
        tree.add_child(root, main_id);

        let parent_id = tree.add_node(Node::new("Player", "CharacterBody2D"));
        tree.add_child(main_id, parent_id);

        let child_id = tree.add_node(Node::new("Camera", "Camera2D"));
        tree.add_child(parent_id, child_id);

        let path = NodePath::new("Player/Camera");
        let iterations = 1_000_000;
        let start = Instant::now();
        let mut found = 0;

        for _ in 0..iterations {
            if let Some(id) = tree.find_node_by_path(main_id, &path) {
                found += id.as_u64();
            }
        }

        let elapsed = start.elapsed();
        let ns_per_lookup = elapsed.as_nanos() as f64 / iterations as f64;
        let lookups_per_sec = iterations as f64 / elapsed.as_secs_f64();

        println!(
            "NodePath Lookups: {} | Time: {:.2?} | Latency: {:.2} ns/lookup | {:>10.0} lookups/s | Found: {}",
            iterations, elapsed, ns_per_lookup, lookups_per_sec, found
        );
    }

    // 4. Vector2 / Vector3 Math Transformations
    println!("\n--- 4. Transform2D / Vector2 Math Primitives Throughput ---");
    {
        let iterations = 10_000_000;
        let start = Instant::now();
        let xform = Transform2D::from_angle_scale_origin(0.5, Vector2::new(1.0, 1.0), Vector2::new(5.0, 10.0));
        let mut sum_x = 0.0;

        for i in 0..iterations {
            let pt = Vector2::new((i % 100) as f32, (i % 50) as f32);
            let res = xform.xform(pt);
            sum_x += res.x;
        }

        let elapsed = start.elapsed();
        let ns_per_xform = elapsed.as_nanos() as f64 / iterations as f64;
        let xforms_per_sec = iterations as f64 / elapsed.as_secs_f64();

        println!(
            "Vector Transformations: {} | Time: {:.2?} | Latency: {:.2} ns/xform | {:>10.0} xforms/s | SumX: {:.1}",
            iterations, elapsed, ns_per_xform, xforms_per_sec, sum_x
        );
    }

    println!("\n============================================================");
    println!("                      Benchmark Complete                    ");
    println!("============================================================");
}
