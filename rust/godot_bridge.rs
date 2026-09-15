#![allow(unsafe_code, reason = "Godot requires one unsafe ExtensionLibrary implementation")]

use godot::classes::{INode2D, Node2D};
use godot::prelude::*;

struct TerraneNBodyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for TerraneNBodyExtension {}

#[derive(GodotClass)]
#[class(base = Node2D)]
struct TerraneNBodyView {
    state: Option<super::SimulationState>,
    accumulated_time: f64,
    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for TerraneNBodyView {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            state: Some(super::default_system()),
            accumulated_time: 0.0,
            base,
        }
    }

    fn process(&mut self, delta: f64) {
        const FIXED_STEP: f64 = 1.0 / 120.0;
        const MAX_FRAME_DELTA: f64 = 0.25;

        self.accumulated_time += delta.min(MAX_FRAME_DELTA);
        while self.accumulated_time >= FIXED_STEP {
            let state = self
                .state
                .take()
                .expect("simulation state is restored after every fixed step");
            self.state = Some(super::step_cpu(state, FIXED_STEP));
            self.accumulated_time -= FIXED_STEP;
        }
        self.base_mut().queue_redraw();
    }

    fn draw(&mut self) {
        let frame = super::snapshot(
            self.state
                .as_ref()
                .expect("simulation state exists outside a fixed step"),
        );
        for body in frame.bodies {
            let radius = (body.mass.sqrt() * 0.22).clamp(3.0, 14.0) as f32;
            let position = Vector2::new(body.x as f32, body.y as f32);
            self.base_mut()
                .draw_circle(position, radius, Color::from_rgb(0.45, 0.78, 1.0));
        }
    }
}
