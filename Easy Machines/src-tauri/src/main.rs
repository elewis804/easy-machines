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
    "current_state":
    [
      "_",
      "_",
      ...
    ],
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

use finite_state_machine::FiniteStateMachine;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![load_machine])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn load_machine(file_name:String) -> bool{
  let machine: FiniteStateMachine = serde_json::from_str(&file_name).unwrap();
  run_machine(machine, 0)
}

fn run_machine(mut machine: FiniteStateMachine, depth: i32)-> bool{
  if depth >= 100 {
    return false;
  }
  
  else if machine.input.is_empty() {
    let result_states = get_next_states("\0", machine.current_state, &machine.transitions, depth + 1);
    for s in result_states {
      println!("{}", s);
      if machine.accept_states.contains(&s) {
        return true;
      }
    }
    return false;
  }

  machine.current_state = 
  get_next_states(&machine.input[0..1], machine.current_state, &machine.transitions, depth);
  machine.input = &machine.input[1..];
  run_machine(machine, depth + 1)
}

fn get_next_states<'i>(in_char: &'i str, current_states: Vec<&'i str>, transitions: &Vec<(&'i str, char, &'i str)>, depth: i32) -> Vec<&'i str> {
  let mut results = Vec::new();
  for s in current_states.iter() {
    for t in transitions.iter() {
      if depth >= 100 {
        results = vec!["NA"];
        return results;
      }

      else if in_char.eq(&t.1.to_string()) && s.eq(&t.0) {
        results.push(t.2);
      }

      else if t.1 == '\0' && s.eq(&t.0) {
        let mut temp_transitions = Vec::new();
        for t2 in transitions {
          temp_transitions.push(t2.to_owned());
        }
        results.append(&mut get_next_states(in_char, vec![t.0], &temp_transitions, depth + 1));
      }

      else if in_char == "\0" {
        for st in current_states.iter() {
          results.push(st.to_owned());
          return results;
        }
      }
    }
  }
  results
}

#[test]
fn machines_serialize_correctly() {
      let mut s = Vec::new();
      s.push("s0");
      let mut t = Vec::new();
      t.push(("s0",'a',"s0"));
      let mut c = Vec::new();
      c.push("s0");
      let mut a = Vec::new();
      a.push("s0");
      let machine = FiniteStateMachine {input: "a", states: s, current_state: c, transitions: t, accept_states: a };
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
    "current_state":
    [
      "s0"
    ]
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
  for s in machine.current_state {
    println!("{}", s);
  }
  for t in machine.transitions {
    println!("({}, {}, {})", t.0, t.1, t.2);
  }
  for a in machine.accept_states {
    println!("{}", a);
  } 
}

#[test]
fn machine_can_return_false() {
  let mut s = Vec::new();
    s.push("s0");
    let mut t = Vec::new();
    t.push(("s0",'a',"s1"));
    let mut c = Vec::new();
    c.push("s0");
    let mut a = Vec::new();
    a.push("s1");
  let machine: FiniteStateMachine = FiniteStateMachine {input: "", states: s, current_state: c, transitions: t, accept_states: a };
  assert!(run_machine(machine, 0) == false);
}

#[test]
fn machine_can_return_true() {
  let mut s = Vec::new();
    s.push("s0");
    let mut t = Vec::new();
    t.push(("s0",'a',"s1"));
    let mut c = Vec::new();
    c.push("s0");
    let mut a = Vec::new();
    a.push("s1");
  let machine: FiniteStateMachine = FiniteStateMachine {input: "a", states: s, current_state: c, transitions: t, accept_states: a };
  assert!(run_machine(machine, 0));
}

#[test]
fn machine_can_run_epsilon_transitions() {
  let mut s = Vec::new();
    s.push("s0");
    s.push("s1");
    let mut t = Vec::new();
    t.push(("s0",'\0',"s1"));
    let mut c = Vec::new();
    c.push("s0");
    let mut a = Vec::new();
    a.push("s1");
  let machine: FiniteStateMachine = FiniteStateMachine {input: "", states: s, current_state: c, transitions: t, accept_states: a };
  assert!(run_machine(machine, 0));
}

#[test]
fn machine_can_take_multiple_transitions() {
  let mut s = Vec::new();
    s.push("s0");
    s.push("s1");
    let mut t = Vec::new();
    t.push(("s0",'a',"s0"));
    t.push(("s0",'a',"s1"));
    t.push(("s1", 'b', "s1"));
    let mut c = Vec::new();
    c.push("s0");
    let mut a = Vec::new();
    a.push("s1");
  let machine: FiniteStateMachine = FiniteStateMachine {input: "aab", states: s, current_state: c, transitions: t, accept_states: a };
  assert!(run_machine(machine, 0));
}