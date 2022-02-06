use anyhow::Result;
use show_image::{create_window, event, ImageInfo, ImageView, WindowOptions, WindowProxy};

use crate::models::Image;

pub struct Window {
    window: WindowProxy,
}

impl Window {
    pub fn run_with_context<F>(func: F)
    where
        F: FnOnce(Self) -> Result<()> + Send + 'static,
    {
        show_image::run_context(move || -> Result<()> {
            let window_options = WindowOptions::default()
                .set_resizable(false)
                .set_size([512, 512]);
            let window = create_window("Franklin", window_options)?;

            let this = Self { window };
            func(this)?;

            Ok(())
        })
    }

    pub fn show_image(&self, title: &str, image: &Image) -> Result<()> {
        let image_info = ImageInfo::rgb8(image.height(), image.width());

        let image_bytes = image.as_raw_bytes();
        let image_view = ImageView::new(image_info, image_bytes.as_slice());

        self.window.set_image(title, image_view)?;

        for event in self.window.event_channel()? {
            if let event::WindowEvent::KeyboardInput(event) = event {
                println!("{:#?}", event);
                if event.input.key_code == Some(event::VirtualKeyCode::Escape)
                    && event.input.state.is_pressed()
                {
                    break;
                }
            }
        }

        Ok(())
    }
}
