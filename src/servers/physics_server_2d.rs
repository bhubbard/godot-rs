use crate::core::math::{Rect2, Vector2};
use std::sync::RwLock;

#[derive(Debug, Clone)]
pub struct RayCastResult2D {
    pub position: Vector2,
    pub normal: Vector2,
    pub collider_id: u64,
}

#[derive(Debug, Clone)]
pub struct PhysicsBody2DDesc {
    pub id: u64,
    pub position: Vector2,
    pub size: Vector2,
    pub is_static: bool,
}

#[derive(Debug, Default)]
pub struct PhysicsServer2D {
    bodies: Vec<PhysicsBody2DDesc>,
    pub gravity: Vector2,
}

static PHYSICS_SERVER_2D: RwLock<Option<PhysicsServer2D>> = RwLock::new(None);

impl PhysicsServer2D {
    pub fn init() {
        let mut lock = PHYSICS_SERVER_2D.write().unwrap();
        *lock = Some(Self {
            bodies: Vec::new(),
            gravity: Vector2::new(0.0, 980.0),
        });
    }

    pub fn register_body(desc: PhysicsBody2DDesc) {
        if let Ok(mut lock) = PHYSICS_SERVER_2D.write() {
            if let Some(ref mut ps) = *lock {
                ps.bodies.retain(|b| b.id != desc.id);
                ps.bodies.push(desc);
            }
        }
    }

    pub fn unregister_body(id: u64) {
        if let Ok(mut lock) = PHYSICS_SERVER_2D.write() {
            if let Some(ref mut ps) = *lock {
                ps.bodies.retain(|b| b.id != id);
            }
        }
    }

    pub fn raycast(from: Vector2, to: Vector2) -> Option<RayCastResult2D> {
        let lock = PHYSICS_SERVER_2D.read().ok()?;
        let ps = lock.as_ref()?;

        let mut closest_dist = f32::INFINITY;
        let mut closest_res = None;

        for body in &ps.bodies {
            let rect = Rect2::new(body.position - body.size * 0.5, body.size);
            // Quick ray-AABB intersection test
            if let Some((hit_pos, hit_norm)) = ray_rect_intersect(from, to, rect) {
                let dist = from.distance_squared_to(hit_pos);
                if dist < closest_dist {
                    closest_dist = dist;
                    closest_res = Some(RayCastResult2D {
                        position: hit_pos,
                        normal: hit_norm,
                        collider_id: body.id,
                    });
                }
            }
        }

        closest_res
    }
}

fn ray_rect_intersect(from: Vector2, to: Vector2, rect: Rect2) -> Option<(Vector2, Vector2)> {
    let dir = to - from;
    let inv_d = Vector2::new(
        if dir.x != 0.0 { 1.0 / dir.x } else { f32::INFINITY },
        if dir.y != 0.0 { 1.0 / dir.y } else { f32::INFINITY },
    );

    let t0 = (rect.position - from) * inv_d;
    let t1 = (rect.end() - from) * inv_d;

    let tmin_x = t0.x.min(t1.x);
    let tmax_x = t0.x.max(t1.x);
    let tmin_y = t0.y.min(t1.y);
    let tmax_y = t0.y.max(t1.y);

    let t_near = tmin_x.max(tmin_y);
    let t_far = tmax_x.min(tmax_y);

    if t_near > t_far || t_far < 0.0 || t_near > 1.0 {
        return None;
    }

    let t = if t_near >= 0.0 { t_near } else { t_far };
    let hit_pos = from + dir * t;

    let normal = if (hit_pos.x - rect.position.x).abs() < 0.001 {
        Vector2::LEFT
    } else if (hit_pos.x - rect.end().x).abs() < 0.001 {
        Vector2::RIGHT
    } else if (hit_pos.y - rect.position.y).abs() < 0.001 {
        Vector2::UP
    } else {
        Vector2::DOWN
    };

    Some((hit_pos, normal))
}
