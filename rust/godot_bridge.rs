#![allow(
    unsafe_code,
    reason = "Godot requires one unsafe ExtensionLibrary implementation"
)]

use godot::classes::{INode2D, Node2D};
use godot::prelude::*;

struct TerraneNBodyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for TerraneNBodyExtension {}

#[derive(GodotClass)]
#[class(base = Node2D)]
struct TerraneNBodyView {
    state: Option<super::GodotViewState>,
    base: Base<Node2D>,
}

#[godot_api]
impl TerraneNBodyView {
    #[func]
    fn completed_steps(&self) -> i64 {
        self.state
            .as_ref()
            .expect("simulation state exists outside a fixed step")
            .completed_steps
    }

    #[func]
    fn simulation_time_scale(&self) -> f64 {
        super::simulation_time_scale()
    }
}

#[godot_api]
impl INode2D for TerraneNBodyView {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            state: Some(super::new_godot_view()),
            base,
        }
    }

    fn process(&mut self, delta: f64) {
        let state = self
            .state
            .take()
            .expect("simulation state is restored after every frame");
        self.state = Some(super::advance_godot_view(state, delta));
        self.base_mut().queue_redraw();
    }

    fn draw(&mut self) {
        let frame = super::render_godot_frame(
            self.state
                .as_ref()
                .expect("simulation state exists outside a frame update"),
        );
        for body in frame.bodies {
            let position = super::godot_draw_position(body.x as f32, body.y as f32);
            self.base_mut().draw_circle(
                position,
                body.radius as f32,
                super::godot_body_color(&body),
            );
        }
    }
}
