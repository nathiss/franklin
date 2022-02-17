/// This enum specifies different options for choosing which images should be displayed on the screen.
pub enum DisplayCondition {
    /// Displays the best specimen from every generation.
    All,

    /// Displays the best specimen from every Nth generation, where N is the `u32` passed in `Every`.
    Every(u32),

    /// Never displays the specimens.
    ///
    /// The GUI context is not loaded.
    None,
}
