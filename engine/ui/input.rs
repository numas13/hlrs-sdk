use xash3d_shared::ffi::common::kbutton_t;

pub use xash3d_shared::input::*;

pub struct KeyButton {
    raw: *mut kbutton_t,
}

impl KeyButton {
    pub(crate) unsafe fn from_raw(raw: *mut kbutton_t) -> Self {
        Self { raw }
    }

    pub fn as_ptr(&self) -> *mut kbutton_t {
        self.raw
    }

    fn state(&self) -> KeyState {
        let state = unsafe { (*self.as_ptr()).state };
        KeyState::from_bits_retain(state)
    }

    pub fn is_down(&self) -> bool {
        self.state().intersects(KeyState::DOWN)
    }

    pub fn is_up(&self) -> bool {
        !self.is_down()
    }

    pub fn is_impulse_down(&self) -> bool {
        self.state().intersects(KeyState::IMPULSE_DOWN)
    }

    pub fn is_impulse_up(&self) -> bool {
        self.state().intersects(KeyState::IMPULSE_UP)
    }

    pub fn is_down_or_impulse_down(&self) -> bool {
        self.state()
            .intersects(KeyState::DOWN | KeyState::IMPULSE_DOWN)
    }
}
