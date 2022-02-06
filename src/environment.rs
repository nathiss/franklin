use anyhow::Result;

use crate::{display::Window, models::Image};

pub struct Environment {
    image: Image,
}

impl Environment {
    pub fn for_image(image: Image) -> Self {
        Self { image }
    }

    pub fn run(self) {
        Window::run_with_context(move |window| -> Result<()> {
            window.show_image("Lorem ipsum", &self.image)?;

            Ok(())
        });
    }
}
