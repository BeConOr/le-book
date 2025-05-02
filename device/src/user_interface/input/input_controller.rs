use anyhow::{Context, Result};
use nix::sys::epoll;
use std::os::fd::RawFd;

pub type CallBack = Box<dyn Fn(InputEvent) + Send>;

#[derive(Debug, Clone, Copy)]
pub enum InputEvent {
    Press,
    Release,
}

pub struct Epoll {
    fd: RawFd,
}

impl Epoll {
    pub fn new() -> Result<Self> {
        Ok(Self {
            fd: epoll::epoll_create().context("epoll_create failed")?,
        })
    }

    pub fn register(&self, fd: RawFd) -> Result<()> {
        let mut ev = epoll::EpollEvent::new(epoll::EpollFlags::EPOLLIN, fd as u64);
        epoll::epoll_ctl(self.fd, epoll::EpollOp::EpollCtlAdd, fd, &mut ev)
            .context("epoll_ctl failed")?;
        Ok(())
    }

    pub fn wait(&self, buffer: &mut [epoll::EpollEvent]) -> Result<usize> {
        Ok(epoll::epoll_wait(self.fd, buffer, 10000).context("epoll_wait failed")?)
    }

    pub fn generate_empty_events<const N: usize>() -> [epoll::EpollEvent; N] {
        [epoll::EpollEvent::empty(); N]
    }

    #[allow(dead_code)]
    pub fn fd(&self) -> RawFd {
        self.fd
    }
}

impl Drop for Epoll {
    fn drop(&mut self) {
        let _ = nix::unistd::close(self.fd);
    }
}

#[allow(dead_code)]
pub trait InputController {
    fn read_line(&self, id: u32) -> bool;
}

pub trait InterruptCapable: InputController {
    fn register_callback(&mut self, id: u32, cb: CallBack);
    fn run(&mut self);
}
