/*
Author: 闫刚 (yes7rose@sina.com)
mod.rs (c) 2021
Desc: 服务模块
Created:  2021-01-17T04:31:08.729Z
Modified: !date!
*/
use tonic::{Request, Response, Status, Streaming};
use manage_define::cashmere::*;

use core_service_handles::view_rules_service_handles::{
    HandleChangeCollectionReadrule, HandleChangeCollectionWriteRule, HandleChangeFieldReadrule,
    HandleChangeFieldWriteRule, HandleChangeManageWriteRule,
};
use core_service_handles::ResponseStream;
use core_service_handles::{
    area_service_handles::HandleEditArea,
    area_service_handles::HandleNewArea,
    country_service_handles::HandleNewCountry,
    data_service_handles::{HandleFileDataUploadFile, HandleNewData},
    entity_service_handles::{
        HandleEditEntityField, HandleEditEntityListFieldAddItems,
        HandleEditEntityListFieldRemoveItems, HandleEditEntityMapField,
        HandleEditEntityMapFieldRemoveKey, HandleGetDataList, HandleGetEntities,
        HandleGetEntitiesPage, HandleGetEntity,
    },
    group_service_handles::HandleNewGroup,
    language_code_handles::{HandleEditLanguageCode, HandleNewLanguageCode},
    manage_service_handle::{
        HandleEditSchemaFieldName, HandleGetManageEntryCount, HandleGetManageSchema,
        HandleGetManages, HandleMarkSchemaFieldRemoved,
    },
    name_service_handles::{HandleNewLanguageName, HandleRename},
    view_rules_service_handles::HandleChangeManageReadrule,
};

mod init;
mod project_servcice_handle;
mod assembly_servcice_handle;
mod assets_servcice_handle;
mod cut_servcice_handle;
mod epic_servcice_handle;
mod library_servcice_handle;
mod sets_servcice_handle;
mod sequence_servcice_handle;

pub mod protocol {
    include!("./io.knitter.rs");
}

// use protocol::{LoginRequest, LoginResponse, NewManageRequest, NewManageResponse};
use crate::services::protocol::knitter_grpc_server::KnitterGrpc;
use crate::services::protocol::{NewAssemblyRequest, NewAssemblyResponse, NewAssetRequest, NewAssetResponse, NewCutRequest, NewCutResponse, NewEpicRequest, NewEpicResponse, NewLibraryRequest, NewLibraryResponse, NewProjectRequest, NewProjectResponse, NewSequenceRequest, NewSequenceResponse, NewSetRequest, NewSetResponse};

/// 管理服务
#[derive(Default)]
pub struct KnitterServer;
// 组
impl HandleNewGroup for KnitterServer {}

// 国家
impl HandleNewCountry for KnitterServer {}

// 地区
impl HandleNewArea for KnitterServer {}
impl HandleEditArea for KnitterServer {}

// 语言
impl HandleNewLanguageCode for KnitterServer {}
impl HandleEditLanguageCode for KnitterServer {}

// 管理
impl HandleGetManages for KnitterServer {}
impl HandleGetManageSchema for KnitterServer {}
impl HandleMarkSchemaFieldRemoved for KnitterServer {}
impl HandleGetManageEntryCount for KnitterServer {}

// 可见性权限管理
impl HandleChangeManageReadrule for KnitterServer {}
impl HandleChangeManageWriteRule for KnitterServer {}
impl HandleChangeCollectionReadrule for KnitterServer {}
impl HandleChangeCollectionWriteRule for KnitterServer {}
impl HandleChangeFieldReadrule for KnitterServer {}
impl HandleChangeFieldWriteRule for KnitterServer {}

// 实体
impl HandleGetEntity for KnitterServer {}
impl HandleGetEntities for KnitterServer {}
impl HandleGetEntitiesPage for KnitterServer {}
impl HandleEditEntityField for KnitterServer {}
impl HandleEditEntityListFieldAddItems for KnitterServer {}
impl HandleEditEntityListFieldRemoveItems for KnitterServer {}
impl HandleEditEntityMapField for KnitterServer {}
impl HandleEditEntityMapFieldRemoveKey for KnitterServer {}

