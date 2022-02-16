use std::path::PathBuf;

use anyhow::Result;
use image::ColorType;

use crate::models::Image;

#[derive(Debug)]
pub(crate) struct ImageWriter {
    output_directory: PathBuf,
    filename_prefix: String,
}

impl ImageWriter {
    #[must_use]
    pub(crate) fn new(output_directory: String, filename_prefix: String) -> Self {
        Self {
            output_directory: PathBuf::from(output_directory),
            filename_prefix,
        }
    }

    pub(crate) fn write(&self, current_generation_number: u32, image: &Image) -> Result<()> {
        let full_path = self.output_directory.join(format!(
            "{}{:0>6}.png",
            self.filename_prefix, current_generation_number
        ));

        image::save_buffer(
            full_path,
            image.as_raw_bytes().as_slice(),
            image.width() as u32,
            image.height() as u32,
            ColorType::Rgb8,
        )?;

        Ok(())
    }
}
