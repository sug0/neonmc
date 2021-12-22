//! Material kinds of a Minecraft block.

use crate::common::ID;

/// Metadata about Minecraft materials.
pub struct MaterialMeta {
    can_burn: bool,
    func_217_d: bool,
    func_216_a: bool,
    func_219_b: bool,
    func_218_c: bool,
}

/// All material kinds in Minecraft.
#[repr(usize)]
#[derive(Copy, Clone, Debug)]
pub enum Material {
    Air,
    Ground,
    Wood,
    Rock,
    Iron,
    Water,
    Lava,
    Leaves,
    Plants,
    Sponge,
    Cloth,
    Fire,
    Sand,
    Circuits,
    Glass,
    Tnt,
    Ice,
    Snow,
    BuiltSnow,
    Cactus,
    Clay,
}

macro_rules! flip {
    ($material:expr, $field:ident) => {{
        let mut material = $material;
        material.$field = !material.$field;
        material
    }}
}

// TODO: to be documented
impl MaterialMeta {
    pub fn can_burn(&self) -> bool {
        self.can_burn
    }

    pub fn func_217_d(&self) -> bool {
        self.func_217_d
    }

    pub fn func_216_a(&self) -> bool {
        self.func_216_a
    }

    pub fn func_219_b(&self) -> bool {
        self.func_219_b
    }

    pub fn func_218_c(&self) -> bool {
        self.func_218_c
    }

    const fn default_material() -> Self {
        Self {
            can_burn: false, func_217_d: false, func_216_a: true,
            func_219_b: true, func_218_c: true,
        }
    }

    const fn burnable_material() -> Self {
        let material = Self::default_material();
        let material = flip!(material, can_burn);
        material
    }

    const fn logic_material() -> Self {
        let material = Self::default_material();
        let material = flip!(material, func_216_a);
        let material = flip!(material, func_219_b);
        let material = flip!(material, func_218_c);
        material
    }

    const fn transparent_material() -> Self {
        let material = Self::default_material();
        let material = flip!(material, func_216_a);
        let material = flip!(material, func_219_b);
        let material = flip!(material, func_218_c);
        material
    }

    const fn liquid_material() -> Self {
        let material = Self::default_material();
        let material = flip!(material, func_217_d);
        let material = flip!(material, func_218_c);
        let material = flip!(material, func_216_a);
        material
    }
}

impl Material {
    /// Returns the metadata of a material.
    pub fn get(self) -> &'static MaterialMeta {
        &LIST[self as ID]
    }
}

static LIST: [MaterialMeta; 21] = [
    // Air
    MaterialMeta::transparent_material(),
    // Ground
    MaterialMeta::default_material(),
    // Wood
    MaterialMeta::burnable_material(),
    // Rock
    MaterialMeta::default_material(),
    // Iron
    MaterialMeta::default_material(),
    // Water
    MaterialMeta::liquid_material(),
    // Lava
    MaterialMeta::liquid_material(),
    // Leaves
    MaterialMeta::burnable_material(),
    // Plants
    MaterialMeta::logic_material(),
    // Sponge
    MaterialMeta::default_material(),
    // Cloth
    MaterialMeta::burnable_material(),
    // Fire
    MaterialMeta::transparent_material(),
    // Sand
    MaterialMeta::default_material(),
    // Circuits
    MaterialMeta::logic_material(),
    // Glass
    MaterialMeta::default_material(),
    // Tnt
    MaterialMeta::burnable_material(),
    // Ice
    MaterialMeta::default_material(),
    // Snow
    MaterialMeta::logic_material(),
    // BuiltSnow
    MaterialMeta::default_material(),
    // Cactus
    MaterialMeta::default_material(),
    // Clay
    MaterialMeta::default_material(),
];

#[cfg(test)]
mod tests {
    use super::Material;

    #[test]
    fn test_get() {
        let tnt = Material::Tnt.get();

        assert!(tnt.can_burn());
    }
}
