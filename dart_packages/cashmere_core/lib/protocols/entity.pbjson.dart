//
//  Generated code. Do not modify.
//  source: entity.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use entityDescriptor instead')
const Entity$json = {
  '1': 'Entity',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 9, '10': 'id'},
    {'1': 'name', '3': 2, '4': 1, '5': 11, '6': '.cashmere.Name', '10': 'name'},
    {'1': 'creator_id', '3': 3, '4': 1, '5': 9, '10': 'creatorId'},
    {'1': 'create_timestamp', '3': 4, '4': 1, '5': 9, '10': 'createTimestamp'},
    {'1': 'modifier_id', '3': 5, '4': 1, '5': 9, '10': 'modifierId'},
    {'1': 'modify_timestamp', '3': 6, '4': 1, '5': 9, '10': 'modifyTimestamp'},
    {'1': 'owner_id', '3': 7, '4': 1, '5': 9, '10': 'ownerId'},
    {'1': 'groups', '3': 8, '4': 3, '5': 9, '10': 'groups'},
    {'1': 'data_ids', '3': 9, '4': 3, '5': 9, '10': 'dataIds'},
    {'1': 'comment_ids', '3': 10, '4': 3, '5': 9, '10': 'commentIds'},
    {'1': 'removed', '3': 11, '4': 1, '5': 8, '10': 'removed'},
    {'1': 'removed_data_ids', '3': 12, '4': 3, '5': 9, '10': 'removedDataIds'},
  ],
};

/// Descriptor for `Entity`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List entityDescriptor = $convert.base64Decode(
    'CgZFbnRpdHkSDgoCaWQYASABKAlSAmlkEiIKBG5hbWUYAiABKAsyDi5jYXNobWVyZS5OYW1lUg'
    'RuYW1lEh0KCmNyZWF0b3JfaWQYAyABKAlSCWNyZWF0b3JJZBIpChBjcmVhdGVfdGltZXN0YW1w'
    'GAQgASgJUg9jcmVhdGVUaW1lc3RhbXASHwoLbW9kaWZpZXJfaWQYBSABKAlSCm1vZGlmaWVySW'
    'QSKQoQbW9kaWZ5X3RpbWVzdGFtcBgGIAEoCVIPbW9kaWZ5VGltZXN0YW1wEhkKCG93bmVyX2lk'
    'GAcgASgJUgdvd25lcklkEhYKBmdyb3VwcxgIIAMoCVIGZ3JvdXBzEhkKCGRhdGFfaWRzGAkgAy'
    'gJUgdkYXRhSWRzEh8KC2NvbW1lbnRfaWRzGAogAygJUgpjb21tZW50SWRzEhgKB3JlbW92ZWQY'
    'CyABKAhSB3JlbW92ZWQSKAoQcmVtb3ZlZF9kYXRhX2lkcxgMIAMoCVIOcmVtb3ZlZERhdGFJZH'
    'M=');

@$core.Deprecated('Use changeEntityOwnerRequestDescriptor instead')
const ChangeEntityOwnerRequest$json = {
  '1': 'ChangeEntityOwnerRequest',
  '2': [
    {'1': 'manage_id', '3': 1, '4': 1, '5': 5, '10': 'manageId'},
    {'1': 'entity_id', '3': 2, '4': 1, '5': 9, '10': 'entityId'},
    {'1': 'old_owner_id', '3': 3, '4': 1, '5': 9, '10': 'oldOwnerId'},
    {'1': 'new_owner_id', '3': 4, '4': 1, '5': 9, '10': 'newOwnerId'},
  ],
};

/// Descriptor for `ChangeEntityOwnerRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List changeEntityOwnerRequestDescriptor = $convert.base64Decode(
    'ChhDaGFuZ2VFbnRpdHlPd25lclJlcXVlc3QSGwoJbWFuYWdlX2lkGAEgASgFUghtYW5hZ2VJZB'
    'IbCgllbnRpdHlfaWQYAiABKAlSCGVudGl0eUlkEiAKDG9sZF9vd25lcl9pZBgDIAEoCVIKb2xk'
    'T3duZXJJZBIgCgxuZXdfb3duZXJfaWQYBCABKAlSCm5ld093bmVySWQ=');

