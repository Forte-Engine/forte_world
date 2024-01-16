pub mod dimensions;

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
macro_rules! define_world {
    (
        $app:ident,
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
        use std::marker::PhantomData;
        use cgmath::Vector3;
        use forte_engine::math::transforms::Transform;
        use forte_world::dimensions::Dimensions;

        // Create full enum
        #[derive(Default, Debug)]
        pub enum Component {
            #[default]
            Empty,
            $($variant($data),)*
        }

        // create node
        #[derive(Debug)]
        pub struct Node {
            // public
            pub transform: Transform,
            pub component: Component,
            pub rel_min_dimensions: Dimensions,
        
            // non-public
            global_transform: Transform,
            dimensions: Dimensions,
            children: Vec<Node>
        }

        // Give node a default
        impl Default for Node {
            fn default() -> Self {
                Self {
                    transform: Transform::default(),
                    global_transform: Transform::default(),
                    rel_min_dimensions: Dimensions::default(),
                    dimensions: Dimensions::default(),
                    component: Component::default(),
                    children: Vec::new()
                }
            }
        }

        // create node functions
        impl Node {
            // accessor functions
            pub fn global_transform(&self) -> &Transform { &self.transform }
            pub fn rel_min_dimensions(&self) -> &Dimensions { &self.rel_min_dimensions }
            pub fn dimensions(&self) -> &Dimensions { &self.dimensions }
            pub fn children(&self) -> &Vec<Node> { &self.children }

            // modification functions
            pub fn add_child(&mut self, mut child: Node) {
                self.children.push(child);
                self.children.last_mut().as_mut().unwrap().call_add_recr();
            }

            pub fn remove_child(&mut self, idx: usize) {
                self.children[idx].call_remove_recr();
                self.children.remove(idx);
            }

            pub fn update(&mut self, previous: &Transform) {
                // calculate new global transform
                let global_transform = Transform {
                    position: self.transform.position + previous.position,
                    rotation: previous.rotation * self.transform.rotation,
                    scale: self.transform.scale.mul_element_wise(previous.scale)
                };

                // calculate starting dimensions
                let mut dimensions = Dimensions {
                    from: Vector3 {
                        x: global_transform.position.x + self.rel_min_dimensions.from.x,
                        y: global_transform.position.y + self.rel_min_dimensions.from.y,
                        z: global_transform.position.z + self.rel_min_dimensions.from.z,
                    },
                    to: Vector3 {
                        x: global_transform.position.x + self.rel_min_dimensions.to.x,
                        y: global_transform.position.y + self.rel_min_dimensions.to.y,
                        z: global_transform.position.z + self.rel_min_dimensions.to.z,
                    }
                };

                // update children first, and update dimensions if/when necessary
                self.children.iter_mut().for_each(|child| {
                    child.update(&global_transform);

                    // check for dimension updates
                    if child.dimensions.from.x < dimensions.from.x { dimensions.from.x = child.dimensions.from.x; }
                    if child.dimensions.from.y < dimensions.from.y { dimensions.from.y = child.dimensions.from.y; }
                    if child.dimensions.from.z < dimensions.from.z { dimensions.from.z = child.dimensions.from.z; }
                    if child.dimensions.to.x > dimensions.to.x { dimensions.to.x = child.dimensions.to.x; }
                    if child.dimensions.to.y > dimensions.to.y { dimensions.to.y = child.dimensions.to.y; }
                    if child.dimensions.to.z > dimensions.to.z { dimensions.to.z = child.dimensions.to.z; }
                });

                // update global transform and dimensions
                self.global_transform = global_transform;
                self.dimensions = dimensions;

                // call component update
                match &self.component {
                    Component::Empty => {},
                    $(Component::$variant(data) => { $update(self) },)*
                }
            }

            // calls the add functions recursively for this node and all its children
            pub(crate) fn call_add_recr(&mut self) {
                match &self.component {
                    Component::Empty => {},
                    $(Component::$variant(data) => { $added(self) },)*
                }

                self.children.iter_mut().for_each(|child| child.call_add_recr());
            }

            // calls the remove functions recursively for this node and all its children
            pub(crate) fn call_remove_recr(&mut self) {
                match &self.component {
                    Component::Empty => {},
                    $(Component::$variant(data) => { $removed(self) },)*
                }

                self.children.iter_mut().for_each(|child| child.call_remove_recr());
            }
        }
    
        // create render trait
        pub trait DrawNodes <'a,'b> where 'b: 'a {
            fn draw_node(
                &mut self,
                app: &'b $app,
                node: &'b Node
            );
        }

        // draw trait for render pass
        impl<'a, 'b> DrawNodes <'a, 'b> for wgpu::RenderPass<'a> where 'b: 'a {
            fn draw_node(
                &mut self,
                app: &'b $app,
                node: &'b Node
            ) {
                match &node.component {
                    Component::Empty => {},
                    $(Component::$variant(data) => { $render(self, app, data) },)*
                }

                node.children().iter().for_each(|child| self.draw_node(app, child));
            }
        }
    };
}
