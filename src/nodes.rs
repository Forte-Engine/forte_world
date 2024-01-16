use std::{fmt::Debug, marker::PhantomData};

use forte_engine::{math::transforms::Transform, EngineApp};

use crate::dimensions::Dimensions;

pub trait ComponentsDef<A: EngineApp>: Debug + Default {
    fn added(&mut self, app: &mut A, node: &mut Node<Self, A>);
    fn update(&mut self, app: &mut A, node: &mut Node<Self, A>);
    fn remove(&mut self, app: &mut A, node: &mut Node<Self, A>);
}

pub struct Node<C: ComponentsDef<A>, A: EngineApp> {
    pub transform: Transform,
    pub global_transform: Transform,
    pub dimensions: Dimensions,
    pub component: C,
    pub children: Vec<Node<C, A>>,
    phantom: PhantomData<A>
}

impl <C: ComponentsDef<A>, A: EngineApp> Default for Node<C, A> {
    fn default() -> Self {
        Self {
            transform: Transform::default(),
            global_transform: Transform::default(),
            dimensions: Dimensions::default(),
            component: C::default(),
            children: Vec::new(),
            phantom: PhantomData::default()
        }
    }
}
