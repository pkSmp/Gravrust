//! Port of `client/js/audio/sound-gain.js`: `sound_gain`.
//!
//! Calculates sound gain with 1.8.9 semantics: instance volume is clamped first,
//! then category and listener/master gains are applied.

/// Calculate sound gain with 1.8.9 semantics.
///
/// - Instance volume is clamped to [0, 1] first
/// - Category volume is applied (1.0 for 'master', else from settings.soundVolumes)
/// - Master volume is applied last
pub fn sound_gain(volume: f64, category: &str, settings: &SoundSettings) -> f64 {
    let level = if category == "master" {
        1.0
    } else {
        settings.sound_volumes.get(category).copied().unwrap_or(1.0)
    };

    let clamped_volume = volume.clamp(0.0, 1.0);
    clamped_volume * level * settings.master_volume
}

/// Sound settings structure matching JS `settings` object.
#[derive(Debug, Clone, Default)]
pub struct SoundSettings {
    pub sound_volumes: std::collections::HashMap<String, f64>,
    pub master_volume: f64,
}

impl SoundSettings {
    /// Create default sound settings.
    pub fn new() -> Self {
        Self {
            sound_volumes: std::collections::HashMap::new(),
            master_volume: 1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sound_gain_master_category() {
        let settings = SoundSettings {
            master_volume: 1.0,
            ..Default::default()
        };
        assert_eq!(sound_gain(0.5, "master", &settings), 0.5);
    }

    #[test]
    fn sound_gain_clamped() {
        let settings = SoundSettings::new();
        // Volume > 1 should be clamped
        assert_eq!(sound_gain(2.0, "music", &SoundSettings::new()), 1.0);
        // Volume < 0 should be clamped
        assert_eq!(sound_gain(-1.0, "music", &SoundSettings::new()), 0.0);
    }

    #[test]
    fn sound_gain_category_volume() {
        let mut settings = SoundSettings::new();
        settings.sound_volumes.insert("music".to_string(), 0.5);
        settings.master_volume = 1.0;
        // 1.0 * 0.5 * 1.0 = 0.5
        assert_eq!(sound_gain(1.0, "music", &settings), 0.5);
    }

    #[test]
    fn sound_gain_master_volume() {
        let settings = SoundSettings {
            master_volume: 0.5,
            ..Default::default()
        };
        // 1.0 * 1.0 * 0.5 = 0.5
        assert_eq!(sound_gain(1.0, "master", &settings), 0.5);
    }
}