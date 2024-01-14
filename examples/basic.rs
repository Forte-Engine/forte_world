use forte_cubes::models::CubeEngine;
use forte_engine::{render::render_engine::RenderEngine, lights::LightEngine};
use forte_world::{WorldApp, nodes::ComponentsDef, ResourceDef, run_world_app};

#[derive(Debug, Default)]
enum Components {
    #[default]
    Empty,
    CubeModel
}

impl ComponentsDef for Components {}

struct Resources {
    render_engine: RenderEngine,
    light_engine: LightEngine,
    cube_engine: CubeEngine
}

impl ResourceDef for Resources {
    fn create(mut engine: RenderEngine) -> Self {
        let light_engine = LightEngine::new(&engine, [1.0, 1.0, 1.0]);
        let cube_engine = CubeEngine::new(&mut engine);
        Self {
            render_engine: engine,
            light_engine, cube_engine
        }
    }

    fn render_engine(&self) -> &RenderEngine { &self.render_engine }
    fn render_engine_mut(&mut self) -> &mut RenderEngine { &mut self.render_engine }
}

struct TestApp;
impl WorldApp<Components, Resources> for TestApp {
    fn create(world: &mut forte_world::World<Components, Resources>) -> Self { Self }
    fn input(&mut self, world: &mut forte_world::World<Components, Resources>, input: forte_engine::render::render_engine::RenderEngineInput) {}
    fn update(&mut self, world: &mut forte_world::World<Components, Resources>) {}
    fn render(&mut self, world: &mut forte_world::World<Components, Resources>) {}
    fn exit(&mut self, world: &mut forte_world::World<Components, Resources>) {}
}

fn main() { run_world_app::<TestApp, Components, Resources>(); }