// 名字
impl HandleRename for KnitterServer {}
impl HandleNewLanguageName for KnitterServer {}

// 数据
impl HandleNewData for KnitterServer {}
impl HandleFileDataUploadFile for KnitterServer {}
impl HandleGetDataList for KnitterServer {}


#[tonic::async_trait]
impl KnitterGrpc for KnitterServer {
    async fn get_manages(
        &self,
        request: Request<GetManagesRequest>,
    ) -> Result<Response<GetManagesResponse>, Status> {
        self.handle_get_manages(request).await
    }

    async fn get_manage_entry_count(
        &self,
        request: Request<GetManageEntryCountRequest>,
    ) -> Result<Response<GetManageEntryCountResponse>, Status> {
        self.handle_get_manage_entry_count(request).await
    }

    async fn get_manage_schema(
        &self,
        request: Request<GetManageSchemaRequest>,
    ) -> Result<Response<GetManageSchemaResponse>, Status> {
        self.handle_get_manage_schema(request).await
    }

    async fn mark_schema_field_removed(
        &self,
        request: Request<MarkSchemaFieldRemovedRequest>,
    ) -> Result<Response<MarkSchemaFieldRemovedResponse>, Status> {
        self.mark_schema_field_removed(request).await
    }

    async fn change_manage_read_rule(
        &self,
        request: Request<ChangeManageReadRuleRequest>,
    ) -> Result<Response<ChangeManageReadRuleResponse>, Status> {
        self.handle_change_manage_read_rule(request).await
    }

    async fn change_manage_write_rule(
        &self,
        request: Request<ChangeManageWriteRuleRequest>,
    ) -> Result<Response<ChangeManageWriteRuleResponse>, Status> {
        self.handle_change_manage_write_rule(request).await
    }

    async fn change_collection_read_rule(
        &self,
        request: Request<ChangeCollectionReadRuleRequest>,
    ) -> Result<Response<ChangeCollectionReadRuleResponse>, Status> {
        self.handle_change_collection_read_rule(request).await
    }

    async fn change_collection_write_rule(
        &self,
        request: Request<ChangeCollectionWriteRuleRequest>,
    ) -> Result<Response<ChangeCollectionWriteRuleResponse>, Status> {
        self.handle_change_collection_write_rule(request).await
    }

    async fn change_field_read_rule(
        &self,
        request: Request<ChangeFieldReadRuleRequest>,
    ) -> Result<Response<ChangeFieldReadRuleResponse>, Status> {
        self.handle_change_field_read_rule(request).await
    }

    async fn change_field_write_rule(
        &self,
        request: Request<ChangeFieldWriteRuleRequest>,
    ) -> Result<Response<ChangeFieldWriteRuleResponse>, Status> {
        self.handle_change_field_write_rule(request).await
    }

    async fn get_entity(
        &self,
        request: Request<GetEntityRequest>,
    ) -> Result<Response<GetEntityResponse>, Status> {
        self.handle_get_entity(request).await
    }

    async fn get_entities(
        &self,
        request: Request<GetEntitiesRequest>,
    ) -> Result<Response<GetEntitiesResponse>, Status> {
        self.handle_get_entities(request).await
    }

    async fn get_entities_page(
        &self,
        request: Request<GetEntitiesPageRequest>,
    ) -> Result<Response<GetEntitiesPageResponse>, Status> {
        self.handle_get_entities_page(request).await
    }

    async fn edit_entity_field(
        &self,
        request: Request<EditEntityFieldRequest>,
    ) -> Result<Response<EditEntityFieldResponse>, Status> {
        self.handle_edit_entity_field(request).await
    }

    async fn edit_entity_map_field(
        &self,
        request: Request<::manage_define::cashmere::EditEntityMapFieldRequest>,
    ) -> Result<Response<::manage_define::cashmere::EditEntityMapFieldResponse>, Status> {
        self.handle_edit_entity_map_field(request).await
    }

    async fn edit_entity_map_field_remove_key(
        &self,
        request: Request<EditEntityMapFieldRemoveKeyRequest>,
    ) -> Result<Response<EditEntityMapFieldRemoveKeyResponse>, Status> {
        self.handle_edit_entity_map_field_remove_key(request).await
    }

