use rsfml::system::Clock;

pub enum AnimationState{
    Play,
    Pause,
    Stop
}

pub enum AnimationMode {
    PlayOnce,
    PlayInfinite
}

pub struct Animation {
    priv texture_ids : ~[i32],
    priv state : AnimationState,
    priv mode : AnimationMode,
    priv lag : f32,
    priv current_texture : u32,
    priv clock : Clock
}

impl Animation {
    pub fn new(texture_ids : ~[i32], state : AnimationState, mode : AnimationMode, lag : f32) -> Animation {
        Animation {
            texture_ids : texture_ids,
            state : state,
            mode : mode,
            lag : lag,
            current_texture : 0,
            clock : Clock::new()
        }
    }

    pub fn set_state(&mut self, new_state : AnimationState) -> () {
        self.state = new_state;
        match new_state {
            Stop    => { self.current_texture = 0; self.clock.restart(); },
            Play    => { self.current_texture = 0; self.clock.restart(); },
            _       => {}
        }
    }

    pub fn set_mode(&mut self, new_mode : AnimationMode) -> () {
        self.mode = new_mode;
    }

    pub fn get_state(&self) -> AnimationState {
        self.state
    }

    pub fn get_mode(&self) -> AnimationMode {
        self.mode
    }

    pub fn set_lag(&mut self, new_lag : f32) -> () {
        self.lag = new_lag
    }

    pub fn get_current_texture_id(&self) -> i32 {
        self.texture_ids[self.current_texture]
    }

    pub fn update(&mut self) -> () {
        match self.state {
            Play    => {
                if self.clock.get_elapsed_time().as_seconds() >= self.lag {
                    if self.current_texture == self.texture_ids.len() as u32 -1{
                        self.current_texture = 0;
                        match self.mode {
                            PlayOnce    => self.state = Stop,
                            _           => {}
                        }
                    }
                    else {
                        self.current_texture += 1;
                    }
                    self.clock.restart();
                }
            },
            _       => {}
        }
    }
}