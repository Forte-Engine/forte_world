use cgmath::Point2;
use forte_engine::render::render_engine::RenderEngineInput;
use winit::event::WindowEvent;

pub fn input(event: &WindowEvent) -> Option<RenderEngineInput> {
    match event {
        WindowEvent::CursorMoved { position, .. } => Some(RenderEngineInput::MouseMove(Point2 { x: position.x as f32, y: position.y as f32 })),
        WindowEvent::MouseInput { state, button, .. } => Some(RenderEngineInput::MouseButton(*button, *state)),
        WindowEvent::MouseWheel { delta, .. } => Some(RenderEngineInput::MouseWheel(*delta)),

        WindowEvent::KeyboardInput { input, .. } => {
            if input.virtual_keycode.is_some() {
                Some(RenderEngineInput::KeyInput(input.virtual_keycode.unwrap(), input.state))
            } else { None }
        },

        _ => None
    }
}