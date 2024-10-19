use rust_state_machine::{SimpleEventListener, StateMachine, JsonFileLoader};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Order {
    id: String,
    customer: String,
    amount: f64,
    state_machine: StateMachine,
}

impl Order {
    fn new(id: String, customer: String, amount: f64, state_machine: StateMachine) -> Self {
        Order {
            id,
            customer,
            amount,
            state_machine,
        }
    }
}

fn main() {
    let (event_listener, sender) = SimpleEventListener::new();
    let event_listener = Arc::new(Mutex::new(event_listener));
    
    let config_loader = JsonFileLoader::new("example-json.json".to_string());
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
    state_machine.register_action("refund_payment", || {
        println!("Refunding payment");
    });

    let order = Arc::new(Order::new(
        "ORD-001".to_string(),
        "John Doe".to_string(),
        100.0,
        state_machine,
    ));

    order.state_machine.run();

    thread::spawn(move || {
        sender.send(String::from("Pay")).unwrap();
        thread::sleep(Duration::from_secs(2));
        sender.send(String::from("Ship")).unwrap();
        thread::sleep(Duration::from_secs(2));
        sender.send(String::from("Deliver")).unwrap();
    });

    // Let the main thread wait for a while to observe the state machine running
    thread::sleep(Duration::from_secs(6));

    println!("Order details:");
    println!("  ID: {}", order.id);
    println!("  Customer: {}", order.customer);
    println!("  Amount: {:.2}", order.amount);
    println!("  Final state: {}", order.state_machine.get_current_state());

    println!("Program ended");
}
