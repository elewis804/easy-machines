use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FiniteStateMachine<'i> {
    pub input: &'i str,
    pub states: Vec<&'i str>,
    pub current_state: &'i str,
    pub transitions: Vec<(&'i str, char, &'i str)>,
    pub accept_states: Vec<&'i str>
}

impl<'i> FiniteStateMachine<'i> {
    pub fn run(&mut self) -> bool {
        'outer: while self.input.len() > 0 && !self.current_state.eq("NA") {
            'inner: for t in &self.transitions {
                if self.current_state.eq(t.0) && t.1.eq(&'\0') {
                    self.current_state = t.2;
                }
                if self.current_state.eq(t.0) && self.input.get(0..1).unwrap().eq(&(t.1).to_string()) {
                    self.current_state = t.2;
                    if self.input.len() > 1 {
                        self.input = self.input.get(1..self.input.len()).unwrap();
                        continue 'inner;
                    }
                    else {
                        break 'outer;
                    }
                }
            }

            self.current_state = "NA";

        }

        if self.accept_states.contains(&self.current_state) {
            return true;
        }
        else {
            return false;
        }
    }
}