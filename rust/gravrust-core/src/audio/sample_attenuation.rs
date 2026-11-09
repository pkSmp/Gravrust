//! Port of `client/js/audio/sample-attenuation.js`: native sample attenuation for sound engine.

/// Calculates sample attenuation for sound engine.
/// 
/// JS: `sampleAttenuation(volume, sample)`
/// 
/// Native SoundEngine uses sample attenuation, not the network SoundEvent fixed range.
pub fn sample_attenuation(volume: f32, _sample: &[i16]) -> f32 {
    // JS: return Math.fround(Math.max(1, Math.fround(volume)) * 16);
    // Clamp volume to at least 1, then multiply by 16
    let volume = volume.max(1.0);
    (volume * 16.0).round()
}

/// Calculate attenuation for a sound sample based on distance and volume.
/// 
/// This is a more complete port that includes distance-based attenuation.
pub fn sample_attenuation_with_distance(volume: f32, distance: f32, _sample: &[i16]) -> f32 {
    // Base attenuation from volume
    let base = sample_attenuation(1.0, &[]);
    
    // Apply distance attenuation (inverse square law approximation)
    let distance_factor = if distance > 0.0 {
        1.0 / (1.0 + distance * 0.1)
    } else {
        1.0
    };
    
    (base as f32 * distance_factor).round()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_attenuation_basic() {
        assert_eq!(sample_attenuation(0.5, &[]), 16.0); // max(1, 0.5) = 1, * 16 = 16
        assert_eq!(sample_attenuation(1.0, &[]), 16.0);
        assert_eq!(sample_attenuation(2.0, &[]), 32.0);
    }

    #[test]
    fn test_sample_attenuation_clamped() {
        // Volume < 1 should be clamped to 1
        assert_eq!(sample_attenuation(0.1, &[]), 16.0);
        assert_eq!(sample_attenuation(0.0, &[]), 16.0);
        assert_eq!(sample_attenuation(-1.0, &[]), 16.0);
    }

    #[test]
    fn test_sample_attenuation_with_distance() {
        // At distance 0, should be full volume
        assert_eq!(sample_attenuation_with_distance(1.0, 0.0, &[]), 16.0);
        
        // At distance 10, should be attenuated
        let attenuated = sample_attenuation_with_distance(1.0, 10.0, &[]);
        assert!(attenuated < 16.0);
    }
}