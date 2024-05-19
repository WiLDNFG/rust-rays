//use
use bevy::pbr::PbrBundle;
use bevy::prelude::{App, Assets, Commands, Mesh, ResMut};
use bevy::DefaultPlugins;
use bevy::{prelude::*, transform::commands};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

//mod
mod map;
mod resources;

fn main() {
    let add_systems = &mut App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Rays".into(),
                        resolution: (1280., 920.).into(),
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_plugins(WorldInspectorPlugin::default())
        .add_systems(Startup, setup)
        .run();
}

#[derive(Component)]
struct MyCameraMarker;

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_matrix(Mat4::from_rotation_translation(
            Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize(),
            Vec3::new(-7.0, 20.0, 4.0),
        )),
        ..Default::default()
    });
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    //Map
    let map_data = map::read_map("assets/level1.mp");
    let map_builder = map::MapBuilder::new(&map_data);
    map_builder.build(&mut commands, &mut meshes, &mut materials);

    // Light
    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
        ..Default::default()
    });

    setup_camera(commands);
}

fn create_world(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Light
    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
        ..Default::default()
    });

    let mesh = meshes.add(Mesh::from(shape::Plane {
        size: 1.,
        subdivisions: 2,
    }));
    let material = materials.add(Color::rgb(1.0, 0.9, 0.));

    //Spawn 64 Tiles
    for i in 0..16 {
        for j in 0..16 {
            commands.spawn(PbrBundle {
                mesh: mesh.clone(),
                material: material.clone(),
                transform: Transform::from_translation(Vec3::new(i as f32, 0., j as f32)),
                ..Default::default()
            });
        }
    }
}

fn initCamera(commands: Commands) {}
// #[derive(Resource)]
// struct UiFont(Handle<Font>);

// fn load_ui_font(
//     mut commands: Commands,
//     server: Res<AssetServer>){
//     //create handles for assets

// }
