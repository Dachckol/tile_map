use amethyst::{
    ecs::{
        Component,
        DenseVecStorage,
    },
};


pub struct Controllable;


impl Component for Controllable {
    type Storage = DenseVecStorage<Self>;
}


impl Controllable {
}
