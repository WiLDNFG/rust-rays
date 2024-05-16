use bevy::prelude::*;

#[derive(Component)]
pub struct Emitter {
    pub angle: f64,
    pub position: Transform,    
}

pub struct EmitterPlugin;

impl Plugin for EmitterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_emitter));
        app.add_systems(Startup, (spawn_ray));
        app.add_systems(Update, (handle_rotation));
        app.add_systems(Update, (handle_ray));
    }
}

fn spawn_emitter(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            material: materials.add(Color::SEA_GREEN),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        Name::new("Emitter"),
    ));
}

fn spawn_ray(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    
}
fn handle_ray(mut commands: Commands, asset_server: Res<AssetServer>) {}

fn handle_rotation(){}