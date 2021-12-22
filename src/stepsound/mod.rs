//! Sounds played upon stepping on a block.

use std::convert::TryFrom;

use crate::common::ID;

/// Represents a step sound produced by an entity.
pub struct StepSoundMeta {
    volume: f32,
    pitch: f32,
    id: StepSound,
    sound: &'static str,
}

/// The sound played by stepping on the respective material.
#[repr(usize)]
#[derive(Copy, Clone, Debug)]
pub enum StepSound {
    Powder,
    Wood,
    Gravel,
    Grass,
    Stone,
    Metal,
    Glass,
    Cloth,
    Sand,
}

impl StepSoundMeta {
    const fn new(id: StepSound, sound: &'static str, volume: f32, pitch: f32) -> Self {
        Self { id, sound, volume, pitch }
    }

    /// Returns the id of the step sound.
    pub fn id(&self) -> StepSound {
        self.id
    }

    /// The name of the sound resource.
    pub fn sound(&self) -> &'static str {
        self.sound
    }

    /// The volume of the sound.
    pub fn volume(&self) -> f32 {
        self.volume
    }

    /// The pitch of the sound.
    pub fn pitch(&self) -> f32 {
        self.pitch
    }
}

impl TryFrom<ID> for StepSound {
    type Error = &'static str;

    fn try_from(id: ID) -> Result<Self, Self::Error> {
        match id {
            0 => Ok(StepSound::Powder),
            1 => Ok(StepSound::Wood),
            2 => Ok(StepSound::Gravel),
            3 => Ok(StepSound::Grass),
            4 => Ok(StepSound::Stone),
            5 => Ok(StepSound::Metal),
            6 => Ok(StepSound::Glass),
            7 => Ok(StepSound::Cloth),
            8 => Ok(StepSound::Sand),
            _ => Err("invalid id for step sound"),
        }
    }
}

impl StepSound {
    /// Returns the metadata of a step sound.
    pub fn get(self) -> &'static StepSoundMeta {
        &LIST[self as ID]
    }
}

static LIST: [StepSoundMeta; 9] = [
    StepSoundMeta::new(StepSound::Powder, "stone", 1.0, 1.0),
    StepSoundMeta::new(StepSound::Wood, "wood", 1.0, 1.0),
    StepSoundMeta::new(StepSound::Gravel, "gravel", 1.0, 1.0),
    StepSoundMeta::new(StepSound::Grass, "grass", 1.0, 1.0),
    StepSoundMeta::new(StepSound::Stone, "stone", 1.0, 1.0),
    StepSoundMeta::new(StepSound::Metal, "stone", 1.0, 1.5),
    StepSoundMeta::new(StepSound::Glass, "stone", 1.0, 1.0),
    StepSoundMeta::new(StepSound::Cloth, "cloth", 1.0, 1.0),
    StepSoundMeta::new(StepSound::Sand, "sand", 1.0, 1.0),
];

#[cfg(test)]
mod tests {
    use super::StepSound;
    use std::convert::TryFrom;

    #[test]
    fn test_get() {
        let stone = StepSound::Stone.get();

        assert_eq!(stone.sound(), "stone");
        assert_eq!(stone.volume(), 1.0);
        assert_eq!(stone.pitch(), 1.0);
    }

    #[test]
    fn test_from() {
        let stone = StepSound::try_from(4)
            .unwrap()
            .get();

        assert_eq!(stone.sound(), "stone");
        assert_eq!(stone.volume(), 1.0);
        assert_eq!(stone.pitch(), 1.0);
    }
}
