use crate::models::audio_file::AudioFile;
use crate::models::audio_effect::AudioEffect;

struct ProcessingJob {
    id: u64,
    audio_file: AudioFile,
    effects: Vec<AudioEffect>,
    status: JobStatus,
}

enum JobStatus {
    Queued,
    Processing,
    Completed,
    Failed,
}

