// Gravity Changer: an empty native mod for the Horizon Minecraft client.
// This file is both the mod's Cargo library (its tests) and the module the
// bundle adds to the game, as `crate::game::gravity_changer`.

/// The mod's id (its settings folder), name and version.
pub const ID: &str = "gravity_changer";
pub const NAME: &str = "Gravity Changer";
pub const VERSION: &str = "0.1.0";
pub const DESCRIPTION: &str = "An empty starting point: change gravity here.";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_mod_identifies_itself() {
        assert_eq!(ID, "gravity_changer");
        assert!(!NAME.is_empty());
        assert!(!VERSION.is_empty());
    }
}
