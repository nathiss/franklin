use std::sync::mpsc::{Receiver, TryRecvError};

use anyhow::Result;
use show_image::{
    create_window,
    event::{self, WindowEvent},
    ImageInfo, ImageView, WindowOptions, WindowProxy,
};

use crate::models::Image;

pub struct Window {
    window: WindowProxy,
    event_receiver: Receiver<WindowEvent>,
    should_exit: bool,
}

impl Window {
    fn new(window: WindowProxy) -> Result<Self> {
        let event_receiver = window.event_channel()?;

        Ok(Self {
            window,
            event_receiver,
            should_exit: false,
        })
    }

    pub fn run_with_context<F>(dimensions: (usize, usize), func: F)
    where
        F: FnOnce(Self) -> Result<()> + Send + 'static,
    {
        show_image::run_context(move || -> Result<()> {
            let window_options = WindowOptions::default()
                .set_resizable(false)
                .set_size([dimensions.0 as u32, dimensions.1 as u32]);
            let window = create_window("Franklin", window_options)?;

            let this = Self::new(window)?;
            func(this)?;

            Ok(())
        })
    }

    pub fn show_image(&mut self, title: &str, image: &Image) -> Result<()> {
        let image_info = ImageInfo::rgb8(image.height() as u32, image.width() as u32);

        let image_bytes = image.as_raw_bytes();
        let image_view = ImageView::new(image_info, image_bytes.as_slice());

        self.handle_events();

        if !self.should_exit {
            self.window.set_image(title, image_view)?;
        }

        Ok(())
    }

    fn handle_events(&mut self) {
        loop {
            match self.event_receiver.try_recv() {
                Ok(event) => {
                    if let event::WindowEvent::KeyboardInput(event) = event {
                        if event.input.key_code == Some(event::VirtualKeyCode::Escape)
                            && event.input.state.is_pressed()
                        {
                            self.should_exit = true;
                        }
                    }
                }
                Err(TryRecvError::Disconnected) => {
                    println!("Disconnected");
                    self.should_exit = true
                }
                Err(TryRecvError::Empty) => break,
            }
        }
    }

    pub fn should_exit(&self) -> bool {
        self.should_exit
    }
}
