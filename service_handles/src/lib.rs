/*
Author: 闫刚 (yes7rose@sina.com)
lib.rs (c) 2021
Desc: 服务处理定义
Created:  2021-02-14T09:31:29.747Z
Modified: !date!
*/

#[macro_use]
extern crate log;

pub mod area_service_handles;
pub mod cashmere;
// pub mod graph_service_handle;
pub mod name_handle_service;
// pub mod point_handle_service;

use tonic::{Response, Status};

pub type UnaryResponseResult<T> = Result<Response<T>, Status>;
pub type StreamResponseResult<T> = Result<Response<T>, Status>;

use cashmere::AreaLevel;
use cashmere::SlotType;

impl SlotType {
    // 整数到槽类型
    // 参考数据
    // RefrenceData = 0,
    // 依赖
    // DepedentData = 1,
    // 工作输出
    // WorkData = 2,
    // 输出
    // OutData = 3,
    pub fn from(val: &i32) -> Option<SlotType> {
        if val == &0i32 {
            Some(SlotType::RefrenceData)
        } else if val == &1i32 {
            Some(SlotType::DepedentData)
        } else if val == &2i32 {
            Some(SlotType::WorkData)
        } else if val == &3i32 {
            Some(SlotType::OutData)
        } else {
            None
        }
    }
}

impl AreaLevel {
    // 整数到区域类型
    // 国家级
    // Country = 0,
    // 省直辖市自治区
    // Province = 1,
    // 城市
    // City = 2,
    // 区
    // Area = 3,
    pub fn from(val: &i32) -> Option<AreaLevel> {
        if val == &0i32 {
            Some(AreaLevel::Country)
        } else if val == &1i32 {
            Some(AreaLevel::Province)
        } else if val == &2i32 {
            Some(AreaLevel::City)
        } else if val == &3i32 {
            Some(AreaLevel::Area)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
