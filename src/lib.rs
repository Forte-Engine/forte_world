use forte_engine::render::render_engine::{RenderEngine, RenderEngineInput};
use log::warn;
use nodes::*;
use winit::{event_loop::{EventLoop, ControlFlow}, window::WindowBuilder, event::{Event, WindowEvent}};

pub mod nodes;
pub mod utils;

pub trait WorldApp<C: ComponentsDef, R: ResourceDef> {
    fn create(world: &mut World<C, R>) -> Self;
    fn input(&mut self, world: &mut World<C, R>, input: RenderEngineInput);
    fn update(&mut self, world: &mut World<C, R>);
    fn render(&mut self, world: &mut World<C, R>);
    fn exit(&mut self, world: &mut World<C, R>);
}

pub struct World<C: ComponentsDef, R: ResourceDef> {
    pub resources: R,
    pub root: Node<C>
}

pub trait ResourceDef {
    fn create(engine: RenderEngine) -> Self;
    fn render_engine(&self) -> &RenderEngine;
    fn render_engine_mut(&mut self) -> &mut RenderEngine;
}

pub fn run_world_app<A: WorldApp<C, R> + 'static, C:ComponentsDef + 'static, R: ResourceDef + 'static>() {
    env_logger::init();

    // setup window and event loop
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    // setup engine
    let engine = pollster::block_on(RenderEngine::new(window));

    // create app and world and resources
    let resources = R::create(engine);
    let mut world: World<C, R> = World { resources, root: Node::default() };
    let mut app = A::create(&mut world);

    event_loop.run(move |event, _, flow| {
        match event {
            // window events
            Event::WindowEvent { window_id, ref event } => {
                // make sure it is our window
                if window_id != world.resources.render_engine().window().id() { return }

                // if input is handled, stop here
                let input = utils::input(event);
                if input.is_some() { app.input(&mut world, input.unwrap()); return }
                
                // if close requested, stop
                match event {
                    // exit
                    WindowEvent::CloseRequested => *flow = ControlFlow::Exit, 

                    // resizes
                    WindowEvent::Resized(size) => world.resources.render_engine_mut().resize(*size),
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => world.resources.render_engine_mut().resize(**new_inner_size),
                    
                    _ => {}
                }
            },

            Event::MainEventsCleared => world.resources.render_engine().window().request_redraw(),
            Event::RedrawRequested(window_id) => {
                // make sure our window
                if window_id != world.resources.render_engine().window().id() { return }

                // update app then egui
                app.update(&mut world);

                // todo
                // render the app via the engine
                // engine.render(app)
                // match engine.render(&mut app) {
                //     // good
                //     Ok(_) => {}

                //     // reconfigure the surface if lost
                //     Err(wgpu::SurfaceError::Lost) => engine.resize(engine.size),

                //     // if out of memory, we should quit
                //     Err(wgpu::SurfaceError::OutOfMemory) => *flow = ControlFlow::Exit,

                //     // other errors
                //     Err(e) => eprintln!("{:?}", e),
                // }
            },

            Event::LoopDestroyed => app.exit(&mut world),

            _ => warn!("Unhandled global event {:?}", event)
        }
    })
}