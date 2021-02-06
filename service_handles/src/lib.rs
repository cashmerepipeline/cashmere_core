pub mod account_handle_service;
pub mod account_service;

pub mod cashmere;
pub mod graph_service_handle;
pub mod name_handle_service;
pub mod point_handle_service;

use tonic::{Request, Response, Status};

type UnaryResponseResult<T> = Result<Response<T>, Status>;
type StreamResponseResult<T> = Result<Response<T>, Status>;

use cashmere::RoadmapVisType;
use cashmere::SlotType;

// 路线图可见类
impl RoadmapVisType {
    // 整数到RoadMapVisType
    // VisTypePerson = 0,
    // VisTypeClass = 1,
    // VisTypeGroup = 2,
    // VisTypeDepartment = 3,
    // VisTypeOrganization = 4,
    pub fn from(val: &i32) -> Option<RoadmapVisType> {
        if val == &0i32 {
            Some(RoadmapVisType::VisTypePerson)
        } else if val == &1i32 {
            Some(RoadmapVisType::VisTypeClass)
        } else if val == &2i32 {
            Some(RoadmapVisType::VisTypeGroup)
        } else if val == &3i32 {
            Some(RoadmapVisType::VisTypeDepartment)
        } else if val == &4i32 {
            Some(RoadmapVisType::VisTypeOrganization)
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
