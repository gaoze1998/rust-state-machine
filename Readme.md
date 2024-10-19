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

   创建一个JSON文件(例如`example-json.json`)来定义你的状态机:

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
           },
           {
               "event": "Cancel",
               "from": "Created",
               "to": "Cancelled",
               "action": "refund_payment"
           },
           {
               "event": "Cancel",
               "from": "Paid",
               "to": "Cancelled",
               "action": "refund_payment"
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
   ```

4. 运行你的程序

   ```
   cargo run
   ```

### 详细说明

1. 首先，我们创建了一个`SimpleEventListener`实例和一个事件发送器。

2. 然后，我们从JSON文件加载状态机配置。

3. 我们注册了与状态转换相关的动作函数。这些函数将在相应的状态转换发生时被调用。

4. 创建一个`Order`实例，其中包含状态机。

5. 调用`state_machine.run()`启动状态机。

6. 在一个新线程中，我们模拟了一系列事件的发送（Pay、Ship、Deliver）。

7. 主线程等待6秒，让状态机有足够的时间处理所有事件。

8. 最后，我们打印订单详情，包括最终状态。

这个示例展示了如何使用状态机来管理订单的生命周期，从创建到支付、发货和交付。通过配置文件和动作函数的注册，我们可以轻松地定制状态机的行为。

## 贡献

欢迎提交问题和拉取请求。对于重大更改,请先开issue讨论您想要更改的内容。

## 许可证

[MIT](https://choosealicense.com/licenses/mit/)
