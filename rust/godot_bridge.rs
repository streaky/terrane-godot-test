#![allow(unsafe_code, reason = "Godot requires one unsafe ExtensionLibrary implementation")]

use godot::classes::{INode2D, Node2D};
use godot::prelude::*;

struct TerraneNBodyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for TerraneNBodyExtension {}

#[derive(GodotClass)]
#[class(base = Node2D)]
struct TerraneNBodyView {
    state: super::SimulationState,
    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for TerraneNBodyView {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            state: super::default_system(),
            base,
        }
    }

    fn process(&mut self, delta: f64) {
        let state = self.state.clone();
        self.state = super::step_cpu(state, delta.min(1.0 / 30.0));
        self.base_mut().queue_redraw();
    }

    fn draw(&mut self) {
        let frame = super::snapshot(&self.state);
        for body in frame.bodies {
            let radius = (body.mass.sqrt() * 0.22).clamp(3.0, 14.0) as f32;
            let position = Vector2::new(body.x as f32, body.y as f32);
            self.base_mut()
                .draw_circle(position, radius, Color::from_rgb(0.45, 0.78, 1.0));
        }
    }
}
