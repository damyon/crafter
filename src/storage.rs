use crate::stored_octree::StoredOctree;
use serde::{Deserialize, Serialize};
use serde_json;
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
    path: String,
}

impl Storage {
    /// Create a new storage.
    pub fn new(path: &str) -> Storage {
        Storage {
            path: path.to_string(),
        }
    }

    /// Save a scene (later in a different thread)
    pub fn save(self, data: StoredOctree) {
        let json_string =
            serde_json::to_string_pretty(&data).expect("Failed to serialize the octree");

        // Create and write to the file
        let mut file = File::create(self.path).expect("Failed to create file");
        file.write_all(json_string.as_bytes())
            .expect("Failed to write to file");
    }

    /// Load a scene.
    pub fn load_scene(self) -> Option<StoredOctree> {
        let file = File::open(self.path.as_str()).expect("File did not exist");
        let reader = BufReader::new(file);

        println!("Read scene from file: {}", self.path);
        // Deserialize the JSON contents of the file into a MyData struct
        let from_disk: StoredOctree = serde_json::from_reader(reader).expect("Failed to read json");

        Some(from_disk)
    }

    /// Load the default scene.
    pub fn load_first_scene(self) -> Option<StoredOctree> {
        self.load_scene()
    }

    /// Get a list of saved scenes.
    pub async fn list_scenes(self) -> Vec<String> {
        let mut names: Vec<String> = vec![];
        names.push(String::from("Default"));

        names.clone()
    }
}
