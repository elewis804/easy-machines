use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FiniteStateMachine<'i> {
    pub input: &'i str,
    pub states: Vec<&'i str>,
    pub current_state: Vec<&'i str>,
    pub transitions: Vec<(&'i str, char, &'i str)>,
    pub accept_states: Vec<&'i str>
}