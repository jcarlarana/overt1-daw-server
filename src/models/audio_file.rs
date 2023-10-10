use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum AudioFormat {
    Mp3,
    Wav,
    // add more formats as needed
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AudioFile {
    pub id: Uuid,  // Unique identifier for the file
    pub format: AudioFormat,  // Format of the audio file
    pub location: PathBuf,  // Location of the file in the file system or object storage
    pub sample_rate: u32,  // Sample rate of the audio file
    pub channels: u16,  // Number of channels in the audio file
    pub bit_depth: u16,  // Bit depth of the audio file
    // Add more metadata as needed
}

impl AudioFile {
    // Create a new AudioFile instance
    pub fn new(id: Uuid, format: AudioFormat, location: PathBuf, sample_rate: u32, channels: u16, bit_depth: u16) -> Self {
        AudioFile {
            id,
            format,
            location,
            sample_rate,
            channels,
            bit_depth,
        }
    }

    // Additional methods to manipulate and retrieve information about the audio file
    // For example: play, stop, pause, etc.
}

// Implement more functionality as needed based on your requirements

