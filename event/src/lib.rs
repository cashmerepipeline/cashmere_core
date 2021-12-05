/*
Author: 闫刚 (yes7rose@sina.com)
lib.rs (c) 2020
Desc: 绒事件系统
Created:  2020-11-05T04:06:45.116Z
Modified: !date!
*/

//线程数限制
#![recursion_limit="256"]

pub mod event;
pub mod handle;
pub mod queue;
pub mod queues_map;
pub mod events_map;
pub mod handles_map;
// mod queque_recievers;

// 事件队列
// 用sled数据库作为后台
// tokio作为异步处理平台

/* 
一种事件可能关联多个事件队列 1:N
一个队列可关联多个类型事件 N:1
一个队列可关联多个事件处理器 1:N
一个事件处理器只处理一种事件，多个处理器可能处理同一种事件，事件和事件处理不直接关联
*/




use event::Event;


pub async fn emit_event(_e: Event) {
    unimplemented!() // TODO
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
