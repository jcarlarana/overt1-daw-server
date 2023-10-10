use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub enum AudioFormat {
    Mp3,
    Wav,
    Flac,
    Other(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AudioFile {
    id: Uuid,
    name: String,
    format: AudioFormat,
    data: Vec<u8>, # This represents the audio data; in a real scenario, you'd likely handle this differently
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    id: Uuid,
    user_id: Uuid,
    title: String,
    description: String,
    audio_files: HashMap<Uuid, AudioFile>,
    # Add other metadata or fields as needed
}

impl Post {
    pub fn new(user_id: Uuid, title: String, description: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            title,
            description,
            audio_files: HashMap::new(),
        }
    }

    pub fn add_audio_file(&mut self, name: String, format: AudioFormat, data: Vec<u8>) {
        let audio_file = AudioFile {
            id: Uuid::new_v4(),
            name,
            format,
            data,
        };
        self.audio_files.insert(audio_file.id, audio_file);
    }

    pub fn get_audio_file(&self, file_id: Uuid) -> Option<&AudioFile> {
        self.audio_files.get(&file_id)
    }

    # More methods for various operations on posts...
}

