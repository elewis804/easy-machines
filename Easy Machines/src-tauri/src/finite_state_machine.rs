use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FiniteStateMachine<'i> {
    input: &'i str,
    states: Vec<&'i str>,
    current_state: &'i str,
    transitions: Vec<(&'i str, char, &'i str)>,
    accept_states: Vec<&'i str>
}