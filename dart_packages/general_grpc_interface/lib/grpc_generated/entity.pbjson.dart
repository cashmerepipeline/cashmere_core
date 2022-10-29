///
//  Generated code. Do not modify.
//  source: entity.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields,deprecated_member_use_from_same_package

import 'dart:core' as $core;
import 'dart:convert' as $convert;
import 'dart:typed_data' as $typed_data;
@$core.Deprecated('Use entityDescriptor instead')
const Entity$json = const {
  '1': 'Entity',
  '2': const [
    const {'1': 'id', '3': 1, '4': 1, '5': 9, '10': 'id'},
    const {'1': 'name', '3': 2, '4': 1, '5': 11, '6': '.cashmere.Name', '10': 'name'},
    const {'1': 'creator_id', '3': 3, '4': 1, '5': 9, '10': 'creatorId'},
    const {'1': 'create_timestamp', '3': 4, '4': 1, '5': 9, '10': 'createTimestamp'},
    const {'1': 'modifier_id', '3': 5, '4': 1, '5': 9, '10': 'modifierId'},
    const {'1': 'modify_timestamp', '3': 6, '4': 1, '5': 9, '10': 'modifyTimestamp'},
    const {'1': 'owner_id', '3': 7, '4': 1, '5': 9, '10': 'ownerId'},
    const {'1': 'groups', '3': 8, '4': 3, '5': 9, '10': 'groups'},
    const {'1': 'data_ids', '3': 9, '4': 3, '5': 9, '10': 'dataIds'},
    const {'1': 'comment_ids', '3': 10, '4': 3, '5': 9, '10': 'commentIds'},
    const {'1': 'removed', '3': 11, '4': 1, '5': 8, '10': 'removed'},
    const {'1': 'removed_data_ids', '3': 12, '4': 3, '5': 9, '10': 'removedDataIds'},
  ],
};

/// Descriptor for `Entity`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List entityDescriptor = $convert.base64Decode('CgZFbnRpdHkSDgoCaWQYASABKAlSAmlkEiIKBG5hbWUYAiABKAsyDi5jYXNobWVyZS5OYW1lUgRuYW1lEh0KCmNyZWF0b3JfaWQYAyABKAlSCWNyZWF0b3JJZBIpChBjcmVhdGVfdGltZXN0YW1wGAQgASgJUg9jcmVhdGVUaW1lc3RhbXASHwoLbW9kaWZpZXJfaWQYBSABKAlSCm1vZGlmaWVySWQSKQoQbW9kaWZ5X3RpbWVzdGFtcBgGIAEoCVIPbW9kaWZ5VGltZXN0YW1wEhkKCG93bmVyX2lkGAcgASgJUgdvd25lcklkEhYKBmdyb3VwcxgIIAMoCVIGZ3JvdXBzEhkKCGRhdGFfaWRzGAkgAygJUgdkYXRhSWRzEh8KC2NvbW1lbnRfaWRzGAogAygJUgpjb21tZW50SWRzEhgKB3JlbW92ZWQYCyABKAhSB3JlbW92ZWQSKAoQcmVtb3ZlZF9kYXRhX2lkcxgMIAMoCVIOcmVtb3ZlZERhdGFJZHM=');
@$core.Deprecated('Use changeOwnerRequestDescriptor instead')
const ChangeOwnerRequest$json = const {
  '1': 'ChangeOwnerRequest',
  '2': const [
    const {'1': 'manage_id', '3': 1, '4': 1, '5': 5, '10': 'manageId'},
    const {'1': 'entity_id', '3': 2, '4': 1, '5': 9, '10': 'entityId'},
    const {'1': 'old_owner_id', '3': 3, '4': 1, '5': 9, '10': 'oldOwnerId'},
    const {'1': 'new_owner_id', '3': 4, '4': 1, '5': 9, '10': 'newOwnerId'},
  ],
};

