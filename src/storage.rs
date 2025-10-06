use crate::stored_octree::StoredOctree;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::Write;

/// Save to a string.
#[derive(Serialize, Deserialize)]
struct UserRef {
    id: u32,
    name: String,
}

/// We don't use this struct.
pub struct Storage {
    _noop: f32,
}

impl Storage {
    /// Create a new storage.
    pub fn new() -> Storage {
        Storage { _noop: 0.0 }
    }

    /// Save a scene (later in a different thread)
    pub async fn save(self, data: StoredOctree) {
        let json_string =
            serde_json::to_string_pretty(&data).expect("Failed to serialize the octree");

        // Create and write to the file
        let mut file = File::create("output.json").expect("Failed to create file");
        file.write_all(json_string.as_bytes())
            .expect("Failed to write to file");
    }

    /// Delete a scene.
    pub async fn delete_scene(self, _name: String) {
        fs::remove_file("output.json").expect("Was not deleted");
    }

    /// Load a scene.
    pub async fn load_scene(self, _name: String) -> Option<StoredOctree> {
        let file = File::open("output.json").expect("File did not exist");
        let reader = BufReader::new(file);

        // Deserialize the JSON contents of the file into a MyData struct
        let from_disk: StoredOctree = serde_json::from_reader(reader).expect("Failed to read json");
        Some(from_disk)
    }

    /// Load the default scene.
    pub async fn load_first_scene(self) -> Option<StoredOctree> {
        self.load_scene("Default".to_string()).await
    }

    /// Get a list of saved scenes.
    pub async fn list_scenes(self) -> Vec<String> {
        let mut names: Vec<String> = vec![];
        names.push(String::from("Default"));

        names.clone()
    }
}
