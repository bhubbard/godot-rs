pub mod audio_server;
pub mod input_server;
pub mod physics_server_2d;
pub mod rendering_server;

pub use audio_server::AudioServer;
pub use input_server::Input;
pub use physics_server_2d::{PhysicsBody2DDesc, PhysicsServer2D, RayCastResult2D};
pub use rendering_server::{DrawCommand, RenderingServer};

pub fn init_servers() {
    RenderingServer::init();
    PhysicsServer2D::init();
    AudioServer::init();
    Input::init();
}
