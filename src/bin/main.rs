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

    // 注册动作函数
    state_machine.register_action("process_payment", || {
        println!("处理支付");
    });
    state_machine.register_action("send_shipping_notification", || {
        println!("发送发货通知");
    });
    state_machine.register_action("update_inventory", || {
        println!("更新库存");
    });
    state_machine.register_action("refund_payment", || {
        println!("退款");
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

    // 让主线程等待一段时间,以便观察状态机的运行
    thread::sleep(Duration::from_secs(6));

    println!("订单详情:");
    println!("  ID: {}", order.id);
    println!("  客户: {}", order.customer);
    println!("  金额: {:.2}", order.amount);
    println!("  最终状态: {}", order.state_machine.get_current_state());

    println!("程序结束");
    
}
