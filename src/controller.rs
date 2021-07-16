use std::sync::{Arc, Mutex};

pub type ControllerRef = Arc<Mutex<Controller>>;

bitflags! {
    pub struct ButtonStatus: u8 {
        const A = 1 << 0;
        const B = 1 << 1;
        const SELECT = 1 << 2;
        const START = 1 << 3;
        const UP = 1 << 4;
        const DOWN = 1 << 5;
        const LEFT = 1 << 6;
        const RIGHT = 1 << 7;
    }
}

pub struct Controller {
    strobe: bool,
    button_status: ButtonStatus,
    shift_register: u8,
}

impl Controller {
    pub fn new() -> Controller {
        Controller {
            strobe: false,
            button_status: ButtonStatus::empty(),
            shift_register: 0,
        }
    }

    pub fn new_ref() -> ControllerRef {
        Arc::new(Mutex::new(Controller::new()))
    }

    pub fn write(&mut self, value: u8) {
        self.strobe = (value & 1) > 0;

        if self.strobe {
            self.shift_register = self.button_status.bits;
        }
    }

    pub fn read(&mut self) -> u8 {
        if self.strobe {
            return self.button_status.bits & 1;
        }

        let value = self.shift_register & 1;

        self.shift_register >>= 1;

        value
    }

    pub fn set_button_status(&mut self, button: ButtonStatus, is_pressed: bool) {
        self.button_status.set(button, is_pressed);
    }
}
