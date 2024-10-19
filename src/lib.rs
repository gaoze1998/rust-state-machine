use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug, Deserialize, Clone)]
struct Transition {
    event: String,
    from: String,
    to: String,
}

#[derive(Debug, Deserialize)]
pub struct StateMachineConfig {
    initial_state: String,
    transitions: Vec<Transition>,
}

pub struct StateMachine {
    transitions: Vec<Transition>,
    current_state: Arc<Mutex<String>>,
    event_listener: Arc<Mutex<dyn EventListener>>,
    running: Arc<Mutex<bool>>,
}

impl StateMachine {
    pub fn new(config: StateMachineConfig, event_listener: Arc<Mutex<dyn EventListener>>) -> Self {
        StateMachine {
            transitions: config.transitions,
            current_state: Arc::new(Mutex::new(config.initial_state)),
            event_listener,
            running: Arc::new(Mutex::new(true)),
        }
    }

    pub fn load_from_file(path: &str, event_listener: Arc<Mutex<dyn EventListener>>) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let config: StateMachineConfig = serde_json::from_reader(reader)?;
        Ok(StateMachine::new(config, event_listener))
    }

    pub fn trigger_event(&self, current_state: &str, event: &str) -> Option<String> {
        for transition in &self.transitions {
            if transition.from == current_state && transition.event == event {
                return Some(transition.to.clone());
            }
        }
        None
    }

    pub fn run(&self) {
        let running = self.running.clone();
        let current_state = self.current_state.clone();
        let event_listener = self.event_listener.clone();
        let transitions = self.transitions.clone();

        thread::spawn(move || {
            while *running.lock().unwrap() {
                if let Some(event) = event_listener.lock().unwrap().listen() {
                    let mut state = current_state.lock().unwrap();
                    if let Some(new_state) = Self::trigger_event_static(&transitions, &state, &event) {
                        println!("状态从 {} 转移到 {}", state, new_state);
                        *state = new_state;
                    } else {
                        println!("无法从状态 {} 触发事件 {}", state, event);
                    }
                }
            }
        });
    }

    fn trigger_event_static(transitions: &[Transition], current_state: &str, event: &str) -> Option<String> {
        for transition in transitions {
            if transition.from == current_state && transition.event == event {
                return Some(transition.to.clone());
            }
        }
        None
    }

    pub fn get_current_state(&self) -> String {
        self.current_state.lock().unwrap().clone()
    }
}

impl Drop for StateMachine {
    fn drop(&mut self) {
        let mut running = self.running.lock().unwrap();
        *running = false;
    }
}

pub trait EventListener: Send + Sync {
    fn listen(&self) -> Option<String>;
}

pub struct SimpleEventListener {
    receiver: Arc<Mutex<std::sync::mpsc::Receiver<String>>>,
}

impl SimpleEventListener {
    pub fn new() -> (Self, std::sync::mpsc::Sender<String>) {
        let (sender, receiver) = std::sync::mpsc::channel();
        (SimpleEventListener { 
            receiver: Arc::new(Mutex::new(receiver))
        }, sender)
    }
}

impl EventListener for SimpleEventListener {
    fn listen(&self) -> Option<String> {
        self.receiver.lock()
            .unwrap()
            .recv()
            .ok()
    }
}
