use anyhow::{Context, Result};
use evdev::{Device, EventType, KeyCode};
use std::collections::HashMap;
use std::os::unix::io::AsRawFd;

pub use super::input_controller::{CallBack, Epoll, InputController, InputEvent, InterruptCapable};

use crate::logger::logger_msg::*;

pub struct KeyboardController {
    device: Device,
    epfd: Epoll,
    callbacks: HashMap<u32, CallBack>,
}

impl KeyboardController {
    pub fn new(path: &str) -> Result<Self> {
        let device = Device::open(path)
            .with_context(|| format!("Failed to open input device at path: {path}"))?;

        let epfd = Epoll::new().context("Failed to create epoll for KeyboardController")?;

        let fd = device.as_raw_fd();
        epfd.register(fd)
            .context("Failed to register device with epoll")?;

        Ok(Self {
            device,
            epfd,
            callbacks: HashMap::new(),
        })
    }
}

impl InputController for KeyboardController {
    fn read_line(&self, id: u32) -> bool {
        self.device
            .get_key_state()
            .map(|set| set.contains(KeyCode::new(id as u16)))
            .unwrap_or(false)
    }
}

impl InterruptCapable for KeyboardController {
    #[instrument(skip(self, cb))]
    fn register_callback(&mut self, id: u32, cb: CallBack) {
        self.callbacks.insert(id, cb);
    }

    fn run(&mut self) {
        let mut events = Epoll::generate_empty_events::<128>();
        event!(Level::INFO, "KeyboardController: running epoll loop...");

        loop {
            let n = match self.epfd.wait(&mut events) {
                Ok(n) => n,
                Err(e) => {
                    event!(Level::ERROR, error = ?e, "Cannot wait for epoll");
                    continue;
                }
            };

            for i in 0..n {
                if let Ok(events) = self.device.fetch_events() {
                    for ev in events {
                        if ev.event_type() == EventType::KEY {
                            let id = ev.code();
                            if let Some(callback) = self.callbacks.get(&(id as u32)) {
                                let event = match ev.value() {
                                    1 => InputEvent::Press,
                                    0 => InputEvent::Release,
                                    _ => continue,
                                };
                                (callback)(event);
                            }
                        }
                    }
                } else {
                    event!(
                        Level::WARN,
                        device_id = i,
                        "Failed to fetch events from device"
                    );
                }
            }
        }
    }
}
