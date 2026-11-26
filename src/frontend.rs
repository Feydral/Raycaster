#[cfg(not(any(feature = "termcanvas", feature = "nanofb")))]
compile_error!("Enable at least one frontend feature: 'termcanvas' or 'nanofb'");

use crate::player::PlayerInput;
use crate::render_target::RenderTarget;

const BACKEND_VARIABLE: &str = "RAYCASTER_BACKEND";

pub struct FrameInput {
    pub player: PlayerInput,
    pub quit: bool,
}

pub trait Frontend: RenderTarget {
    fn poll_input(&mut self) -> FrameInput;

    fn shutdown(self: Box<Self>) {}
}

pub fn create() -> Box<dyn Frontend> {
    match std::env::var(BACKEND_VARIABLE).ok().as_deref() {
        Some("termcanvas") => termcanvas(),
        Some("nanofb") => nanofb(),
        Some(other) => panic!("Unknown backend '{other}', expected 'termcanvas' or 'nanofb'"),
        None if cfg!(feature = "termcanvas") => termcanvas(),
        None => nanofb(),
    }
}

#[cfg(feature = "termcanvas")]
fn termcanvas() -> Box<dyn Frontend> {
    Box::new(crate::render_target::termcanvas::TermcanvasTarget::new())
}

#[cfg(not(feature = "termcanvas"))]
fn termcanvas() -> Box<dyn Frontend> {
    panic!("Backend 'termcanvas' is not compiled in, enable the 'termcanvas' feature")
}

#[cfg(feature = "nanofb")]
fn nanofb() -> Box<dyn Frontend> {
    Box::new(crate::render_target::nanofb::NanofbTarget::new())
}

#[cfg(not(feature = "nanofb"))]
fn nanofb() -> Box<dyn Frontend> {
    panic!("Backend 'nanofb' is not compiled in, enable the 'nanofb' feature")
}
