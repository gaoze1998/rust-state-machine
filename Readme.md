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
     "initial_state": "Initial",
     "transitions": [
       {
         "event": "start",
         "from": "Initial",
         "to": "Processing"
       },
       {
         "event": "finish",
         "from": "Processing",
         "to": "Final"
       }
     ]
   }
   ```

3. 在代码中使用状态机

   ```rust
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
   ```

4. 运行你的程序

   ```
   cargo run
   ```

## 贡献

欢迎提交问题和拉取请求。对于重大更改,请先开issue讨论您想要更改的内容。

## 许可证

[MIT](https://choosealicense.com/licenses/mit/)

