use bevy::prelude::*;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

mod emitter;
use emitter::Emitter;

#[derive(Resource)]
pub struct RayMapData {
    file_version: i8,
    entities: Vec<(i8, Vec3)>, // Vector of tuples containing entity type and positions
    map_data: Vec<(i8, Vec3)>, // Vector of tuples containing object type and position
}

impl Default for RayMapData {
    fn default() -> Self {
        Self {
            file_version: 1,
            entities: Vec::new(), // Initialize with empty vectors
            map_data: Vec::new(), // Initialize with default Vec3 value
        }
    }
}

enum LineType {
    Version,
    Mark,
    Data,
    Empty,
}

#[derive(Resource)]
enum PlacingMode {
    Entities,
    MapData,
}

impl Default for PlacingMode {
    fn default() -> Self {
        PlacingMode::Entities
    }
}

fn determine_line_type(line: &str) -> LineType {
    if line.contains("#") {
        LineType::Version
    } else if line.contains("[") {
        LineType::Mark
    } else if line.contains(",") {
        LineType::Data
    } else {
        LineType::Empty
    }
}

pub fn read_map(map_path: &str) -> RayMapData {
    if let Ok(lines) = read_lines(map_path) {
        let mut map_data = RayMapData::default();
        let mut placing_mode = PlacingMode::default();

        for line in lines {
            if let Ok(data) = line {
                let line_type = determine_line_type(&data);
                match line_type {
                    LineType::Version => handle_version_line(&data, &mut map_data),
                    LineType::Mark => handle_mark_line(&data, &mut placing_mode),
                    LineType::Data => handle_data_line(&data, &mut map_data, &placing_mode),
                    LineType::Empty => continue,
                }
            }
        }
        return map_data;
    } else {
        panic!("Failed to read map file");
    }
}

fn handle_version_line(data: &str, map_data: &mut RayMapData) {
    let values: Vec<&str> = data.split(' ').collect();
    map_data.file_version = values[1].parse().unwrap();
}

fn handle_mark_line(data: &str, placing_mode: &mut PlacingMode) {
    if data.contains("entities") {
        *placing_mode = PlacingMode::Entities;
    } else if data.contains("mapdata") {
        *placing_mode = PlacingMode::MapData;
    } else {
        panic!("Placing Mode should be assigned");
    }
}

fn handle_data_line(data: &str, map_data: &mut RayMapData, placing_mode: &PlacingMode) {
    if let PlacingMode::Entities = placing_mode {
        let parts: Vec<&str> = data.split(',').collect();
        if parts.len() == 4 {
            let entity_type = parts[0].parse().unwrap();
            let x: f32 = parts[1].parse().unwrap();
            let y: f32 = parts[2].parse().unwrap();
            let z: f32 = parts[3].parse().unwrap();

            // Push the new entity into the vector
            map_data.entities.push((entity_type, Vec3::new(x, y, z)));
        } else {
            panic!("Data line for entity is malformed");
        }
    } else if let PlacingMode::MapData = placing_mode {
        let parts: Vec<&str> = data.split(',').collect();
        if parts.len() == 4 {
            let object_type = parts[0].parse().unwrap();
            let x: f32 = parts[1].parse().unwrap();
            let y: f32 = parts[2].parse().unwrap();
            let z: f32 = parts[3].parse().unwrap();

            // Push the new object into the vector
            map_data.map_data.push((object_type, Vec3::new(x, y, z)));
        } else {
            panic!("Data line for map data is malformed");
        }
    }
}

pub struct MapBuilder<'a> {
    map_data: &'a RayMapData,
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

impl<'a> MapBuilder<'a> {
    pub fn new(map_data: &'a RayMapData) -> Self {
        Self { map_data }
    }

    fn build_entities(&self, commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<StandardMaterial>>) {
        let mesh = Cuboid::new(1.0, 1.0, 1.0);
        let material = Color::SEA_GREEN;

        for &(_, position) in self.map_data.entities.iter() {

            
            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(mesh),
                    material: materials.add(material),
                    transform: Transform::from_translation(position),
                    ..Default::default()
                },
                Emitter {angle: 0., position:  Transform::from_translation(position), mesh: mesh.mesh(), material: material},
        ));
        }
    }

    pub fn build(&self, commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<StandardMaterial>>) {
        self.build_entities(commands, meshes, materials);
        //self.build_map_objects(commands, meshes);
    }
}
