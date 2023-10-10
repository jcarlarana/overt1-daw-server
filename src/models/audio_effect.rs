use crate::models::equalizer_settings::{EqualizerSettings,ReverbSettings,LowPassFilterSettings};

pub enum AudioEffect {
    Equalization(EqualizerSettings),
    Reverb(ReverbSettings),
    LowPassFilter(LowPassFilterSettings),
    // Add more effects as needed
}

