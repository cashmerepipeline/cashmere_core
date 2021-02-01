pub mod account_handle_service;
pub mod account_service;

pub mod cashmere;
pub mod common_handle_service;
pub mod graph_service_handle;
pub mod point_handle_service;

use tonic::{Request, Response, Status};

type UnaryResponseResult<T> = Result<Response<T>, Status>;
type StreamResponseResult<T> = Result<Response<T>, Status>;

use cashmere::RoadMapVisType;
use cashmere::SlotType;

impl RoadMapVisType {
    // 整数到RoadMapVisType
    // VisTypePerson = 0,
    // VisTypeClass = 1,
    // VisTypeGroup = 2,
    // VisTypeDepartment = 3,
    // VisTypeOrganization = 4,
    pub fn from(val: &i32) -> Option<RoadMapVisType> {
        if val == &0i32 {
            Some(RoadMapVisType::VisTypePerson)
        } else if val == &1i32 {
            Some(RoadMapVisType::VisTypeClass)
        } else if val == &2i32 {
            Some(RoadMapVisType::VisTypeGroup)
        } else if val == &3i32 {
            Some(RoadMapVisType::VisTypeDepartment)
        } else if val == &4i32 {
            Some(RoadMapVisType::VisTypeOrganization)
        } else {
            None
        }
    }
}

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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
