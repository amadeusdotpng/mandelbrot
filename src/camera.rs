use winit::{
    event::*,
    keyboard::{KeyCode, PhysicalKey},
};

pub struct Camera {
    pub(crate) scale: f32,
    pub(crate) position: [f32; 2],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    // although this is a 3x3 matrix, we need treat it as a 3x4 matrix since vec3's have an
    // alignment of 16 bytes in wgsl. we can simply add some padding at the end of each row.
    view_mat: [[f32; 4]; 3], 
}

impl Camera {
    pub fn new(scale: f32, position: [f32; 2]) -> Self {
        Self { scale, position }
    }

    fn translate_horiz(&mut self, dx: i8) {
        self.position[0] += (dx as f32) * (1.0 / self.scale);
    }

    fn translate_vert(&mut self, dy: i8) {
        self.position[1] += (dy as f32) * (1.0 / self.scale);
    }

    fn scale(&mut self, ds: f32) {
        self.scale = f32::max(self.scale * ds, 0.5);
    }

    pub fn process_events(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                event: KeyEvent {
                    state: ElementState::Pressed,
                    physical_key: PhysicalKey::Code(keycode),
                    ..
                },
                ..
            } => match keycode {
                KeyCode::KeyW | KeyCode::ArrowUp    => { self.translate_vert(1); true }
                KeyCode::KeyS | KeyCode::ArrowDown  => { self.translate_vert(-1); true }
                KeyCode::KeyD | KeyCode::ArrowRight => { self.translate_horiz(1); true }
                KeyCode::KeyA | KeyCode::ArrowLeft  => { self.translate_horiz(-1); true }
                KeyCode::KeyQ  => { self.scale(0.5); true }
                KeyCode::KeyE  => { self.scale(2.0); true }
                _ => false
            }
            _ => false
        }
    }

    pub fn into_matrix(&self) -> CameraUniform {
        let ds = 1.0 / self.scale;
        let dx = self.position[0];
        let dy = self.position[1];
        // wgsl's matrices are column-major ordered, i.e. these are four vec3's going down.
        // We could have represented this as a three vec4's going down as well if we wanted to.
        let view_mat = [
            [ ds , 0.0, 0.0, 0.0],
            [ 0.0, ds , 0.0, 0.0],
            [ dx , dy , 1.0, 0.0],
        ];

        CameraUniform { view_mat }
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            scale: 0.5,
            position: [-0.5, 0.0]
        }
    }
    
}

