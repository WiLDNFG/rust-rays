use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use bevy::prelude::*;

struct MapData {
    file_version: i8,
    entitiesmd:[],
    
}

enum LineType {
    Version,
    Mark,
    Data,
    Empty,
}

fn determine_line_type(line: &str) -> LineType {
    if line.contains("#") {
        return LineType::Version;
    } else if line.contains("[") {
        return LineType::Mark;
    } else if line.contains(",") {
        return LineType::Data;
    } else {
        return LineType::Empty;
    }
}
fn read_map(mut commands: Commands, map_path: &str) -> MapData {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines(map_path) {
        let map_data: MapData;
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(data) = line {
                //Check type of line
                let line_type = determine_line_type(&data);
                match line_type {
                    LineType::Version => handle_version_line(data, &mut commands),
                    LineType::Mark => handle_mark_line(data),
                    LineType::Data => handle_data_line(data),
                    LineType::Empty => continue,
                }

            }
        }
    }
}

fn handle_version_line(data: String, commands: &mut Commands){
//    let values: Vec<&str> = data.split(' ').collect();
//    commands.insert_resource(MapVersion {file_version: values[1].parse::<i8>()})
}

fn handle_mark_line(line: String){
    
}

fn handle_data_line(line: String){
    
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
