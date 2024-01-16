pub mod dimensions;
pub mod nodes;

/// Generates a component definition with its ComponentDef supporting functions and render functions.
/// 
/// Example:
/// ```rust 
/// define_components!(
/// Components,
/// TestApp,
/// DrawNodes,
/// draw_node,
/// [
///     CubeModel => {
///         DATA => CubeModel,
///         ADDED => |_: &mut TestApp, _: &mut Node<Components, TestApp>| { println!("Added"); },
///         UPDATE => |_: &mut TestApp, _: &mut Node<Components, TestApp>| { println!("Updated"); },
///         RENDER => |pass: &mut wgpu::RenderPass<'a>, app: &'b TestApp, data: &'b CubeModel| {
///             pass.prepare_cube_engine(&app.cube_engine, &app.camera);
///             pass.draw_cube_model(&app.render_engine, &app.cube_engine, data);
///         },
///         REMOVED => |_: &mut TestApp, _: &mut Node<Components, TestApp>| { println!("Removed"); }
///     }
/// ]
/// );
/// ```
#[macro_export]
macro_rules! define_components {
    (
        $comp_name:ident,
        $app:ident,
        $draw_name:ident,
        $draw_fn_name:ident,
        [$(
            $variant:ident => {
                DATA => $data:ty,
                ADDED => $added:expr,
                UPDATE => $update:expr,
                RENDER => $render:expr,
                REMOVED => $removed:expr
            }
        ),*]
    ) => {
        // Create full enum
        #[derive(Default, Debug)]
        pub enum $comp_name {
            #[default]
            Empty,
            $($variant($data),)*
        }

        // match added, update, and removed functions
        impl forte_world::nodes::ComponentsDef<$app> for $comp_name {
            fn added(&mut self, app: &mut $app, node: &mut Node<$comp_name, $app>) {
                match self {
                    $comp_name::Empty => {},
                    $($comp_name::$variant(data) => { $added(app, node) },)*
                }
            }

            fn update(&mut self, app: &mut $app, node: &mut Node<$comp_name, $app>) {
                match self {
                    $comp_name::Empty => {},
                    $($comp_name::$variant(data) => { $update(app, node) },)*
                }
            }

            fn remove(&mut self, app: &mut $app, node: &mut Node<$comp_name, $app>) {
                match self {
                    $comp_name::Empty => {},
                    $($comp_name::$variant(data) => { $removed(app, node) },)*
                }
            }
        }
    
        // create render trait
        pub trait $draw_name <'a,'b> where 'b: 'a {
            fn $draw_fn_name(
                &mut self,
                app: &'b $app,
                node: &'b Node<$comp_name, $app>
            );
        }

        // draw trait for render pass
        impl<'a, 'b> $draw_name <'a, 'b> for wgpu::RenderPass<'a> where 'b: 'a {
            fn $draw_fn_name(
                &mut self,
                app: &'b $app,
                node: &'b Node<$comp_name, $app>
            ) {
                match &node.component {
                    $comp_name::Empty => {},
                    $($comp_name::$variant(data) => { $render(self, app, data) },)*
                }

                node.children.iter().for_each(|child| self.$draw_fn_name(app, child));
            }
        }
    };
}