@$core.Deprecated('Use changeEntityOwnerResponseDescriptor instead')
const ChangeEntityOwnerResponse$json = {
  '1': 'ChangeEntityOwnerResponse',
  '2': [
    {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `ChangeEntityOwnerResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List changeEntityOwnerResponseDescriptor = $convert.base64Decode(
    'ChlDaGFuZ2VFbnRpdHlPd25lclJlc3BvbnNlEhYKBnJlc3VsdBgBIAEoCVIGcmVzdWx0');

@$core.Deprecated('Use newEntityRequestDescriptor instead')
const NewEntityRequest$json = {
  '1': 'NewEntityRequest',
  '2': [
    {'1': 'manage_id', '3': 1, '4': 1, '5': 5, '10': 'manageId'},
    {'1': 'data', '3': 2, '4': 1, '5': 12, '10': 'data'},
  ],
};

/// Descriptor for `NewEntityRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List newEntityRequestDescriptor = $convert.base64Decode(
    'ChBOZXdFbnRpdHlSZXF1ZXN0EhsKCW1hbmFnZV9pZBgBIAEoBVIIbWFuYWdlSWQSEgoEZGF0YR'
    'gCIAEoDFIEZGF0YQ==');

@$core.Deprecated('Use newEntityResponseDescriptor instead')
const NewEntityResponse$json = {
  '1': 'NewEntityResponse',
  '2': [
    {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `NewEntityResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List newEntityResponseDescriptor = $convert.base64Decode(
    'ChFOZXdFbnRpdHlSZXNwb25zZRIWCgZyZXN1bHQYASABKAlSBnJlc3VsdA==');

@$core.Deprecated('Use editEntityRequestDescriptor instead')
const EditEntityRequest$json = {
  '1': 'EditEntityRequest',
  '2': [
    {'1': 'manage_id', '3': 1, '4': 1, '5': 5, '10': 'manageId'},
    {'1': 'entity_id', '3': 2, '4': 1, '5': 9, '10': 'entityId'},
    {'1': 'data', '3': 3, '4': 1, '5': 12, '10': 'data'},
  ],
};

/// Descriptor for `EditEntityRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List editEntityRequestDescriptor = $convert.base64Decode(
    'ChFFZGl0RW50aXR5UmVxdWVzdBIbCgltYW5hZ2VfaWQYASABKAVSCG1hbmFnZUlkEhsKCWVudG'
    'l0eV9pZBgCIAEoCVIIZW50aXR5SWQSEgoEZGF0YRgDIAEoDFIEZGF0YQ==');

@$core.Deprecated('Use editEntityResponseDescriptor instead')
const EditEntityResponse$json = {
  '1': 'EditEntityResponse',
  '2': [
    {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `EditEntityResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List editEntityResponseDescriptor = $convert.base64Decode(
    'ChJFZGl0RW50aXR5UmVzcG9uc2USFgoGcmVzdWx0GAEgASgJUgZyZXN1bHQ=');

@$core.Deprecated('Use editEntityFieldRequestDescriptor instead')
const EditEntityFieldRequest$json = {
  '1': 'EditEntityFieldRequest',
  '2': [
    {'1': 'manage_id', '3': 1, '4': 1, '5': 5, '10': 'manageId'},
    {'1': 'entity_id', '3': 2, '4': 1, '5': 9, '10': 'entityId'},
    {'1': 'field_id', '3': 3, '4': 1, '5': 9, '10': 'fieldId'},
    {'1': 'new_value', '3': 4, '4': 1, '5': 12, '10': 'newValue'},
  ],
};

/// Descriptor for `EditEntityFieldRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List editEntityFieldRequestDescriptor = $convert.base64Decode(
    'ChZFZGl0RW50aXR5RmllbGRSZXF1ZXN0EhsKCW1hbmFnZV9pZBgBIAEoBVIIbWFuYWdlSWQSGw'
    'oJZW50aXR5X2lkGAIgASgJUghlbnRpdHlJZBIZCghmaWVsZF9pZBgDIAEoCVIHZmllbGRJZBIb'
    'CgluZXdfdmFsdWUYBCABKAxSCG5ld1ZhbHVl');

@$core.Deprecated('Use editEntityFieldResponseDescriptor instead')
const EditEntityFieldResponse$json = {
  '1': 'EditEntityFieldResponse',
  '2': [
    {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `EditEntityFieldResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List editEntityFieldResponseDescriptor = $convert.base64Decode(
    'ChdFZGl0RW50aXR5RmllbGRSZXNwb25zZRIWCgZyZXN1bHQYASABKAlSBnJlc3VsdA==');

@$core.Deprecated('Use editEntityMapFieldRequestDescriptor instead')
const EditEntityMapFieldRequest$json = {
  '1': 'EditEntityMapFieldRequest',
  '2': [
    {'1': 'manage_id', '3': 1, '4': 1, '5': 5, '10': 'manageId'},
    {'1': 'entity_id', '3': 2, '4': 1, '5': 9, '10': 'entityId'},
    {'1': 'field_id', '3': 3, '4': 1, '5': 9, '10': 'fieldId'},
    {'1': 'key', '3': 4, '4': 1, '5': 9, '10': 'key'},
    {'1': 'new_value', '3': 5, '4': 1, '5': 12, '10': 'newValue'},
  ],
};

/// Descriptor for `EditEntityMapFieldRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List editEntityMapFieldRequestDescriptor = $convert.base64Decode(
    'ChlFZGl0RW50aXR5TWFwRmllbGRSZXF1ZXN0EhsKCW1hbmFnZV9pZBgBIAEoBVIIbWFuYWdlSW'
    'QSGwoJZW50aXR5X2lkGAIgASgJUghlbnRpdHlJZBIZCghmaWVsZF9pZBgDIAEoCVIHZmllbGRJ'
    'ZBIQCgNrZXkYBCABKAlSA2tleRIbCgluZXdfdmFsdWUYBSABKAxSCG5ld1ZhbHVl');

@$core.Deprecated('Use editEntityMapFieldResponseDescriptor instead')
const EditEntityMapFieldResponse$json = {
  '1': 'EditEntityMapFieldResponse',
  '2': [
    {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `EditEntityMapFieldResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List editEntityMapFieldResponseDescriptor = $convert.base64Decode(
    'ChpFZGl0RW50aXR5TWFwRmllbGRSZXNwb25zZRIWCgZyZXN1bHQYASABKAlSBnJlc3VsdA==');

@$core.Deprecated('Use editEntityMapFieldRemoveKeyRequestDescriptor instead')
const EditEntityMapFieldRemoveKeyRequest$json = {
  '1': 'EditEntityMapFieldRemoveKeyRequest',
  '2': [
    {'1': 'manage_id', '3': 1, '4': 1, '5': 5, '10': 'manageId'},
    {'1': 'entity_id', '3': 2, '4': 1, '5': 9, '10': 'entityId'},
    {'1': 'field_id', '3': 3, '4': 1, '5': 9, '10': 'fieldId'},
    {'1': 'key', '3': 4, '4': 1, '5': 9, '10': 'key'},
  ],
};

/// Descriptor for `EditEntityMapFieldRemoveKeyRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List editEntityMapFieldRemoveKeyRequestDescriptor = $convert.base64Decode(
    'CiJFZGl0RW50aXR5TWFwRmllbGRSZW1vdmVLZXlSZXF1ZXN0EhsKCW1hbmFnZV9pZBgBIAEoBV'
    'IIbWFuYWdlSWQSGwoJZW50aXR5X2lkGAIgASgJUghlbnRpdHlJZBIZCghmaWVsZF9pZBgDIAEo'
    'CVIHZmllbGRJZBIQCgNrZXkYBCABKAlSA2tleQ==');

@$core.Deprecated('Use editEntityMapFieldRemoveKeyResponseDescriptor instead')
const EditEntityMapFieldRemoveKeyResponse$json = {
  '1': 'EditEntityMapFieldRemoveKeyResponse',
  '2': [
    {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `EditEntityMapFieldRemoveKeyResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List editEntityMapFieldRemoveKeyResponseDescriptor = $convert.base64Decode(
    'CiNFZGl0RW50aXR5TWFwRmllbGRSZW1vdmVLZXlSZXNwb25zZRIWCgZyZXN1bHQYASABKAlSBn'
    'Jlc3VsdA==');

@$core.Deprecated('Use editEntityArrayFieldAddItemsRequestDescriptor instead')
const EditEntityArrayFieldAddItemsRequest$json = {
  '1': 'EditEntityArrayFieldAddItemsRequest',
  '2': [
    {'1': 'manage_id', '3': 1, '4': 1, '5': 5, '10': 'manageId'},
    {'1': 'entity_id', '3': 2, '4': 1, '5': 9, '10': 'entityId'},
    {'1': 'field_id', '3': 3, '4': 1, '5': 9, '10': 'fieldId'},
    {'1': 'items', '3': 4, '4': 1, '5': 12, '10': 'items'},
  ],
};

/// Descriptor for `EditEntityArrayFieldAddItemsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List editEntityArrayFieldAddItemsRequestDescriptor = $convert.base64Decode(
    'CiNFZGl0RW50aXR5QXJyYXlGaWVsZEFkZEl0ZW1zUmVxdWVzdBIbCgltYW5hZ2VfaWQYASABKA'
    'VSCG1hbmFnZUlkEhsKCWVudGl0eV9pZBgCIAEoCVIIZW50aXR5SWQSGQoIZmllbGRfaWQYAyAB'
    'KAlSB2ZpZWxkSWQSFAoFaXRlbXMYBCABKAxSBWl0ZW1z');

@$core.Deprecated('Use editEntityArrayFieldAddItemsResponseDescriptor instead')
const EditEntityArrayFieldAddItemsResponse$json = {
  '1': 'EditEntityArrayFieldAddItemsResponse',
  '2': [
    {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `EditEntityArrayFieldAddItemsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List editEntityArrayFieldAddItemsResponseDescriptor = $convert.base64Decode(
    'CiRFZGl0RW50aXR5QXJyYXlGaWVsZEFkZEl0ZW1zUmVzcG9uc2USFgoGcmVzdWx0GAEgASgJUg'
    'ZyZXN1bHQ=');

@$core.Deprecated('Use editEntityArrayFieldRemoveItemsRequestDescriptor instead')
const EditEntityArrayFieldRemoveItemsRequest$json = {
  '1': 'EditEntityArrayFieldRemoveItemsRequest',
  '2': [
    {'1': 'manage_id', '3': 1, '4': 1, '5': 5, '10': 'manageId'},
    {'1': 'entity_id', '3': 2, '4': 1, '5': 9, '10': 'entityId'},
    {'1': 'field_id', '3': 3, '4': 1, '5': 9, '10': 'fieldId'},
    {'1': 'items', '3': 4, '4': 1, '5': 12, '10': 'items'},
  ],
};

/// Descriptor for `EditEntityArrayFieldRemoveItemsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List editEntityArrayFieldRemoveItemsRequestDescriptor = $convert.base64Decode(
    'CiZFZGl0RW50aXR5QXJyYXlGaWVsZFJlbW92ZUl0ZW1zUmVxdWVzdBIbCgltYW5hZ2VfaWQYAS'
    'ABKAVSCG1hbmFnZUlkEhsKCWVudGl0eV9pZBgCIAEoCVIIZW50aXR5SWQSGQoIZmllbGRfaWQY'
    'AyABKAlSB2ZpZWxkSWQSFAoFaXRlbXMYBCABKAxSBWl0ZW1z');

@$core.Deprecated('Use editEntityArrayFieldRemoveItemsResponseDescriptor instead')
const EditEntityArrayFieldRemoveItemsResponse$json = {
  '1': 'EditEntityArrayFieldRemoveItemsResponse',
  '2': [
    {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `EditEntityArrayFieldRemoveItemsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List editEntityArrayFieldRemoveItemsResponseDescriptor = $convert.base64Decode(
    'CidFZGl0RW50aXR5QXJyYXlGaWVsZFJlbW92ZUl0ZW1zUmVzcG9uc2USFgoGcmVzdWx0GAEgAS'
    'gJUgZyZXN1bHQ=');

@$core.Deprecated('Use getEntityRequestDescriptor instead')
const GetEntityRequest$json = {
  '1': 'GetEntityRequest',
  '2': [
    {'1': 'manage_id', '3': 1, '4': 1, '5': 5, '10': 'manageId'},
    {'1': 'entity_id', '3': 2, '4': 1, '5': 9, '10': 'entityId'},
  ],
};

/// Descriptor for `GetEntityRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getEntityRequestDescriptor = $convert.base64Decode(
    'ChBHZXRFbnRpdHlSZXF1ZXN0EhsKCW1hbmFnZV9pZBgBIAEoBVIIbWFuYWdlSWQSGwoJZW50aX'
    'R5X2lkGAIgASgJUghlbnRpdHlJZA==');

@$core.Deprecated('Use getEntityResponseDescriptor instead')
const GetEntityResponse$json = {
  '1': 'GetEntityResponse',
  '2': [
    {'1': 'entity', '3': 1, '4': 1, '5': 12, '10': 'entity'},
  ],
};

/// Descriptor for `GetEntityResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getEntityResponseDescriptor = $convert.base64Decode(
    'ChFHZXRFbnRpdHlSZXNwb25zZRIWCgZlbnRpdHkYASABKAxSBmVudGl0eQ==');

@$core.Deprecated('Use getEntitiesRequestDescriptor instead')
const GetEntitiesRequest$json = {
  '1': 'GetEntitiesRequest',
  '2': [
    {'1': 'manage_id', '3': 1, '4': 1, '5': 5, '10': 'manageId'},
    {'1': 'entity_ids', '3': 2, '4': 3, '5': 9, '10': 'entityIds'},
  ],
};

/// Descriptor for `GetEntitiesRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getEntitiesRequestDescriptor = $convert.base64Decode(
    'ChJHZXRFbnRpdGllc1JlcXVlc3QSGwoJbWFuYWdlX2lkGAEgASgFUghtYW5hZ2VJZBIdCgplbn'
    'RpdHlfaWRzGAIgAygJUgllbnRpdHlJZHM=');

@$core.Deprecated('Use getEntitiesResponseDescriptor instead')
const GetEntitiesResponse$json = {
  '1': 'GetEntitiesResponse',
  '2': [
    {'1': 'entities', '3': 1, '4': 3, '5': 12, '10': 'entities'},
  ],
};

/// Descriptor for `GetEntitiesResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getEntitiesResponseDescriptor = $convert.base64Decode(
    'ChNHZXRFbnRpdGllc1Jlc3BvbnNlEhoKCGVudGl0aWVzGAEgAygMUghlbnRpdGllcw==');

@$core.Deprecated('Use getEntitiesPageRequestDescriptor instead')
const GetEntitiesPageRequest$json = {
  '1': 'GetEntitiesPageRequest',
  '2': [
    {'1': 'manage_id', '3': 1, '4': 1, '5': 5, '10': 'manageId'},
    {'1': 'page_index', '3': 2, '4': 1, '5': 13, '10': 'pageIndex'},
    {'1': 'conditions', '3': 3, '4': 1, '5': 12, '10': 'conditions'},
  ],
};

/// Descriptor for `GetEntitiesPageRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getEntitiesPageRequestDescriptor = $convert.base64Decode(
    'ChZHZXRFbnRpdGllc1BhZ2VSZXF1ZXN0EhsKCW1hbmFnZV9pZBgBIAEoBVIIbWFuYWdlSWQSHQ'
    'oKcGFnZV9pbmRleBgCIAEoDVIJcGFnZUluZGV4Eh4KCmNvbmRpdGlvbnMYAyABKAxSCmNvbmRp'
    'dGlvbnM=');

@$core.Deprecated('Use getEntitiesPageResponseDescriptor instead')
const GetEntitiesPageResponse$json = {
  '1': 'GetEntitiesPageResponse',
  '2': [
    {'1': 'entities', '3': 1, '4': 3, '5': 12, '10': 'entities'},
  ],
};

/// Descriptor for `GetEntitiesPageResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getEntitiesPageResponseDescriptor = $convert.base64Decode(
    'ChdHZXRFbnRpdGllc1BhZ2VSZXNwb25zZRIaCghlbnRpdGllcxgBIAMoDFIIZW50aXRpZXM=');

@$core.Deprecated('Use markEntityRemovedRequestDescriptor instead')
const MarkEntityRemovedRequest$json = {
  '1': 'MarkEntityRemovedRequest',
  '2': [
    {'1': 'manage_id', '3': 1, '4': 1, '5': 5, '10': 'manageId'},
    {'1': 'entity_id', '3': 2, '4': 1, '5': 9, '10': 'entityId'},
  ],
};

/// Descriptor for `MarkEntityRemovedRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List markEntityRemovedRequestDescriptor = $convert.base64Decode(
    'ChhNYXJrRW50aXR5UmVtb3ZlZFJlcXVlc3QSGwoJbWFuYWdlX2lkGAEgASgFUghtYW5hZ2VJZB'
    'IbCgllbnRpdHlfaWQYAiABKAlSCGVudGl0eUlk');

@$core.Deprecated('Use markEntityRemovedResponseDescriptor instead')
const MarkEntityRemovedResponse$json = {
  '1': 'MarkEntityRemovedResponse',
  '2': [
    {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `MarkEntityRemovedResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List markEntityRemovedResponseDescriptor = $convert.base64Decode(
    'ChlNYXJrRW50aXR5UmVtb3ZlZFJlc3BvbnNlEhYKBnJlc3VsdBgBIAEoCVIGcmVzdWx0');

@$core.Deprecated('Use recoverRemovedEntityRequestDescriptor instead')
const RecoverRemovedEntityRequest$json = {
  '1': 'RecoverRemovedEntityRequest',
  '2': [
    {'1': 'manage_id', '3': 1, '4': 1, '5': 5, '10': 'manageId'},
    {'1': 'entity_id', '3': 2, '4': 1, '5': 9, '10': 'entityId'},
  ],
};

/// Descriptor for `RecoverRemovedEntityRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List recoverRemovedEntityRequestDescriptor = $convert.base64Decode(
    'ChtSZWNvdmVyUmVtb3ZlZEVudGl0eVJlcXVlc3QSGwoJbWFuYWdlX2lkGAEgASgFUghtYW5hZ2'
    'VJZBIbCgllbnRpdHlfaWQYAiABKAlSCGVudGl0eUlk');

@$core.Deprecated('Use recoverRemovedEntityResponseDescriptor instead')
const RecoverRemovedEntityResponse$json = {
  '1': 'RecoverRemovedEntityResponse',
  '2': [
    {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `RecoverRemovedEntityResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List recoverRemovedEntityResponseDescriptor = $convert.base64Decode(
    'ChxSZWNvdmVyUmVtb3ZlZEVudGl0eVJlc3BvbnNlEhYKBnJlc3VsdBgBIAEoCVIGcmVzdWx0');

@$core.Deprecated('Use getRemovedEntitiesPageRequestDescriptor instead')
const GetRemovedEntitiesPageRequest$json = {
  '1': 'GetRemovedEntitiesPageRequest',
  '2': [
    {'1': 'manage_id', '3': 1, '4': 1, '5': 5, '10': 'manageId'},
    {'1': 'page_index', '3': 2, '4': 1, '5': 13, '10': 'pageIndex'},
    {'1': 'conditions', '3': 3, '4': 1, '5': 12, '10': 'conditions'},
  ],
};

/// Descriptor for `GetRemovedEntitiesPageRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getRemovedEntitiesPageRequestDescriptor = $convert.base64Decode(
    'Ch1HZXRSZW1vdmVkRW50aXRpZXNQYWdlUmVxdWVzdBIbCgltYW5hZ2VfaWQYASABKAVSCG1hbm'
    'FnZUlkEh0KCnBhZ2VfaW5kZXgYAiABKA1SCXBhZ2VJbmRleBIeCgpjb25kaXRpb25zGAMgASgM'
    'Ugpjb25kaXRpb25z');

@$core.Deprecated('Use getRemovedEntitiesPageResponseDescriptor instead')
const GetRemovedEntitiesPageResponse$json = {
  '1': 'GetRemovedEntitiesPageResponse',
  '2': [
    {'1': 'entities', '3': 1, '4': 3, '5': 12, '10': 'entities'},
  ],
};

/// Descriptor for `GetRemovedEntitiesPageResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getRemovedEntitiesPageResponseDescriptor = $convert.base64Decode(
    'Ch5HZXRSZW1vdmVkRW50aXRpZXNQYWdlUmVzcG9uc2USGgoIZW50aXRpZXMYASADKAxSCGVudG'
    'l0aWVz');

@$core.Deprecated('Use getRemovedDataListRequestDescriptor instead')
const GetRemovedDataListRequest$json = {
  '1': 'GetRemovedDataListRequest',
  '2': [
    {'1': 'manage_id', '3': 1, '4': 1, '5': 9, '10': 'manageId'},
    {'1': 'entity_id', '3': 2, '4': 1, '5': 9, '10': 'entityId'},
    {'1': 'data_id', '3': 3, '4': 1, '5': 9, '10': 'dataId'},
  ],
};

/// Descriptor for `GetRemovedDataListRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getRemovedDataListRequestDescriptor = $convert.base64Decode(
    'ChlHZXRSZW1vdmVkRGF0YUxpc3RSZXF1ZXN0EhsKCW1hbmFnZV9pZBgBIAEoCVIIbWFuYWdlSW'
    'QSGwoJZW50aXR5X2lkGAIgASgJUghlbnRpdHlJZBIXCgdkYXRhX2lkGAMgASgJUgZkYXRhSWQ=');

@$core.Deprecated('Use getRemovedDataListResponseDescriptor instead')
const GetRemovedDataListResponse$json = {
  '1': 'GetRemovedDataListResponse',
  '2': [
    {'1': 'data_ids', '3': 1, '4': 3, '5': 9, '10': 'dataIds'},
  ],
};

/// Descriptor for `GetRemovedDataListResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getRemovedDataListResponseDescriptor = $convert.base64Decode(
    'ChpHZXRSZW1vdmVkRGF0YUxpc3RSZXNwb25zZRIZCghkYXRhX2lkcxgBIAMoCVIHZGF0YUlkcw'
    '==');

@$core.Deprecated('Use entityTimestampDescriptor instead')
const EntityTimestamp$json = {
  '1': 'EntityTimestamp',
  '2': [
    {'1': 'entity_id', '3': 1, '4': 1, '5': 9, '10': 'entityId'},
    {'1': 'timestamp', '3': 2, '4': 1, '5': 12, '10': 'timestamp'},
  ],
};

/// Descriptor for `EntityTimestamp`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List entityTimestampDescriptor = $convert.base64Decode(
    'Cg9FbnRpdHlUaW1lc3RhbXASGwoJZW50aXR5X2lkGAEgASgJUghlbnRpdHlJZBIcCgl0aW1lc3'
    'RhbXAYAiABKAxSCXRpbWVzdGFtcA==');

@$core.Deprecated('Use checkEntitiesUpdateRequestDescriptor instead')
const CheckEntitiesUpdateRequest$json = {
  '1': 'CheckEntitiesUpdateRequest',
  '2': [
    {'1': 'manage_id', '3': 1, '4': 1, '5': 5, '10': 'manageId'},
    {'1': 'entities', '3': 2, '4': 3, '5': 11, '6': '.cashmere.EntityTimestamp', '10': 'entities'},
  ],
};

/// Descriptor for `CheckEntitiesUpdateRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List checkEntitiesUpdateRequestDescriptor = $convert.base64Decode(
    'ChpDaGVja0VudGl0aWVzVXBkYXRlUmVxdWVzdBIbCgltYW5hZ2VfaWQYASABKAVSCG1hbmFnZU'
    'lkEjUKCGVudGl0aWVzGAIgAygLMhkuY2FzaG1lcmUuRW50aXR5VGltZXN0YW1wUghlbnRpdGll'
    'cw==');

@$core.Deprecated('Use checkEntitiesUpdateResponseDescriptor instead')
const CheckEntitiesUpdateResponse$json = {
  '1': 'CheckEntitiesUpdateResponse',
  '2': [
    {'1': 'entities', '3': 1, '4': 3, '5': 12, '10': 'entities'},
  ],
};

/// Descriptor for `CheckEntitiesUpdateResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List checkEntitiesUpdateResponseDescriptor = $convert.base64Decode(
    'ChtDaGVja0VudGl0aWVzVXBkYXRlUmVzcG9uc2USGgoIZW50aXRpZXMYASADKAxSCGVudGl0aW'
    'Vz');

@$core.Deprecated('Use checkUpdatesLaterThenTimeRequestDescriptor instead')
const CheckUpdatesLaterThenTimeRequest$json = {
  '1': 'CheckUpdatesLaterThenTimeRequest',
  '2': [
    {'1': 'manage_id', '3': 1, '4': 1, '5': 5, '10': 'manageId'},
    {'1': 'timestamp', '3': 2, '4': 1, '5': 12, '10': 'timestamp'},
    {'1': 'ascending_order', '3': 3, '4': 1, '5': 8, '10': 'ascendingOrder'},
  ],
};

/// Descriptor for `CheckUpdatesLaterThenTimeRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List checkUpdatesLaterThenTimeRequestDescriptor = $convert.base64Decode(
    'CiBDaGVja1VwZGF0ZXNMYXRlclRoZW5UaW1lUmVxdWVzdBIbCgltYW5hZ2VfaWQYASABKAVSCG'
    '1hbmFnZUlkEhwKCXRpbWVzdGFtcBgCIAEoDFIJdGltZXN0YW1wEicKD2FzY2VuZGluZ19vcmRl'
    'chgDIAEoCFIOYXNjZW5kaW5nT3JkZXI=');

@$core.Deprecated('Use checkUpdatesLaterThenTimeResponseDescriptor instead')
const CheckUpdatesLaterThenTimeResponse$json = {
  '1': 'CheckUpdatesLaterThenTimeResponse',
  '2': [
    {'1': 'entity_ids', '3': 1, '4': 3, '5': 9, '10': 'entityIds'},
  ],
};

/// Descriptor for `CheckUpdatesLaterThenTimeResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List checkUpdatesLaterThenTimeResponseDescriptor = $convert.base64Decode(
    'CiFDaGVja1VwZGF0ZXNMYXRlclRoZW5UaW1lUmVzcG9uc2USHQoKZW50aXR5X2lkcxgBIAMoCV'
    'IJZW50aXR5SWRz');

