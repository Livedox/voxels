#[derive(Clone, Copy, PartialEq, Debug)]
pub enum State {
    Pressed,
    Released,
    Jpressed,
    Jreleased
}


#[derive(Debug)]
pub struct InputEvent {
    pub keyboard: [State; 256],
    pub mouse: [State; 4],
    prev_key_id: Option<usize>,
    prev_mouse_id: Option<usize>,
    pub delta_x: f32,
    pub delta_y: f32,
    x: f32,
    y: f32
}

impl InputEvent {
    pub fn new() -> InputEvent {
        InputEvent {
            keyboard: [State::Released; 256],
            mouse: [State::Released; 4],
            prev_key_id: None,
            prev_mouse_id: None,
            delta_x: 0.0,
            delta_y: 0.0,
            x: 0.0,
            y: 0.0
        }
    }


    pub fn update(&mut self) {
        self.update_key();
        self.update_mouse();
        self.delta_x = 0.;
        self.delta_y = 0.;
    }


    fn update_key(&mut self) {
        if let Some(prev_key_id) = self.prev_key_id {
            let state = if self.keyboard[prev_key_id] != State::Jreleased {State::Pressed} else {State::Released};
            self.keyboard[prev_key_id] = state;
            self.prev_key_id = None;
        }
    }


    fn update_mouse(&mut self) {
        if let Some(prev_mouse_id) = self.prev_mouse_id {
            let state = if self.mouse[prev_mouse_id] != State::Jreleased {State::Pressed} else {State::Released};
            self.mouse[prev_mouse_id] = state;
            self.prev_mouse_id = None;
        }
    }


    pub fn press_key(&mut self, id: usize, state: State) {
        if (self.keyboard[id] != State::Pressed && state == State::Jpressed) || state == State::Jreleased {
            self.keyboard[id] = state;
            self.prev_key_id = Some(id);
        }
    }


    pub fn press_mouse(&mut self, id: usize, state: State) {
        if (self.mouse[id] != State::Pressed && state == State::Jpressed) || state == State::Jreleased {
            self.mouse[id] = state;
            self.prev_mouse_id = Some(id);
        }
    }


    pub fn mouse_move(&mut self, x: f32, y: f32) {
        self.delta_x = self.x - x;
        self.delta_y = self.y - y;
        self.x = x;
        self.y = y;
    }

    pub fn set_delta(&mut self, delta_x: f32, delta_y: f32) {
        self.delta_x = delta_x;
        self.delta_y = delta_y;
    }
}