use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};
use hexx::{shapes, *};

/// World size of the hexagons (outer radius)
const HEX_SIZE: Vec2 = Vec2::splat(8.0);
const CHUNK_SIZE: u32 = 5;
const COLORS: [Color; 3] = [Color::WHITE, Color::BLUE, Color::RED];

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1_000.0, 1_000.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (setup_camera, setup_grid))
        .run();
}

/// 3D Orthogrpahic camera setup
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

/// Hex grid setup
fn setup_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let layout = HexLayout {
        hex_size: HEX_SIZE,
        ..default()
    };
    // materials
    let materials = COLORS.map(|c| materials.add(c.into()));
    // mesh
    let mesh = hexagonal_plane(&layout);
    let mesh_handle = meshes.add(mesh);

    for hex in shapes::flat_rectangle([-40, 40, -35, 35]) {
        let pos = layout.hex_to_world_pos(hex);
        let hex_mod = hex.to_lower_res(CHUNK_SIZE);
        let color_index = (hex_mod.x - hex_mod.y).rem_euclid(3);
        commands.spawn(ColorMesh2dBundle {
            transform: Transform::from_xyz(pos.x, pos.y, 0.0),
            mesh: mesh_handle.clone().into(),
            material: materials[color_index as usize].clone(),
            ..default()
        });
    }
}

/// Compute a bevy mesh from the layout
fn hexagonal_plane(hex_layout: &HexLayout) -> Mesh {
    let mesh_info = PlaneMeshBuilder::new(hex_layout)
        .with_scale(Vec3::splat(0.9))
        .facing(Vec3::Z)
        .build();
    Mesh::new(PrimitiveTopology::TriangleList)
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, mesh_info.vertices)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs)
        .with_indices(Some(Indices::U16(mesh_info.indices)))
}
