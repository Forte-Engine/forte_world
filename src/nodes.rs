use std::fmt::Debug;

use cgmath::Vector3;
use forte_engine::math::transforms::Transform;

pub trait ComponentsDef: Debug + Default {}

pub struct Node<C: ComponentsDef> {
    pub transform: Transform,
    pub global_transform: Transform,
    pub dimensions: Vector3<f32>,
    pub component: C
}

impl <C: ComponentsDef> Default for Node<C> {
    fn default() -> Self {
        Self {
            transform: Transform::default(),
            global_transform: Transform::default(),
            dimensions: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            component: C::default()
        }
    }
}
