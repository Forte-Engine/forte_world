use forte_cubes::models::{CubeEngine, cubes::CubeModel, file::SBFile, DrawCubes};
use forte_engine::{render::{render_engine::RenderEngine, primitives::cameras::{Camera, CameraController}, render_utils}, lights::{LightEngine, SetupLights}, EngineApp, run_app};
use forte_world::{nodes::*, define_components};

define_components!(
    Components,
    TestApp,
    DrawNodes,
    draw_node,
    [
        CubeModel => {
            DATA => CubeModel,
            ADDED => |_: &mut TestApp, _: &mut Node<Components, TestApp>| { println!("Added"); },
            UPDATE => |_: &mut TestApp, _: &mut Node<Components, TestApp>| { println!("Updated"); },
            RENDER => |pass: &mut wgpu::RenderPass<'a>, app: &'b TestApp, data: &'b CubeModel| {
                pass.prepare_cube_engine(&app.cube_engine, &app.camera);
                pass.draw_cube_model(&app.render_engine, &app.cube_engine, data);
            },
            REMOVED => |_: &mut TestApp, _: &mut Node<Components, TestApp>| { println!("Removed"); }
        }
    ]
);

pub struct TestApp {
    render_engine: RenderEngine,
    light_engine: LightEngine,
    cube_engine: CubeEngine,
    camera: Camera,
    controller: CameraController,
    root: Node<Components, TestApp>
}

impl EngineApp for TestApp {
    fn create(mut engine: RenderEngine) -> Self {
        // create engines
        let light_engine = LightEngine::new(&engine, [0.1, 0.1, 0.1]);
        let cube_engine = CubeEngine::new(&mut engine);

        // create camera stuffs
        let mut camera = Camera::new(
            &engine, 
            engine.config.width as f32 / engine.config.height as f32,
            45.0, 0.1, 100.0
        );
        camera.position = (0.0, 0.0, 5.0).into();
        camera.update(&mut engine);
        let controller = CameraController::new(0.02);

        // create nodes
        let mut root = Node::default();
        let mut model = Node::default();
        model.component = Components::CubeModel(SBFile::load("assets/warrior.json").as_model(&mut engine));
        root.children.push(model);

        // create final app
        Self {
            render_engine: engine,
            light_engine, cube_engine, root,
            camera, controller
        }
    }


    fn update(&mut self) {
        // start render
        let resources = render_utils::prepare_render(&self.render_engine);
        let mut resources = if resources.is_ok() { resources.unwrap() } else { return };

        {
            // create render pass
            let mut pass = resources.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &resources.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.render_engine.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store
                    }),
                    stencil_ops: None
                }),
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            // setup environment
            pass.load_lights(&self.light_engine);

            // have nodes render to renderables
            pass.draw_node(&self, &self.root);
        }

        // end render
        render_utils::finalize_render(&mut self.render_engine, resources);
    }

    fn input(&mut self, input: forte_engine::render::input::EngineInput) {
        self.controller.input(&input);
        self.controller.update_camera(&mut self.camera);
        self.camera.update(&mut self.render_engine);
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) { self.render_engine.resize(new_size); }
    fn events_cleared(&mut self) { self.render_engine.next_frame(); }
    fn exit(&mut self) {}
}

fn main() { pollster::block_on(run_app::<TestApp>()); }
