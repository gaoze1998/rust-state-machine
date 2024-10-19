# Rust 状态机

这是一个用Rust实现的灵活、可配置且高效的状态机库。它提供了一种简单而强大的方式来管理复杂的状态转换逻辑。

## 主要特性

1. **配置驱动**: 通过JSON文件定义状态机(支持扩展配置文件读取方式)，使配置更加灵活和易于修改。

2. **高度可重用**: 核心状态机逻辑封装在库中，可在多个项目中复用。

3. **并发安全**: 利用Rust的并发原语确保线程安全操作。

4. **事件驱动**: 采用事件监听器模式，支持异步事件处理。

5. **强类型错误处理**: 使用Rust的Result类型进行健壮的错误处理。

6. **序列化支持**: 通过serde库实现配置的序列化和反序列化。

7. **自定义动作**: 允许注册自定义动作函数，在状态转换时执行。

8. **轻量级**: 最小化依赖，保持库的轻量性。

## 快速开始

### 1. 添加依赖

在你的`Cargo.toml`文件中添加以下依赖：

```toml
[dependencies]
rust-state-machine = { path = "path/to/rust-state-machine" }
```

### 2. 创建状态机配置

创建一个JSON文件（例如`state_machine_config.json`）来定义你的状态机：

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

### 3. 创建状态机

在你的Rust代码中创建一个状态机实例：

```rust
use rust_state_machine::{SimpleEventListener, StateMachine, JsonFileLoader};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
fn main() {
    // 创建事件监听器和发送器
    let (event_listener, sender) = SimpleEventListener::new();
    let event_listener = Arc::new(Mutex::new(event_listener));
    // 加载配置并创建状态机
    let config_loader = JsonFileLoader::new("state_machine_config.json".to_string());
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
    // 启动状态机
    state_machine.run();
    // 模拟事件发送
    thread::spawn(move || {
        sender.send(String::from("Pay")).unwrap();
        thread::sleep(Duration::from_secs(1));
        sender.send(String::from("Ship")).unwrap();
        thread::sleep(Duration::from_secs(1));
        sender.send(String::from("Deliver")).unwrap();
    });
    // 等待状态机处理事件
    thread::sleep(Duration::from_secs(3));
    println!("最终状态: {}", state_machine.get_current_state());
}
```

## 详细说明

1. 首先创建`SimpleEventListener`实例和事件发送器。
2. 使用`JsonFileLoader`从JSON文件加载状态机配置。
3. 创建`StateMachine`实例，并注册相关的动作函数。
4. 调用`state_machine.run()`启动状态机。
5. 使用事件发送器模拟事件的发送。
6. 主线程等待一段时间，让状态机有足够的时间处理所有事件。
7. 最后，打印状态机的最终状态。

这个示例展示了如何使用状态机库来管理一个简单的订单流程，从创建到支付、发货和交付。通过配置文件和动作函数的注册，你可以轻松地定制状态机的行为以适应各种复杂的业务逻辑。

## 贡献

欢迎提交问题和拉取请求。对于重大更改，请先开issue讨论您想要更改的内容。

## 许可证

[MIT](https://choosealicense.com/licenses/mit/)