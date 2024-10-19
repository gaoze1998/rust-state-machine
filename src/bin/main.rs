use rust_state_machine::{SimpleEventListener, StateMachine, EventListener};
use std::thread;
use std::time::Duration;

fn main() {
    let event_listener = SimpleEventListener::new();
    let sender = event_listener.get_sender();
    
    let mut state_machine = StateMachine::load_from_file("example-json.json", Box::new(event_listener))
        .expect("Failed to load state machine configuration");

    thread::spawn(move || {
        thread::sleep(Duration::from_secs(2));
        sender.send(String::from("start")).unwrap();
        thread::sleep(Duration::from_secs(2));
        sender.send(String::from("finish")).unwrap();
    });

    state_machine.run();
}