    async fn edit_entity_list_field_add_items(
        &self,
        request: Request<EditEntityListFieldAddItemsRequest>,
    ) -> Result<Response<EditEntityListFieldAddItemsResponse>, Status> {
        self.handle_edit_entity_list_field_add_items(request).await
    }

    async fn edit_entity_list_field_remove_items(
        &self,
        request: Request<EditEntityListFieldRemoveItemsRequest>,
    ) -> Result<Response<EditEntityListFieldRemoveItemsResponse>, Status> {
        self.handle_edit_entity_list_field_remove_items(request)
            .await
    }

    async fn rename(
        &self,
        request: Request<RenameRequest>,
    ) -> Result<Response<RenameResponse>, Status> {
        self.handle_rename(request).await
    }

    async fn new_language_name(
        &self,
        request: Request<NewLanguageNameRequest>,
    ) -> Result<Response<NewLanguageNameResponse>, Status> {
        self.handle_new_language_name(request).await
    }

    async fn new_country(
        &self,
        request: Request<NewCountryRequest>,
    ) -> Result<Response<NewCountryResponse>, Status> {
        self.handle_new_country(request).await
    }

    // 语言编码
    async fn new_language_code(
        &self,
        request: Request<NewLanguageCodeRequest>,
    ) -> Result<Response<NewLanguageCodeResponse>, Status> {
        self.handle_new_language_code(request).await
    }

    async fn edit_language_code(
        &self,
        request: Request<EditLanguageCodeRequest>,
    ) -> Result<Response<EditLanguageCodeResponse>, Status> {
        self.handle_edit_language_code(request).await
    }

    async fn new_group(
        &self,
        request: Request<NewGroupRequest>,
    ) -> Result<Response<NewGroupResponse>, Status> {
        self.handle_new_group(request).await
    }

    async fn new_data(
        &self,
        request: Request<NewDataRequest>,
    ) -> Result<Response<NewDataResponse>, Status> {
        self.handle_new_data(request).await
    }
    async fn get_data_list(
        &self,
        request: Request<GetDataListRequest>,
    ) -> Result<Response<GetDataListResponse>, Status> {
        self.handle_get_data_list(request).await
    }

    async fn mark_data_removed(
        &self,
        _request: Request<MarkDataRemovedRequest>,
    ) -> Result<Response<MarkDataRemovedResponse>, Status> {
        todo!()
    }

    // 必须要有这个声明
    type FileDataUploadFileStream = ResponseStream<FileDataUploadFileResponse>;

    async fn file_data_upload_file(
        &self,
        request: Request<Streaming<FileDataUploadFileRequest>>,
    ) -> Result<Response<Self::FileDataUploadFileStream>, Status> {
        self.handle_file_data_upload_file(request).await
    }

    async fn new_project(&self, request: Request<NewProjectRequest>) -> Result<Response<NewProjectResponse>, Status> {
        self.handle_new_project(request).await
    }

    async fn new_library(&self, request: Request<NewLibraryRequest>) -> Result<Response<NewLibraryResponse>, Status> {
        self.handle_new_library(request).await
    }

    async fn new_asset(&self, request: Request<NewAssetRequest>) -> Result<Response<NewAssetResponse>, Status> {
        self.handle_new_asset(request).await
    }

    async fn new_assembly(&self, request: Request<NewAssemblyRequest>) -> Result<Response<NewAssemblyResponse>, Status> {
        self.handle_new_assembly(request).await
    }

    async fn new_epic(&self, request: Request<NewEpicRequest>) -> Result<Response<NewEpicResponse>, Status> {
        self.handle_new_epic(request).await
    }

    async fn new_sequence(&self, request: Request<NewSequenceRequest>) -> Result<Response<NewSequenceResponse>, Status> {
        self.handle_new_sequence(request).await
    }

    async fn new_cut(&self, request: Request<NewCutRequest>) -> Result<Response<NewCutResponse>, Status> {
        self.handle_new_cut(request).await
    }

    async fn new_set(&self, request: Request<NewSetRequest>) -> Result<Response<NewSetResponse>, Status> {
        self.handle_new_set(request).await
    }
}
