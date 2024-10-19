use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug, Deserialize, Clone)]
struct Transition {
    event: String,
    from: String,
    to: String,
    action: Option<String>,
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
    actions: Arc<Mutex<HashMap<String, Box<dyn Fn() + Send + Sync>>>>,
}

impl StateMachine {
    pub fn new(config_loader: &dyn ConfigLoader, event_listener: Arc<Mutex<dyn EventListener>>) -> Result<Self, Box<dyn std::error::Error>> {
        let config = config_loader.load_config()?;
        Ok(StateMachine {
            transitions: config.transitions,
            current_state: Arc::new(Mutex::new(config.initial_state)),
            event_listener,
            running: Arc::new(Mutex::new(true)),
            actions: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    pub fn run(&self) {
        let running = self.running.clone();
        let current_state = self.current_state.clone();
        let event_listener = self.event_listener.clone();
        let transitions = self.transitions.clone();
        let actions = self.actions.clone();

        thread::spawn(move || {
            while *running.lock().unwrap() {
                if let Some(event) = event_listener.lock().unwrap().listen() {
                    let mut state = current_state.lock().unwrap();
                    if let Some(transition) = Self::find_transition(&transitions, &state, &event) {
                        println!("State transition from {} to {}", state, transition.to);
                        *state = transition.to.clone();
                        
                        if let Some(action_name) = &transition.action {
                            if let Some(action) = actions.lock().unwrap().get(action_name) {
                                action();
                            }
                        }
                    } else {
                        println!("Unable to trigger event {} from state {}", event, state);
                    }
                }
            }
        });
    }

    fn find_transition<'a>(transitions: &'a [Transition], current_state: &str, event: &str) -> Option<&'a Transition> {
        transitions.iter().find(|t| t.from == current_state && t.event == event)
    }

    pub fn get_current_state(&self) -> String {
        self.current_state.lock().unwrap().clone()
    }

    pub fn register_action<F>(&self, name: &str, action: F)
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.actions.lock().unwrap().insert(name.to_string(), Box::new(action));
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

pub trait ConfigLoader {
    fn load_config(&self) -> Result<StateMachineConfig, Box<dyn std::error::Error>>;
}

pub struct JsonFileLoader {
    path: String,
}

impl JsonFileLoader {
    pub fn new(path: String) -> Self {
        JsonFileLoader { path }
    }
}

impl ConfigLoader for JsonFileLoader {
    fn load_config(&self) -> Result<StateMachineConfig, Box<dyn std::error::Error>> {
        let file = File::open(&self.path)?;
        let reader = BufReader::new(file);
        let config: StateMachineConfig = serde_json::from_reader(reader)?;
        Ok(config)
    }
}
