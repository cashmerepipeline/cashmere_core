/*
Project: cashmere_server
Creator: 闫刚
Create time: 2020-12-28 13:33
Introduction:
*/

use bson::{Bson, doc};
use dependencies_sync::bson::{self, document};
use cash_core::field::ids::*;
use dependencies_sync::chrono::Utc;
use majordomo::get_majordomo;
use manage_define::manage_ids::*;
use managers::traits::ManagerTrait;
use dependencies_sync::tonic::{Request, Response, Status};

use crate::CashmereServer;
use crate::protocol::*;

impl CashmereServer {
    // 管理实体
    pub(crate) async fn handle_new_manage_entity(
        &self,
        request: Request<NewEntityRequest>,
    ) -> Result<Response<NewEntityResponse>, Status> {
        Err(Status::unimplemented("unimplemented"))
    }

    pub(crate) async fn handle_edit_manage_entity(
        &self,
        request: Request<EditEntityRequest>,
    ) -> Result<Response<EditEntityResponse>, Status> {
        Err(Status::unimplemented("unimplemented"))
    }

    pub(crate) async fn handle_remove_manage_entity(
        &self,
        request: Request<RemoveEntityRequest>,
    ) -> Result<Response<RemoveEntityResponse>, Status> {
        Err(Status::unimplemented("unimplemented"))
    }

    

    pub(crate) async fn handle_edit_entity_template(
        &self,
        request: Request<EditEntityTemplateRequest>,
    ) -> Result<Response<EditEntityTemplateResponse>, Status> {
        Err(Status::unimplemented("unimplemented"))
    }

    pub(crate) async fn handle_remove_entity_template(
        &self,
        request: Request<RemoveEntityTemplateRequest>,
    ) -> Result<Response<RemoveEntityTemplateResponse>, Status> {
        Err(Status::unimplemented("unimplemented"))
    }
}


