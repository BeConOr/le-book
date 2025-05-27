use anyhow::{Context, Result};
use gpio_cdev::{Chip, EventRequestFlags, Line, LineEventHandle, LineRequestFlags};
use std::cell::RefCell;
use std::collections::HashMap;
use std::os::unix::io::AsRawFd;

pub use super::input_controller::{CallBack, Epoll, InputController, InputEvent, InterruptCapable};

use crate::logger::logger_msg::*;

struct GpioMetadata {
    cb: CallBack,
    eh: LineEventHandle,
}

pub struct GpioController {
    chip: RefCell<Chip>,
    epfd: Epoll,
    callbacks: HashMap<u32, GpioMetadata>,
}

impl GpioController {
    #[allow(dead_code)]
    pub fn new(name: &str) -> Result<Self> {
        let chip = RefCell::new(
            Chip::new(name).with_context(|| format!("Failed to open a GPIO controller {name}"))?,
        );

        let epfd = Epoll::new().context("Failed to create epoll for KeyboardController")?;

        Ok(Self {
            chip,
            epfd,
            callbacks: HashMap::new(),
        })
    }
}

impl InputController for GpioController {
    fn read_line(&self, id: u32) -> bool {
        match self
            .chip
            .borrow_mut()
            .get_line(id)
            .and_then(|line| line.request(LineRequestFlags::INPUT, 0, "GpioController.ReadLine"))
            .and_then(|handle| handle.get_value())
        {
            Ok(value) => value > 0,
            Err(e) => {
                event!(Level::WARN, error = ?e, "Failed to read GPIO value for line {id}");
                false
            }
        }
    }
}

impl InterruptCapable for GpioController {
    #[instrument(skip(self, cb))]
    fn register_callback(&mut self, id: u32, cb: CallBack) -> Result<()> {
        let line: Line = self
            .chip
            .borrow_mut()
            .get_line(id)
            .with_context(|| format!("Cannot get a line {id}"))?;

        let event_handle: LineEventHandle = line
            .events(
                LineRequestFlags::INPUT,
                EventRequestFlags::BOTH_EDGES,
                format!("GpioController.Line{id}").as_str(),
            )
            .with_context(|| format!("Cannot get an event handler for a line {id}"))?;
        let fd = event_handle.as_raw_fd();
        self.epfd
            .register(fd, Some(id as u64))
            .with_context(|| format!("Failed to register a line {id} with epoll"))?;

        let metadata = GpioMetadata {
            cb: cb,
            eh: event_handle,
        };

        self.callbacks.insert(id, metadata);
        Ok(())
    }

    fn run(&mut self) {
        let mut events = Epoll::generate_empty_events::<128>();
        event!(Level::INFO, "GpioController: running epoll loop...");

        loop {
            let n = match self.epfd.wait(&mut events) {
                Ok(n) => n,
                Err(e) => {
                    event!(Level::ERROR, error = ?e, "Cannot wait for epoll");
                    continue;
                }
            };

            for ev in &events[..n] {
                let id = ev.data();

                if let Some(metadata) = self.callbacks.get(&(id as u32)) {
                    let event = match metadata.eh.get_value() {
                        Ok(value) => match value {
                            1 => InputEvent::Press,
                            0 => InputEvent::Release,
                            _ => continue,
                        },
                        Err(e) => {
                            event!(Level::WARN, error = ?e, "Failed to read GPIO value for line {id}");
                            continue;
                        }
                    };
                    (metadata.cb)(event);
                }
            }
        }
    }
}
