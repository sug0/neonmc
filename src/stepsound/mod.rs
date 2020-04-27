use crate::common::ID;

/// Represents a step sound produced by an entity.
pub struct StepSoundMeta {
    volume: f32,
    pitch: f32,
    id: StepSound,
    sound: &'static str,
}

/// The sound played by stepping on the
/// respective material.
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

    /// The pitch of the sound.
    pub fn volume(&self) -> f32 {
        self.volume
    }

    /// The volume of the sound.
    pub fn pitch(&self) -> f32 {
        self.pitch
    }
}

impl From<ID> for StepSound {
    fn from(id: ID) -> Self {
        match id {
            0 => StepSound::Powder,
            1 => StepSound::Wood,
            2 => StepSound::Gravel,
            3 => StepSound::Grass,
            4 => StepSound::Stone,
            5 => StepSound::Metal,
            6 => StepSound::Glass,
            7 => StepSound::Cloth,
            8 => StepSound::Sand,
            _ => StepSound::Stone,
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
    StepSoundMeta::new(StepSound::Powder, "step.stone", 1.0, 1.0),
    StepSoundMeta::new(StepSound::Wood, "step.wood", 1.0, 1.0),
    StepSoundMeta::new(StepSound::Gravel, "step.gravel", 1.0, 1.0),
    StepSoundMeta::new(StepSound::Grass, "step.grass", 1.0, 1.0),
    StepSoundMeta::new(StepSound::Stone, "step.stone", 1.0, 1.0),
    StepSoundMeta::new(StepSound::Metal, "step.stone", 1.0, 1.5),
    StepSoundMeta::new(StepSound::Glass, "step.stone", 1.0, 1.0),
    StepSoundMeta::new(StepSound::Cloth, "step.cloth", 1.0, 1.0),
    StepSoundMeta::new(StepSound::Sand, "step.sand", 1.0, 1.0),
];

#[cfg(test)]
mod tests {
    use super::StepSound;

    #[test]
    fn test_get() {
        let stone = StepSound::Stone.get();

        assert_eq!(stone.sound(), "step.stone");
        assert_eq!(stone.volume(), 1.0);
        assert_eq!(stone.pitch(), 1.0);
    }

    #[test]
    fn test_from() {
        let stone = StepSound::from(4).get();

        assert_eq!(stone.sound(), "step.stone");
        assert_eq!(stone.volume(), 1.0);
        assert_eq!(stone.pitch(), 1.0);
    }
}
