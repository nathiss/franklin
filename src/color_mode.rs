/// This enum specifies different options for choosing color mode.
#[derive(Debug, Clone, Copy)]
pub enum ColorMode {
    /// Informs the environment that color channels of specimens should be treated separately.
    Rgb,

    /// Informs the environment that color channels of specimens should be treated together, i.e. all three hold the
    /// same value.
    Grayscale,
}
