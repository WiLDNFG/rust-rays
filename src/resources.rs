use bevy::{prelude::*, transform::commands};

#[derive(Resource, Default)]
struct GameProgress {
    game_completed: bool,
    secrets_unlocked: u32,
}

#[derive(Resource)]
struct StartingLevel(usize);

impl Default for StartingLevel {
    fn default() -> Self {
        StartingLevel(1)
    }
}

#[derive(Resource, Default)]
enum GameMode {
    Tutorial,
    Ingame,
    Paused,
    #[default]
    Menu,
}

//Will be read through the map format using #v tag
#[derive(Resource)]
struct MapVersion { 
    file_version: i8,
}