use amethyst::{
    prelude::*,
    core::transform::Transform,
    renderer::{Camera, Projection},
};



pub fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_z(1.0);
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            1024.,
            0.0,
            1024.,
        )))
        .with(transform)
        .build();
}
