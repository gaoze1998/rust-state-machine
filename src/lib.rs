use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Mutex};

#[derive(Debug, Deserialize)]
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
    current_state: String,
    event_listener: Box<dyn EventListener>,
}

impl StateMachine {
    pub fn new(config: StateMachineConfig, event_listener: Box<dyn EventListener>) -> Self {
        StateMachine {
            transitions: config.transitions,
            current_state: config.initial_state,
            event_listener,
        }
    }

    pub fn load_from_file(path: &str, event_listener: Box<dyn EventListener>) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let config: StateMachineConfig = serde_json::from_reader(reader)?;
        Ok(StateMachine::new(config, event_listener))
    }

    pub fn trigger_event(&self, current_state: &str, event: &str) -> Option<&str> {
        for transition in &self.transitions {
            if transition.from == current_state && transition.event == event {
                return Some(transition.to.as_str());
            }
        }
        None
    }

    pub fn run(&mut self) {
        loop {
            let event = self.event_listener.listen();
            if let Some(new_state) = self.trigger_event(&self.current_state, &event) {
                println!("状态从 {} 转移到 {}", self.current_state, new_state);
                self.current_state = new_state.to_string();
            } else {
                println!("无法从状态 {} 触发事件 {}", self.current_state, event);
            }
        }
    }

    pub fn get_current_state(&self) -> String {
        self.current_state.clone()
    }
}

pub trait EventListener: Send + Sync {
    fn listen(&self) -> String;
    fn get_sender(&self) -> Sender<String>;
}

pub struct SimpleEventListener {
    receiver: Mutex<Receiver<String>>,
    sender: Sender<String>,
}

impl SimpleEventListener {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        SimpleEventListener { 
            receiver: Mutex::new(receiver), 
            sender 
        }
    }
}

impl EventListener for SimpleEventListener {
    fn listen(&self) -> String {
        self.receiver.lock()
            .unwrap()
            .recv()
            .unwrap_or_else(|_| String::from("错误"))
    }

    fn get_sender(&self) -> Sender<String> {
        self.sender.clone()
    }
}
