use geometry;
use js;
use map;
use na;
use nc::shape::{Cuboid, ShapeHandle, TriMesh};
use np::{
    force_generator::{ConstantAcceleration, ForceGeneratorHandle},
    object::{BodyHandle, Material},
    volumetric::Volumetric,
    world::World,
};
use std::{f32::consts::FRAC_PI_2, intrinsics, sync::Mutex};

pub const COLLIDER_MARGIN: f32 = 0.01;
/// In m/s^2, as defined by la Conférence générale des poids et mesures
pub const STANDARD_GRAVITY: f32 = 9.806_65;
pub const CONTROL_FORCE: f32 = 20.0;

lazy_static! {
    pub static ref WORLD: Mutex<World<f32>> =
        Mutex::new(World::new(time_in_sec));
    pub static ref PLAYER: Mutex<BodyHandle> =
        Mutex::new(BodyHandle::ground());
    pub static ref CONTROL_ACC: Mutex<(na::Vector3<f32>, ForceGeneratorHandle)> =
        Mutex::new((na::Vector3::zeros(), 0));
}

pub fn init_world(map_data: &map::Map, player_pos: &na::Point3<f32>) {
    let mut world = WORLD.lock().unwrap();

    world.set_gravity(na::Vector3::y() * -STANDARD_GRAVITY);

    let cuboid = ShapeHandle::new(Cuboid::new(na::Vector3::repeat(1.0)));
    let cuboid_inertia = cuboid.inertia(1.0);
    let cuboid_center_of_mass = cuboid.center_of_mass();
    let player_body_handle = world.add_rigid_body(
        na::Isometry3::new(player_pos.coords, na::Vector3::zeros()),
        cuboid_inertia,
        cuboid_center_of_mass,
    );

    *PLAYER.lock().unwrap() = player_body_handle;

    let _player_collider = world.add_collider(
        COLLIDER_MARGIN,
        ShapeHandle::new(Cuboid::new(na::Vector3::repeat(
            1.0 - COLLIDER_MARGIN,
        ))),
        player_body_handle,
        na::Isometry3::identity(),
        Material::default(),
    );

    let hex_prism_verts = geometry::HEXAGONAL_PRISM_VERTS
        .exact_chunks(3)
        .map(|s| na::Point3::new(s[0], s[1], s[2]))
        .collect::<Vec<_>>();
    let hex_prism_indices = geometry::HEXAGONAL_PRISM_INDICES
        .exact_chunks(3)
        .map(|s| na::Point3::new(s[0], s[1], s[2]))
        .collect::<Vec<_>>();

    for (hex, (x, y)) in map_data.iter() {
        world.add_collider(
            COLLIDER_MARGIN,
            ShapeHandle::new(TriMesh::new(
                hex_prism_verts.clone(),
                hex_prism_indices.clone(),
                None,
            )),
            BodyHandle::ground(),
            na::Isometry3::new(
                na::Vector3::new(*x, hex.height, -(*y)),
                na::Vector3::new(-FRAC_PI_2, 0.0, 0.0),
            ),
            Material::default(),
        );
    }

    // Initial control accelerator
    let mut ctrl_force =
        ConstantAcceleration::new(na::Vector3::zeros(), na::Vector3::zeros());

    ctrl_force.add_body_part(player_body_handle);
    let ctrl_force_handle = world.add_force_generator(ctrl_force);

    set_control_acc(na::Vector3::zeros(), ctrl_force_handle);
}

#[inline]
pub fn step() {
    WORLD.lock().unwrap().step();
}

#[inline]
fn time_in_sec() -> f64 {
    unsafe { intrinsics::fdiv_fast(js::now(), 1000.0) }
}

pub fn update_control_acc(lin_acc: na::Vector3<f32>) {
    let (curr_lin_acc, curr_handle) = get_control_acc();

    js::log(&format!("{:?}, {:?}", curr_lin_acc, lin_acc));

    if curr_lin_acc != lin_acc {
        let mut world = WORLD.lock().unwrap();
        world.remove_force_generator(curr_handle);

        let mut ctrl_force =
            ConstantAcceleration::new(lin_acc, na::Vector3::zeros());
        ctrl_force.add_body_part(*PLAYER.lock().unwrap());
        let ctrl_force_handle = world.add_force_generator(ctrl_force);

        js::log(&format!("ctrl_force_handle: {}", ctrl_force_handle));

        set_control_acc(lin_acc, ctrl_force_handle);
    }
}

#[inline]
fn set_control_acc(lin_acc: na::Vector3<f32>, handle: ForceGeneratorHandle) {
    *CONTROL_ACC.lock().unwrap() = (lin_acc, handle);
}

#[inline]
fn get_control_acc() -> (na::Vector3<f32>, ForceGeneratorHandle) {
    *CONTROL_ACC.lock().unwrap()
}
