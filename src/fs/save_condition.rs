/// This enum specifies different options for choosing which images should be saved on the filesystem.
pub enum SaveCondition {
    /// Saves the best specimen from every generation.
    All,

    /// Saves the best specimen from every Nth generation, where N is the `u32` passed in `Each`.
    Each(u32),

    /// Never saves the specimens.
    Never,
}
