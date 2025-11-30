use bevy::prelude::*;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::asset::RenderAssetUsages;
use bevy::window::PrimaryWindow;

const GRID_WIDTH: i32 = 11;
const GRID_HEIGHT: i32 = 11;
const HEX_RADIUS: f32 = 50.0; 

const SQRT_3: f32 = 1.7320508;
const HEX_H_SPACING: f32 = HEX_RADIUS * SQRT_3;
const HEX_V_SPACING: f32 = HEX_RADIUS * 1.5;

#[derive(Component)]
struct HexTile {
    row: i32,
    col: i32
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (handle_click).chain())
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let camera_x = (GRID_WIDTH as f32 * HEX_H_SPACING) / 2.0;
    let camera_y = (GRID_HEIGHT as f32 * HEX_V_SPACING) / 2.0;
    commands.spawn((
        Camera2d,
        Transform::from_xyz(camera_x, camera_y, 0.0),
    ));
    
    let hex_mesh = meshes.add(create_hex_mesh());

    let cx = GRID_WIDTH as f32 / 2.0;
    let cy = GRID_HEIGHT as f32 / 2.0;

    for row in 0..GRID_HEIGHT {
        for col in 0..GRID_WIDTH {
            let mut x = col as f32 * HEX_H_SPACING;
            let y = row as f32 * HEX_V_SPACING;
           
            if row % 2 != 0 {
                x += HEX_H_SPACING / 2.0;
            }

            let diff = ((cx - col as f32).abs() + (cy - row as f32).abs()) / 16.0;

            commands.spawn((
                Mesh2d(hex_mesh.clone()),
                MeshMaterial2d(materials.add(Color::linear_rgb(0.5 - diff, 0.8, 0.5 - diff))),
                Transform::from_xyz(x, y, 0.0),
                HexTile {row, col}
            ));
        }
    }
}


fn create_hex_mesh() -> Mesh {
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default()
    );

    let mut vertices = Vec::<[f32; 3]>::new();
    for i in 0..6 {
        let angle_deg = 60.0 * i as f32 - 30.0;
        let angle_rad = std::f32::consts::PI / 180.0 * angle_deg;
        vertices.push([
            HEX_RADIUS * angle_rad.cos(),
            HEX_RADIUS * angle_rad.sin(),
            0.0
        ]);
    }

    let indices = Indices::U32(vec![
        0, 5, 4,
        0, 4, 3,
        0, 3, 2,
        0, 2, 1
    ]);

    let uvs: Vec<[f32; 2]> = vec![
        [0.75, 0.75], // 0: Top-right
        [1.0, 0.5],  // 1: Right
        [0.75, 0.25], // 2: Bottom-right
        [0.25, 0.25], // 3: Bottom-left
        [0.0, 0.5],  // 4: Left
        [0.25, 0.75], // 5: Top-left
    ];

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(indices);
    
    mesh
}

fn handle_click(
    mouse_pos: Res<ButtonInput<MouseButton>>,
    hex_query: Query<(&Transform, &HexTile)>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
) {
    if mouse_pos.just_released(MouseButton::Left) {
        let Some(click_pos) = cursor_to_world(q_window, q_camera) else { return };

        for (transform, tile) in hex_query.iter() {
            let center = transform.translation.xy();

            if point_in_hex(click_pos, center) {
                println!("Clicked tile ({}, {})", tile.row, tile.col);
            }
        }
    }
}

fn cursor_to_world(
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
) -> Option<Vec2> {
    let (camera, camera_transform) = q_camera.single().ok()?;
    let window = q_window.single().ok()?;

    if let Some(screen_pos) = window.cursor_position() {
        camera.viewport_to_world_2d(camera_transform, screen_pos).ok()
    } else {
        None
    }
}

fn point_in_hex(
    point: Vec2,
    center: Vec2,
) -> bool {
    let p = point - center;

    let half_width = HEX_H_SPACING / 2.0; 
    if p.x.abs() > half_width {
        return false;
    }

    if p.y.abs() > HEX_RADIUS {
        return false;
    }

    if p.x.abs() * (HEX_RADIUS / half_width) + p.y.abs() > HEX_RADIUS {
        return false;
    }

    true
}
