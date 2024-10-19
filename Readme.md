# Rust 状态机

这是一个用Rust实现的灵活且可配置的状态机库。

## 项目优势

1. **灵活性**: 通过JSON配置文件定义状态机,易于修改和扩展。

2. **可重用性**: 核心状态机逻辑封装在库中,可以在多个项目中重复使用。

3. **线程安全**: 使用Rust的并发原语确保线程安全。

4. **事件驱动**: 采用事件监听器模式,支持异步事件处理。

5. **错误处理**: 使用Rust的Result类型进行健壮的错误处理。

6. **序列化支持**: 利用serde库实现配置的序列化和反序列化。

## 快速使用指南

1. 添加依赖
   
   在你的`Cargo.toml`文件中添加以下依赖:

   ```toml
   [dependencies]
   rust-state-machine = { path = "path/to/rust-state-machine" }
   ```

2. 创建状态机配置文件

   创建一个JSON文件(例如`config.json`)来定义你的状态机:

   ```json
   {
        "initial_state": "Created",
        "transitions": [
            {
                "event": "Pay",
                "from": "Created",
                "to": "Paid"
            },
            {
                "event": "Ship",
                "from": "Paid",
                "to": "Shipped"
            },
            {
                "event": "Deliver",
                "from": "Shipped",
                "to": "Delivered"
            },
            {
                "event": "Cancel",
                "from": "Created",
                "to": "Cancelled"
            },
            {
                "event": "Cancel",
                "from": "Paid",
                "to": "Cancelled"
            }
        ]
   }
   ```

3. 在代码中使用状态机

   ```rust
    use rust_state_machine::{SimpleEventListener, StateMachine};
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

        let state_machine = StateMachine::load_from_file("example-json.json", event_listener.clone())
            .expect("Failed to load state machine configuration");

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

        println!("订单 {} 的最终状态: {}", order.id, order.state_machine.get_current_state());
        println!("程序结束");
    }

   ```

4. 运行你的程序

   ```
   cargo run
   ```

### 详细说明

#### 状态机配置

状态机通过JSON文件进行配置。配置文件包含以下字段:

- `initial_state`: 初始状态
- `transitions`: 状态转换数组,每个转换包含:
  - `event`: 触发事件
  - `from`: 起始状态
  - `to`: 目标状态

#### 核心组件

1. `StateMachine`: 主要的状态机结构,负责管理状态和处理事件。

2. `EventListener`: 事件监听器trait,用于接收事件。

3. `SimpleEventListener`: `EventListener`的一个简单实现,使用channel进行事件传递。

## 贡献

欢迎提交问题和拉取请求。对于重大更改,请先开issue讨论您想要更改的内容。

## 许可证

[MIT](https://choosealicense.com/licenses/mit/)
