#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

/* JSON FORMAT
{
    "input":"_",
    "states":
    [
      "_",
      "_",
      ...
    ],
    "current_state":"s0",
    "transitions":[
      [
        "_",
        "_",
        "_"
      ],
      [
        "_",
        "_",
        "_"
      ],
      ...
    ],
    "accept_states":
    [
      "_",
      "_",
      ...
    ]
  } */

pub mod finite_state_machine;

use std::ops::Range;

use finite_state_machine::FiniteStateMachine;
use serde::{Deserialize, Serialize};
use serde_json::Result;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![load_machine])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn load_machine(file_name:String) -> bool{
  let mut machine: FiniteStateMachine = serde_json::from_str(&file_name).unwrap();
  machine.run()
}

#[test]
fn machines_serialize_correctly() {
      let mut s = Vec::new();
      s.push("s0");
      let mut t = Vec::new();
      t.push(("s0",'a',"s0"));
      let mut a = Vec::new();
      a.push("s0");
      let machine = FiniteStateMachine {input: "a", states: s, current_state: "s0", transitions: t, accept_states: a };
      let serialized_machine = serde_json::to_string(&machine).unwrap();
      println!("{}", serialized_machine);
}

#[test]
fn machines_deserialize_correctly() {
  let serialized_machine = r#"
  {
    "input":"a",
    "states":
    [
      "s0"
    ],
    "current_state":"s0",
    "transitions":[
      [
        "s0",
        "a",
        "s0"
      ]
    ],
    "accept_states":
    [
      "s0"
    ]
  }"#;
  let machine: FiniteStateMachine = serde_json::from_str(serialized_machine).unwrap();
  println!("{}", machine.input);
  for s in machine.states {
    println!("{}", s);
  }
  println!("{}", machine.current_state);
  for t in machine.transitions {
    println!("({}, {}, {})", t.0, t.1, t.2);
  }
  for a in machine.accept_states {
    println!("{}", a);
  }

  //TODO Write tests for FSM
}