/// Descriptor for `ChangeOwnerRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List changeOwnerRequestDescriptor = $convert.base64Decode('ChJDaGFuZ2VPd25lclJlcXVlc3QSGwoJbWFuYWdlX2lkGAEgASgFUghtYW5hZ2VJZBIbCgllbnRpdHlfaWQYAiABKAlSCGVudGl0eUlkEiAKDG9sZF9vd25lcl9pZBgDIAEoCVIKb2xkT3duZXJJZBIgCgxuZXdfb3duZXJfaWQYBCABKAlSCm5ld093bmVySWQ=');
@$core.Deprecated('Use changeOwnerResponseDescriptor instead')
const ChangeOwnerResponse$json = const {
  '1': 'ChangeOwnerResponse',
  '2': const [
    const {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `ChangeOwnerResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List changeOwnerResponseDescriptor = $convert.base64Decode('ChNDaGFuZ2VPd25lclJlc3BvbnNlEhYKBnJlc3VsdBgBIAEoCVIGcmVzdWx0');
@$core.Deprecated('Use newEntityRequestDescriptor instead')
const NewEntityRequest$json = const {
  '1': 'NewEntityRequest',
  '2': const [
    const {'1': 'manage_id', '3': 1, '4': 1, '5': 5, '10': 'manageId'},
    const {'1': 'data', '3': 2, '4': 1, '5': 12, '10': 'data'},
  ],
};

/// Descriptor for `NewEntityRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List newEntityRequestDescriptor = $convert.base64Decode('ChBOZXdFbnRpdHlSZXF1ZXN0EhsKCW1hbmFnZV9pZBgBIAEoBVIIbWFuYWdlSWQSEgoEZGF0YRgCIAEoDFIEZGF0YQ==');
@$core.Deprecated('Use newEntityResponseDescriptor instead')
const NewEntityResponse$json = const {
  '1': 'NewEntityResponse',
  '2': const [
    const {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `NewEntityResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List newEntityResponseDescriptor = $convert.base64Decode('ChFOZXdFbnRpdHlSZXNwb25zZRIWCgZyZXN1bHQYASABKAlSBnJlc3VsdA==');
@$core.Deprecated('Use editEntityRequestDescriptor instead')
const EditEntityRequest$json = const {
  '1': 'EditEntityRequest',
  '2': const [
    const {'1': 'manage_id', '3': 1, '4': 1, '5': 5, '10': 'manageId'},
    const {'1': 'entity_id', '3': 2, '4': 1, '5': 9, '10': 'entityId'},
    const {'1': 'data', '3': 3, '4': 1, '5': 12, '10': 'data'},
  ],
};

/// Descriptor for `EditEntityRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List editEntityRequestDescriptor = $convert.base64Decode('ChFFZGl0RW50aXR5UmVxdWVzdBIbCgltYW5hZ2VfaWQYASABKAVSCG1hbmFnZUlkEhsKCWVudGl0eV9pZBgCIAEoCVIIZW50aXR5SWQSEgoEZGF0YRgDIAEoDFIEZGF0YQ==');
@$core.Deprecated('Use editEntityResponseDescriptor instead')
const EditEntityResponse$json = const {
  '1': 'EditEntityResponse',
  '2': const [
    const {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `EditEntityResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List editEntityResponseDescriptor = $convert.base64Decode('ChJFZGl0RW50aXR5UmVzcG9uc2USFgoGcmVzdWx0GAEgASgJUgZyZXN1bHQ=');
@$core.Deprecated('Use editEntityFieldRequestDescriptor instead')
const EditEntityFieldRequest$json = const {
  '1': 'EditEntityFieldRequest',
  '2': const [
    const {'1': 'manage_id', '3': 1, '4': 1, '5': 5, '10': 'manageId'},
    const {'1': 'entity_id', '3': 2, '4': 1, '5': 9, '10': 'entityId'},
    const {'1': 'field_id', '3': 3, '4': 1, '5': 9, '10': 'fieldId'},
    const {'1': 'new_value', '3': 4, '4': 1, '5': 12, '10': 'newValue'},
  ],
};

/// Descriptor for `EditEntityFieldRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List editEntityFieldRequestDescriptor = $convert.base64Decode('ChZFZGl0RW50aXR5RmllbGRSZXF1ZXN0EhsKCW1hbmFnZV9pZBgBIAEoBVIIbWFuYWdlSWQSGwoJZW50aXR5X2lkGAIgASgJUghlbnRpdHlJZBIZCghmaWVsZF9pZBgDIAEoCVIHZmllbGRJZBIbCgluZXdfdmFsdWUYBCABKAxSCG5ld1ZhbHVl');
@$core.Deprecated('Use editEntityFieldResponseDescriptor instead')
const EditEntityFieldResponse$json = const {
  '1': 'EditEntityFieldResponse',
  '2': const [
    const {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `EditEntityFieldResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List editEntityFieldResponseDescriptor = $convert.base64Decode('ChdFZGl0RW50aXR5RmllbGRSZXNwb25zZRIWCgZyZXN1bHQYASABKAlSBnJlc3VsdA==');
@$core.Deprecated('Use editEntityMapFieldRequestDescriptor instead')
const EditEntityMapFieldRequest$json = const {
  '1': 'EditEntityMapFieldRequest',
  '2': const [
    const {'1': 'manage_id', '3': 1, '4': 1, '5': 5, '10': 'manageId'},
    const {'1': 'entity_id', '3': 2, '4': 1, '5': 9, '10': 'entityId'},
    const {'1': 'field_id', '3': 3, '4': 1, '5': 9, '10': 'fieldId'},
    const {'1': 'key', '3': 4, '4': 1, '5': 9, '10': 'key'},
    const {'1': 'new_value', '3': 5, '4': 1, '5': 12, '10': 'newValue'},
  ],
};

/// Descriptor for `EditEntityMapFieldRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List editEntityMapFieldRequestDescriptor = $convert.base64Decode('ChlFZGl0RW50aXR5TWFwRmllbGRSZXF1ZXN0EhsKCW1hbmFnZV9pZBgBIAEoBVIIbWFuYWdlSWQSGwoJZW50aXR5X2lkGAIgASgJUghlbnRpdHlJZBIZCghmaWVsZF9pZBgDIAEoCVIHZmllbGRJZBIQCgNrZXkYBCABKAlSA2tleRIbCgluZXdfdmFsdWUYBSABKAxSCG5ld1ZhbHVl');
@$core.Deprecated('Use editEntityMapFieldResponseDescriptor instead')
const EditEntityMapFieldResponse$json = const {
  '1': 'EditEntityMapFieldResponse',
  '2': const [
    const {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `EditEntityMapFieldResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List editEntityMapFieldResponseDescriptor = $convert.base64Decode('ChpFZGl0RW50aXR5TWFwRmllbGRSZXNwb25zZRIWCgZyZXN1bHQYASABKAlSBnJlc3VsdA==');
@$core.Deprecated('Use editEntityListFieldAddItemsRequestDescriptor instead')
const EditEntityListFieldAddItemsRequest$json = const {
  '1': 'EditEntityListFieldAddItemsRequest',
  '2': const [
    const {'1': 'manage_id', '3': 1, '4': 1, '5': 5, '10': 'manageId'},
    const {'1': 'entity_id', '3': 2, '4': 1, '5': 9, '10': 'entityId'},
    const {'1': 'field_id', '3': 3, '4': 1, '5': 9, '10': 'fieldId'},
    const {'1': 'items', '3': 4, '4': 1, '5': 12, '10': 'items'},
  ],
};

/// Descriptor for `EditEntityListFieldAddItemsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List editEntityListFieldAddItemsRequestDescriptor = $convert.base64Decode('CiJFZGl0RW50aXR5TGlzdEZpZWxkQWRkSXRlbXNSZXF1ZXN0EhsKCW1hbmFnZV9pZBgBIAEoBVIIbWFuYWdlSWQSGwoJZW50aXR5X2lkGAIgASgJUghlbnRpdHlJZBIZCghmaWVsZF9pZBgDIAEoCVIHZmllbGRJZBIUCgVpdGVtcxgEIAEoDFIFaXRlbXM=');
@$core.Deprecated('Use editEntityListFieldAddItemsResponseDescriptor instead')
const EditEntityListFieldAddItemsResponse$json = const {
  '1': 'EditEntityListFieldAddItemsResponse',
  '2': const [
    const {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `EditEntityListFieldAddItemsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List editEntityListFieldAddItemsResponseDescriptor = $convert.base64Decode('CiNFZGl0RW50aXR5TGlzdEZpZWxkQWRkSXRlbXNSZXNwb25zZRIWCgZyZXN1bHQYASABKAlSBnJlc3VsdA==');
@$core.Deprecated('Use editEntityListFieldRemoveItemsRequestDescriptor instead')
const EditEntityListFieldRemoveItemsRequest$json = const {
  '1': 'EditEntityListFieldRemoveItemsRequest',
  '2': const [
    const {'1': 'manage_id', '3': 1, '4': 1, '5': 5, '10': 'manageId'},
    const {'1': 'entity_id', '3': 2, '4': 1, '5': 9, '10': 'entityId'},
    const {'1': 'field_id', '3': 3, '4': 1, '5': 9, '10': 'fieldId'},
    const {'1': 'items', '3': 4, '4': 1, '5': 12, '10': 'items'},
  ],
};

/// Descriptor for `EditEntityListFieldRemoveItemsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List editEntityListFieldRemoveItemsRequestDescriptor = $convert.base64Decode('CiVFZGl0RW50aXR5TGlzdEZpZWxkUmVtb3ZlSXRlbXNSZXF1ZXN0EhsKCW1hbmFnZV9pZBgBIAEoBVIIbWFuYWdlSWQSGwoJZW50aXR5X2lkGAIgASgJUghlbnRpdHlJZBIZCghmaWVsZF9pZBgDIAEoCVIHZmllbGRJZBIUCgVpdGVtcxgEIAEoDFIFaXRlbXM=');
@$core.Deprecated('Use editEntityListFieldRemoveItemsResponseDescriptor instead')
const EditEntityListFieldRemoveItemsResponse$json = const {
  '1': 'EditEntityListFieldRemoveItemsResponse',
  '2': const [
    const {'1': 'result', '3': 1, '4': 1, '5': 12, '10': 'result'},
  ],
};

/// Descriptor for `EditEntityListFieldRemoveItemsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List editEntityListFieldRemoveItemsResponseDescriptor = $convert.base64Decode('CiZFZGl0RW50aXR5TGlzdEZpZWxkUmVtb3ZlSXRlbXNSZXNwb25zZRIWCgZyZXN1bHQYASABKAxSBnJlc3VsdA==');
@$core.Deprecated('Use getEntityRequestDescriptor instead')
const GetEntityRequest$json = const {
  '1': 'GetEntityRequest',
  '2': const [
    const {'1': 'manage_id', '3': 1, '4': 1, '5': 5, '10': 'manageId'},
    const {'1': 'entity_id', '3': 2, '4': 1, '5': 9, '10': 'entityId'},
  ],
};

/// Descriptor for `GetEntityRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getEntityRequestDescriptor = $convert.base64Decode('ChBHZXRFbnRpdHlSZXF1ZXN0EhsKCW1hbmFnZV9pZBgBIAEoBVIIbWFuYWdlSWQSGwoJZW50aXR5X2lkGAIgASgJUghlbnRpdHlJZA==');
@$core.Deprecated('Use getEntityResponseDescriptor instead')
const GetEntityResponse$json = const {
  '1': 'GetEntityResponse',
  '2': const [
    const {'1': 'entity', '3': 1, '4': 1, '5': 12, '10': 'entity'},
  ],
};

/// Descriptor for `GetEntityResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getEntityResponseDescriptor = $convert.base64Decode('ChFHZXRFbnRpdHlSZXNwb25zZRIWCgZlbnRpdHkYASABKAxSBmVudGl0eQ==');
@$core.Deprecated('Use getEntitiesRequestDescriptor instead')
const GetEntitiesRequest$json = const {
  '1': 'GetEntitiesRequest',
  '2': const [
    const {'1': 'manage_id', '3': 1, '4': 1, '5': 5, '10': 'manageId'},
    const {'1': 'entity_ids', '3': 2, '4': 3, '5': 9, '10': 'entityIds'},
  ],
};

/// Descriptor for `GetEntitiesRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getEntitiesRequestDescriptor = $convert.base64Decode('ChJHZXRFbnRpdGllc1JlcXVlc3QSGwoJbWFuYWdlX2lkGAEgASgFUghtYW5hZ2VJZBIdCgplbnRpdHlfaWRzGAIgAygJUgllbnRpdHlJZHM=');
@$core.Deprecated('Use getEntitiesResponseDescriptor instead')
const GetEntitiesResponse$json = const {
  '1': 'GetEntitiesResponse',
  '2': const [
    const {'1': 'entities', '3': 1, '4': 3, '5': 12, '10': 'entities'},
  ],
};

/// Descriptor for `GetEntitiesResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getEntitiesResponseDescriptor = $convert.base64Decode('ChNHZXRFbnRpdGllc1Jlc3BvbnNlEhoKCGVudGl0aWVzGAEgAygMUghlbnRpdGllcw==');
@$core.Deprecated('Use getEntitiesPageRequestDescriptor instead')
const GetEntitiesPageRequest$json = const {
  '1': 'GetEntitiesPageRequest',
  '2': const [
    const {'1': 'manage_id', '3': 1, '4': 1, '5': 5, '10': 'manageId'},
    const {'1': 'page_index', '3': 2, '4': 1, '5': 13, '10': 'pageIndex'},
    const {'1': 'conditions', '3': 3, '4': 1, '5': 12, '10': 'conditions'},
  ],
};

/// Descriptor for `GetEntitiesPageRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getEntitiesPageRequestDescriptor = $convert.base64Decode('ChZHZXRFbnRpdGllc1BhZ2VSZXF1ZXN0EhsKCW1hbmFnZV9pZBgBIAEoBVIIbWFuYWdlSWQSHQoKcGFnZV9pbmRleBgCIAEoDVIJcGFnZUluZGV4Eh4KCmNvbmRpdGlvbnMYAyABKAxSCmNvbmRpdGlvbnM=');
@$core.Deprecated('Use getEntitiesPageResponseDescriptor instead')
const GetEntitiesPageResponse$json = const {
  '1': 'GetEntitiesPageResponse',
  '2': const [
    const {'1': 'entities', '3': 1, '4': 3, '5': 12, '10': 'entities'},
  ],
};

/// Descriptor for `GetEntitiesPageResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getEntitiesPageResponseDescriptor = $convert.base64Decode('ChdHZXRFbnRpdGllc1BhZ2VSZXNwb25zZRIaCghlbnRpdGllcxgBIAMoDFIIZW50aXRpZXM=');
@$core.Deprecated('Use markEntityRemovedRequestDescriptor instead')
const MarkEntityRemovedRequest$json = const {
  '1': 'MarkEntityRemovedRequest',
  '2': const [
    const {'1': 'manage_id', '3': 1, '4': 1, '5': 9, '10': 'manageId'},
    const {'1': 'entity_id', '3': 2, '4': 1, '5': 9, '10': 'entityId'},
  ],
};

/// Descriptor for `MarkEntityRemovedRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List markEntityRemovedRequestDescriptor = $convert.base64Decode('ChhNYXJrRW50aXR5UmVtb3ZlZFJlcXVlc3QSGwoJbWFuYWdlX2lkGAEgASgJUghtYW5hZ2VJZBIbCgllbnRpdHlfaWQYAiABKAlSCGVudGl0eUlk');
@$core.Deprecated('Use markEntityRemovedResponseDescriptor instead')
const MarkEntityRemovedResponse$json = const {
  '1': 'MarkEntityRemovedResponse',
  '2': const [
    const {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `MarkEntityRemovedResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List markEntityRemovedResponseDescriptor = $convert.base64Decode('ChlNYXJrRW50aXR5UmVtb3ZlZFJlc3BvbnNlEhYKBnJlc3VsdBgBIAEoCVIGcmVzdWx0');
@$core.Deprecated('Use getDataListRequestDescriptor instead')
const GetDataListRequest$json = const {
  '1': 'GetDataListRequest',
  '2': const [
    const {'1': 'manage_id', '3': 1, '4': 1, '5': 5, '10': 'manageId'},
    const {'1': 'entity_id', '3': 2, '4': 1, '5': 9, '10': 'entityId'},
  ],
};

/// Descriptor for `GetDataListRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getDataListRequestDescriptor = $convert.base64Decode('ChJHZXREYXRhTGlzdFJlcXVlc3QSGwoJbWFuYWdlX2lkGAEgASgFUghtYW5hZ2VJZBIbCgllbnRpdHlfaWQYAiABKAlSCGVudGl0eUlk');
@$core.Deprecated('Use getDataListResponseDescriptor instead')
const GetDataListResponse$json = const {
  '1': 'GetDataListResponse',
  '2': const [
    const {'1': 'data_ids', '3': 1, '4': 3, '5': 9, '10': 'dataIds'},
  ],
};

/// Descriptor for `GetDataListResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getDataListResponseDescriptor = $convert.base64Decode('ChNHZXREYXRhTGlzdFJlc3BvbnNlEhkKCGRhdGFfaWRzGAEgAygJUgdkYXRhSWRz');
@$core.Deprecated('Use getRemovedDataListRequestDescriptor instead')
const GetRemovedDataListRequest$json = const {
  '1': 'GetRemovedDataListRequest',
  '2': const [
    const {'1': 'manage_id', '3': 1, '4': 1, '5': 9, '10': 'manageId'},
    const {'1': 'entity_id', '3': 2, '4': 1, '5': 9, '10': 'entityId'},
    const {'1': 'data_id', '3': 3, '4': 1, '5': 9, '10': 'dataId'},
  ],
};

/// Descriptor for `GetRemovedDataListRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getRemovedDataListRequestDescriptor = $convert.base64Decode('ChlHZXRSZW1vdmVkRGF0YUxpc3RSZXF1ZXN0EhsKCW1hbmFnZV9pZBgBIAEoCVIIbWFuYWdlSWQSGwoJZW50aXR5X2lkGAIgASgJUghlbnRpdHlJZBIXCgdkYXRhX2lkGAMgASgJUgZkYXRhSWQ=');
@$core.Deprecated('Use getRemovedDataListResponseDescriptor instead')
const GetRemovedDataListResponse$json = const {
  '1': 'GetRemovedDataListResponse',
  '2': const [
    const {'1': 'data_ids', '3': 1, '4': 3, '5': 9, '10': 'dataIds'},
  ],
};

/// Descriptor for `GetRemovedDataListResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getRemovedDataListResponseDescriptor = $convert.base64Decode('ChpHZXRSZW1vdmVkRGF0YUxpc3RSZXNwb25zZRIZCghkYXRhX2lkcxgBIAMoCVIHZGF0YUlkcw==');
