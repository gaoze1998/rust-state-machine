# Rust State Machine

[中文](Readme-zh.md) | English

A flexible, configurable, and efficient state machine library implemented in Rust. This library provides a simple yet powerful way to manage complex state transition logic.

## Key Features

1. **Configuration-Driven**: Define state machines using JSON files (with support for extensible configuration file reading methods), allowing for flexible and easily modifiable configurations.

2. **Highly Reusable**: Core state machine logic encapsulated in the library, reusable across multiple projects.

3. **Concurrency-Safe**: Utilizes Rust's concurrency primitives to ensure thread-safe operations.

4. **Event-Driven**: Implements an event listener pattern, supporting asynchronous event handling.

5. **Strong Typing for Error Handling**: Employs Rust's Result type for robust error handling.

6. **Serialization Support**: Implements configuration serialization and deserialization using the serde library.

7. **Custom Actions**: Allows registration of custom action functions to be executed during state transitions.

8. **Lightweight**: Minimizes dependencies to maintain library lightness.

## Quick Start

### 1. Add Dependency

Add the following dependency to your `Cargo.toml` file:

```toml
[dependencies]
rust-state-machine = { path = "path/to/rust-state-machine" }
```

### 2. Create State Machine Configuration

Create a JSON file (e.g., `state_machine_config.json`) to define your state machine:

```json
{
    "initial_state": "Created",
    "transitions": [
        {
            "event": "Pay",
            "from": "Created",
            "to": "Paid",
            "action": "process_payment"
        },
        {
            "event": "Ship",
            "from": "Paid",
            "to": "Shipped",
            "action": "send_shipping_notification"
        },
        {
            "event": "Deliver",
            "from": "Shipped",
            "to": "Delivered",
            "action": "update_inventory"
        }
    ]
}
```

### 3. Implement State Machine

Create a state machine instance in your Rust code:

```rust
use rust_state_machine::{SimpleEventListener, StateMachine, JsonFileLoader};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // Create event listener and sender
    let (event_listener, sender) = SimpleEventListener::new();
    let event_listener = Arc::new(Mutex::new(event_listener));

    // Load configuration and create state machine
    let config_loader = JsonFileLoader::new("state_machine_config.json".to_string());
    let state_machine = StateMachine::new(&config_loader, event_listener.clone())
        .expect("Failed to load state machine configuration");

    // Register action functions
    state_machine.register_action("process_payment", || {
        println!("Processing payment");
    });
    state_machine.register_action("send_shipping_notification", || {
        println!("Sending shipping notification");
    });
    state_machine.register_action("update_inventory", || {
        println!("Updating inventory");
    });

    // Start the state machine
    state_machine.run();

    // Simulate event sending
    thread::spawn(move || {
        sender.send(String::from("Pay")).unwrap();
        thread::sleep(Duration::from_secs(1));
        sender.send(String::from("Ship")).unwrap();
        thread::sleep(Duration::from_secs(1));
        sender.send(String::from("Deliver")).unwrap();
    });

    // Allow time for the state machine to process events
    thread::sleep(Duration::from_secs(3));
    println!("Final state: {}", state_machine.get_current_state());
}
```

## Detailed Explanation

1. First, create a `SimpleEventListener` instance and event sender.
2. Use `JsonFileLoader` to load the state machine configuration from a JSON file.
3. Create a `StateMachine` instance and register relevant action functions.
4. Call `state_machine.run()` to start the state machine.
5. Use the event sender to simulate event sending.
6. The main thread waits for a period, allowing the state machine sufficient time to process all events.
7. Finally, print the state machine's final state.

This example demonstrates how to use the state machine library to manage a simple order process, from creation to payment, shipping, and delivery. Through configuration files and action function registration, you can easily customize the state machine's behavior to accommodate various complex business logics.

## Contributing

Issues and pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## License

[MIT](https://choosealicense.com/licenses/mit/)