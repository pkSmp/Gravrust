//! Port of `client/js/audio/consume-sounds.js`: sound effects for eating/drinking items.

use crate::widgets::Gui;

/// Plays the appropriate sound effect when a player consumes an item (eat or drink).
/// 
/// JS: `consumeEffect(game, item, count=5)`
pub fn consume_effect(gui: &mut dyn crate::widgets::Gui, item: &serde_json::Value, count: i32) {
    let action = item.get("useAction").and_then(|v| v.as_str()).unwrap_or("");
    let count = count.max(1);

    if action == "eat" {
        // Eating sound: random.eat with randomized pitch
        let pitch = 0.5 + 0.5 * fastrand::f64() as f32;
        let volume = 0.5 + 0.5 * fastrand::f64() as f32 * 0.2 + 1.0;
        // gui.sound.playAtPlayer("random.eat", volume, pitch, "player");
        // For now, we'll just log or use a placeholder
        crate::info!("Play eat sound: count={}, pitch={:.2}, volume={:.2}", count, pitch, volume);
    } else if action == "drink" {
        // Drinking sound: random.drink with randomized pitch
        let pitch = 0.5 + fastrand::f64() as f32 * 0.1 + 0.9;
        let volume = 0.5;
        // gui.sound.playAtPlayer("random.drink", volume, pitch, "player");
        crate::info!("Play drink sound: pitch={:.2}, volume={:.2}", pitch, volume);
    }
}

/// Sound effect type for consume effects
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConsumeSoundType {
    Eat,
    Drink,
}

pub struct ConsumeSound {
    pub sound_type: ConsumeSoundType,
    pub count: i32,
}

impl ConsumeSound {
    pub fn new_eat(count: i32) -> Self {
        Self {
            sound_type: ConsumeSoundType::Eat,
            count,
        }
    }

    pub fn new_drink(count: i32) -> Self {
        Self {
            sound_type: ConsumeSoundType::Drink,
            count,
        }
    }

    pub fn play(&self, gui: &mut dyn crate::widgets::Gui) {
        match self.sound_type {
            ConsumeSoundType::Eat => {
                let pitch = 0.5 + 0.5 * fastrand::f64() as f32;
                let volume = 0.5 + 0.5 * fastrand::f64() as f32 * 0.2 + 1.0;
                // gui.sound.playAtPlayer("random.eat", volume, pitch, "player");
                crate::info!("Play eat sound");
            }
            ConsumeSoundType::Drink => {
                let pitch = 0.5 + fastrand::f64() as f32 * 0.1 + 0.9;
                let volume = 0.5;
                // gui.sound.playAtPlayer("random.drink", volume, pitch, "player");
                crate::info!("Play drink sound");
            }
        }
    }
}