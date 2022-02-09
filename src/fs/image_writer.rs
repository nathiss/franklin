use std::path::{Path, PathBuf};

use anyhow::Result;
use image::ColorType;

use crate::models::Image;

#[derive(Debug)]
pub struct ImageWriter {
    output_directory: PathBuf,
}

impl ImageWriter {
    pub fn to_dir(output_directory: &str) -> Self {
        Self {
            output_directory: Path::new(output_directory).to_owned(),
        }
    }

    pub fn write(&self, filename: &str, image: &Image) -> Result<()> {
        let full_path = self.output_directory.join(filename);

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
