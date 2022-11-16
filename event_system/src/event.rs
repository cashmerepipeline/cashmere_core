use crate::event_message::EventMessageField;

use super::event_echo_type::EventEchoType;

#[derive(Debug, Clone, PartialEq)]
pub struct Event {
    // 事件类型
    pub type_uid: String,
    // 发射者
    pub emitter_uid: String,
    pub serial_number: u64,
    // 发射时间
    pub timestamp: u64,
    // 过期时长, 单位为毫秒，0为不过期
    pub out_time: u32,
    // 是否需要反馈
    pub echo_type: EventEchoType, 
    // 附加信息
    pub messages: Vec<EventMessageField>
}
