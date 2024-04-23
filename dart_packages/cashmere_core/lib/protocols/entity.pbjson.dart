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

@$core.Deprecated('Use editOperationTypeEnumDescriptor instead')
const EditOperationTypeEnum$json = {
  '1': 'EditOperationTypeEnum',
  '2': [
    {'1': 'EDIT_PRIMARY_FIELD', '2': 0},
    {'1': 'EIDT_MAP_FIELD', '2': 1},
    {'1': 'EDIT_MAP_FIELD_REMOVE_KEY', '2': 2},
    {'1': 'EDIT_ADD_TO_ARRAY_FIELD', '2': 3},
    {'1': 'Edit_UPDATE_ARRAY_ELEMENT_FIELD', '2': 5},
    {'1': 'EDIT_REMOVE_FROM_ARRAY_FIELD', '2': 4},
  ],
};

/// Descriptor for `EditOperationTypeEnum`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List editOperationTypeEnumDescriptor = $convert.base64Decode(
    'ChVFZGl0T3BlcmF0aW9uVHlwZUVudW0SFgoSRURJVF9QUklNQVJZX0ZJRUxEEAASEgoORUlEVF'
    '9NQVBfRklFTEQQARIdChlFRElUX01BUF9GSUVMRF9SRU1PVkVfS0VZEAISGwoXRURJVF9BRERf'
    'VE9fQVJSQVlfRklFTEQQAxIjCh9FZGl0X1VQREFURV9BUlJBWV9FTEVNRU5UX0ZJRUxEEAUSIA'
    'ocRURJVF9SRU1PVkVfRlJPTV9BUlJBWV9GSUVMRBAE');

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
    {'1': 'specs_ids', '3': 9, '4': 3, '5': 9, '10': 'specsIds'},
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
    'GAcgASgJUgdvd25lcklkEhYKBmdyb3VwcxgIIAMoCVIGZ3JvdXBzEhsKCXNwZWNzX2lkcxgJIA'
    'MoCVIIc3BlY3NJZHMSHwoLY29tbWVudF9pZHMYCiADKAlSCmNvbW1lbnRJZHMSGAoHcmVtb3Zl'
    'ZBgLIAEoCFIHcmVtb3ZlZBIoChByZW1vdmVkX2RhdGFfaWRzGAwgAygJUg5yZW1vdmVkRGF0YU'
    'lkcw==');

@$core.Deprecated('Use changeEntityOwnerRequestDescriptor instead')
const ChangeEntityOwnerRequest$json = {
  '1': 'ChangeEntityOwnerRequest',
  '2': [
    {'1': 'manage_id', '3': 1, '4': 1, '5': 9, '10': 'manageId'},
    {'1': 'entity_id', '3': 2, '4': 1, '5': 9, '10': 'entityId'},
    {'1': 'old_owner_id', '3': 3, '4': 1, '5': 9, '10': 'oldOwnerId'},
    {'1': 'new_owner_id', '3': 4, '4': 1, '5': 9, '10': 'newOwnerId'},
  ],
};

/// Descriptor for `ChangeEntityOwnerRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List changeEntityOwnerRequestDescriptor = $convert.base64Decode(
    'ChhDaGFuZ2VFbnRpdHlPd25lclJlcXVlc3QSGwoJbWFuYWdlX2lkGAEgASgJUghtYW5hZ2VJZB'
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
    {'1': 'manage_id', '3': 1, '4': 1, '5': 9, '10': 'manageId'},
    {'1': 'data', '3': 2, '4': 1, '5': 12, '10': 'data'},
  ],
};

/// Descriptor for `NewEntityRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List newEntityRequestDescriptor = $convert.base64Decode(
    'ChBOZXdFbnRpdHlSZXF1ZXN0EhsKCW1hbmFnZV9pZBgBIAEoCVIIbWFuYWdlSWQSEgoEZGF0YR'
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
    {'1': 'manage_id', '3': 1, '4': 1, '5': 9, '10': 'manageId'},
    {'1': 'entity_id', '3': 2, '4': 1, '5': 9, '10': 'entityId'},
    {'1': 'data', '3': 3, '4': 1, '5': 12, '10': 'data'},
  ],
};

/// Descriptor for `EditEntityRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List editEntityRequestDescriptor = $convert.base64Decode(
    'ChFFZGl0RW50aXR5UmVxdWVzdBIbCgltYW5hZ2VfaWQYASABKAlSCG1hbmFnZUlkEhsKCWVudG'
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

@$core.Deprecated('Use entityFieldEditDescriptor instead')
const EntityFieldEdit$json = {
  '1': 'EntityFieldEdit',
  '2': [
    {'1': 'manage_id', '3': 1, '4': 1, '5': 9, '10': 'manageId'},
    {'1': 'entity_id', '3': 2, '4': 1, '5': 9, '10': 'entityId'},
    {'1': 'field_id', '3': 3, '4': 1, '5': 9, '10': 'fieldId'},
    {'1': 'operation_type', '3': 4, '4': 1, '5': 14, '6': '.cashmere.EditOperationTypeEnum', '10': 'operationType'},
    {'1': 'edit', '3': 5, '4': 1, '5': 12, '10': 'edit'},
  ],
};

/// Descriptor for `EntityFieldEdit`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List entityFieldEditDescriptor = $convert.base64Decode(
    'Cg9FbnRpdHlGaWVsZEVkaXQSGwoJbWFuYWdlX2lkGAEgASgJUghtYW5hZ2VJZBIbCgllbnRpdH'
    'lfaWQYAiABKAlSCGVudGl0eUlkEhkKCGZpZWxkX2lkGAMgASgJUgdmaWVsZElkEkYKDm9wZXJh'
    'dGlvbl90eXBlGAQgASgOMh8uY2FzaG1lcmUuRWRpdE9wZXJhdGlvblR5cGVFbnVtUg1vcGVyYX'
    'Rpb25UeXBlEhIKBGVkaXQYBSABKAxSBGVkaXQ=');

@$core.Deprecated('Use editMultiEntityFieldsRequestDescriptor instead')
const EditMultiEntityFieldsRequest$json = {
  '1': 'EditMultiEntityFieldsRequest',
  '2': [
    {'1': 'edits', '3': 1, '4': 3, '5': 11, '6': '.cashmere.EntityFieldEdit', '10': 'edits'},
  ],
};

/// Descriptor for `EditMultiEntityFieldsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List editMultiEntityFieldsRequestDescriptor = $convert.base64Decode(
    'ChxFZGl0TXVsdGlFbnRpdHlGaWVsZHNSZXF1ZXN0Ei8KBWVkaXRzGAEgAygLMhkuY2FzaG1lcm'
    'UuRW50aXR5RmllbGRFZGl0UgVlZGl0cw==');

@$core.Deprecated('Use editMultiEntityFieldsResponseDescriptor instead')
const EditMultiEntityFieldsResponse$json = {
  '1': 'EditMultiEntityFieldsResponse',
  '2': [
    {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `EditMultiEntityFieldsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List editMultiEntityFieldsResponseDescriptor = $convert.base64Decode(
    'Ch1FZGl0TXVsdGlFbnRpdHlGaWVsZHNSZXNwb25zZRIWCgZyZXN1bHQYASABKAlSBnJlc3VsdA'
    '==');

@$core.Deprecated('Use editEntityFieldRequestDescriptor instead')
const EditEntityFieldRequest$json = {
  '1': 'EditEntityFieldRequest',
  '2': [
    {'1': 'manage_id', '3': 1, '4': 1, '5': 9, '10': 'manageId'},
    {'1': 'entity_id', '3': 2, '4': 1, '5': 9, '10': 'entityId'},
    {'1': 'field_id', '3': 3, '4': 1, '5': 9, '10': 'fieldId'},
    {'1': 'new_value', '3': 4, '4': 1, '5': 12, '10': 'newValue'},
  ],
};

/// Descriptor for `EditEntityFieldRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List editEntityFieldRequestDescriptor = $convert.base64Decode(
    'ChZFZGl0RW50aXR5RmllbGRSZXF1ZXN0EhsKCW1hbmFnZV9pZBgBIAEoCVIIbWFuYWdlSWQSGw'
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
    {'1': 'manage_id', '3': 1, '4': 1, '5': 9, '10': 'manageId'},
    {'1': 'entity_id', '3': 2, '4': 1, '5': 9, '10': 'entityId'},
    {'1': 'field_id', '3': 3, '4': 1, '5': 9, '10': 'fieldId'},
    {'1': 'key', '3': 4, '4': 1, '5': 9, '10': 'key'},
    {'1': 'new_value', '3': 5, '4': 1, '5': 12, '10': 'newValue'},
  ],
};

/// Descriptor for `EditEntityMapFieldRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List editEntityMapFieldRequestDescriptor = $convert.base64Decode(
    'ChlFZGl0RW50aXR5TWFwRmllbGRSZXF1ZXN0EhsKCW1hbmFnZV9pZBgBIAEoCVIIbWFuYWdlSW'
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
    {'1': 'manage_id', '3': 1, '4': 1, '5': 9, '10': 'manageId'},
    {'1': 'entity_id', '3': 2, '4': 1, '5': 9, '10': 'entityId'},
    {'1': 'field_id', '3': 3, '4': 1, '5': 9, '10': 'fieldId'},
    {'1': 'key', '3': 4, '4': 1, '5': 9, '10': 'key'},
  ],
};

/// Descriptor for `EditEntityMapFieldRemoveKeyRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List editEntityMapFieldRemoveKeyRequestDescriptor = $convert.base64Decode(
    'CiJFZGl0RW50aXR5TWFwRmllbGRSZW1vdmVLZXlSZXF1ZXN0EhsKCW1hbmFnZV9pZBgBIAEoCV'
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
    {'1': 'manage_id', '3': 1, '4': 1, '5': 9, '10': 'manageId'},
    {'1': 'entity_id', '3': 2, '4': 1, '5': 9, '10': 'entityId'},
    {'1': 'field_id', '3': 3, '4': 1, '5': 9, '10': 'fieldId'},
    {'1': 'items', '3': 4, '4': 1, '5': 12, '10': 'items'},
  ],
};

/// Descriptor for `EditEntityArrayFieldAddItemsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List editEntityArrayFieldAddItemsRequestDescriptor = $convert.base64Decode(
    'CiNFZGl0RW50aXR5QXJyYXlGaWVsZEFkZEl0ZW1zUmVxdWVzdBIbCgltYW5hZ2VfaWQYASABKA'
    'lSCG1hbmFnZUlkEhsKCWVudGl0eV9pZBgCIAEoCVIIZW50aXR5SWQSGQoIZmllbGRfaWQYAyAB'
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
    {'1': 'manage_id', '3': 1, '4': 1, '5': 9, '10': 'manageId'},
    {'1': 'entity_id', '3': 2, '4': 1, '5': 9, '10': 'entityId'},
    {'1': 'field_id', '3': 3, '4': 1, '5': 9, '10': 'fieldId'},
    {'1': 'items', '3': 4, '4': 1, '5': 12, '10': 'items'},
  ],
};

/// Descriptor for `EditEntityArrayFieldRemoveItemsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List editEntityArrayFieldRemoveItemsRequestDescriptor = $convert.base64Decode(
    'CiZFZGl0RW50aXR5QXJyYXlGaWVsZFJlbW92ZUl0ZW1zUmVxdWVzdBIbCgltYW5hZ2VfaWQYAS'
    'ABKAlSCG1hbmFnZUlkEhsKCWVudGl0eV9pZBgCIAEoCVIIZW50aXR5SWQSGQoIZmllbGRfaWQY'
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
    {'1': 'manage_id', '3': 1, '4': 1, '5': 9, '10': 'manageId'},
    {'1': 'entity_id', '3': 2, '4': 1, '5': 9, '10': 'entityId'},
    {'1': 'no_present_fields', '3': 3, '4': 3, '5': 9, '10': 'noPresentFields'},
    {'1': 'present_fields', '3': 4, '4': 3, '5': 9, '10': 'presentFields'},
  ],
};

/// Descriptor for `GetEntityRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getEntityRequestDescriptor = $convert.base64Decode(
    'ChBHZXRFbnRpdHlSZXF1ZXN0EhsKCW1hbmFnZV9pZBgBIAEoCVIIbWFuYWdlSWQSGwoJZW50aX'
    'R5X2lkGAIgASgJUghlbnRpdHlJZBIqChFub19wcmVzZW50X2ZpZWxkcxgDIAMoCVIPbm9QcmVz'
    'ZW50RmllbGRzEiUKDnByZXNlbnRfZmllbGRzGAQgAygJUg1wcmVzZW50RmllbGRz');

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
    {'1': 'manage_id', '3': 1, '4': 1, '5': 9, '10': 'manageId'},
    {'1': 'entity_ids', '3': 2, '4': 3, '5': 9, '10': 'entityIds'},
    {'1': 'no_present_fields', '3': 3, '4': 3, '5': 9, '10': 'noPresentFields'},
    {'1': 'present_fields', '3': 4, '4': 3, '5': 9, '10': 'presentFields'},
  ],
};

/// Descriptor for `GetEntitiesRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getEntitiesRequestDescriptor = $convert.base64Decode(
    'ChJHZXRFbnRpdGllc1JlcXVlc3QSGwoJbWFuYWdlX2lkGAEgASgJUghtYW5hZ2VJZBIdCgplbn'
    'RpdHlfaWRzGAIgAygJUgllbnRpdHlJZHMSKgoRbm9fcHJlc2VudF9maWVsZHMYAyADKAlSD25v'
    'UHJlc2VudEZpZWxkcxIlCg5wcmVzZW50X2ZpZWxkcxgEIAMoCVINcHJlc2VudEZpZWxkcw==');

@$core.Deprecated('Use getEntitiesResponseDescriptor instead')
const GetEntitiesResponse$json = {
  '1': 'GetEntitiesResponse',
  '2': [
    {'1': 'entity', '3': 1, '4': 1, '5': 12, '10': 'entity'},
  ],
};

/// Descriptor for `GetEntitiesResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getEntitiesResponseDescriptor = $convert.base64Decode(
    'ChNHZXRFbnRpdGllc1Jlc3BvbnNlEhYKBmVudGl0eRgBIAEoDFIGZW50aXR5');

@$core.Deprecated('Use getEntitiesPageRequestDescriptor instead')
const GetEntitiesPageRequest$json = {
  '1': 'GetEntitiesPageRequest',
  '2': [
    {'1': 'manage_id', '3': 1, '4': 1, '5': 9, '10': 'manageId'},
    {'1': 'page_index', '3': 2, '4': 1, '5': 13, '10': 'pageIndex'},
    {'1': 'match_doc', '3': 3, '4': 1, '5': 12, '10': 'matchDoc'},
    {'1': 'sort_doc', '3': 4, '4': 1, '5': 12, '10': 'sortDoc'},
    {'1': 'no_present_fields', '3': 5, '4': 3, '5': 9, '10': 'noPresentFields'},
    {'1': 'start_oid', '3': 6, '4': 1, '5': 9, '10': 'startOid'},
    {'1': 'present_fields', '3': 7, '4': 3, '5': 9, '10': 'presentFields'},
  ],
};

/// Descriptor for `GetEntitiesPageRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getEntitiesPageRequestDescriptor = $convert.base64Decode(
    'ChZHZXRFbnRpdGllc1BhZ2VSZXF1ZXN0EhsKCW1hbmFnZV9pZBgBIAEoCVIIbWFuYWdlSWQSHQ'
    'oKcGFnZV9pbmRleBgCIAEoDVIJcGFnZUluZGV4EhsKCW1hdGNoX2RvYxgDIAEoDFIIbWF0Y2hE'
    'b2MSGQoIc29ydF9kb2MYBCABKAxSB3NvcnREb2MSKgoRbm9fcHJlc2VudF9maWVsZHMYBSADKA'
    'lSD25vUHJlc2VudEZpZWxkcxIbCglzdGFydF9vaWQYBiABKAlSCHN0YXJ0T2lkEiUKDnByZXNl'
    'bnRfZmllbGRzGAcgAygJUg1wcmVzZW50RmllbGRz');

@$core.Deprecated('Use getEntitiesPageResponseDescriptor instead')
const GetEntitiesPageResponse$json = {
  '1': 'GetEntitiesPageResponse',
  '2': [
    {'1': 'entity', '3': 1, '4': 1, '5': 12, '10': 'entity'},
  ],
};

/// Descriptor for `GetEntitiesPageResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getEntitiesPageResponseDescriptor = $convert.base64Decode(
    'ChdHZXRFbnRpdGllc1BhZ2VSZXNwb25zZRIWCgZlbnRpdHkYASABKAxSBmVudGl0eQ==');

@$core.Deprecated('Use interactiveGetEntitiesRequestDescriptor instead')
const InteractiveGetEntitiesRequest$json = {
  '1': 'InteractiveGetEntitiesRequest',
  '2': [
    {'1': 'manage_id', '3': 1, '4': 1, '5': 9, '10': 'manageId'},
    {'1': 'page_index', '3': 2, '4': 1, '5': 13, '10': 'pageIndex'},
    {'1': 'match_doc', '3': 3, '4': 1, '5': 12, '10': 'matchDoc'},
    {'1': 'sort_doc', '3': 4, '4': 1, '5': 12, '10': 'sortDoc'},
    {'1': 'no_present_fields', '3': 5, '4': 3, '5': 9, '10': 'noPresentFields'},
    {'1': 'present_fields', '3': 6, '4': 3, '5': 9, '10': 'presentFields'},
  ],
};

/// Descriptor for `InteractiveGetEntitiesRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List interactiveGetEntitiesRequestDescriptor = $convert.base64Decode(
    'Ch1JbnRlcmFjdGl2ZUdldEVudGl0aWVzUmVxdWVzdBIbCgltYW5hZ2VfaWQYASABKAlSCG1hbm'
    'FnZUlkEh0KCnBhZ2VfaW5kZXgYAiABKA1SCXBhZ2VJbmRleBIbCgltYXRjaF9kb2MYAyABKAxS'
    'CG1hdGNoRG9jEhkKCHNvcnRfZG9jGAQgASgMUgdzb3J0RG9jEioKEW5vX3ByZXNlbnRfZmllbG'
    'RzGAUgAygJUg9ub1ByZXNlbnRGaWVsZHMSJQoOcHJlc2VudF9maWVsZHMYBiADKAlSDXByZXNl'
    'bnRGaWVsZHM=');

@$core.Deprecated('Use interactiveGetEntitiesResponseDescriptor instead')
const InteractiveGetEntitiesResponse$json = {
  '1': 'InteractiveGetEntitiesResponse',
  '2': [
    {'1': 'page_index', '3': 1, '4': 1, '5': 13, '10': 'pageIndex'},
    {'1': 'entities', '3': 2, '4': 3, '5': 12, '10': 'entities'},
    {'1': 'total_count', '3': 3, '4': 1, '5': 4, '10': 'totalCount'},
  ],
};

/// Descriptor for `InteractiveGetEntitiesResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List interactiveGetEntitiesResponseDescriptor = $convert.base64Decode(
    'Ch5JbnRlcmFjdGl2ZUdldEVudGl0aWVzUmVzcG9uc2USHQoKcGFnZV9pbmRleBgBIAEoDVIJcG'
    'FnZUluZGV4EhoKCGVudGl0aWVzGAIgAygMUghlbnRpdGllcxIfCgt0b3RhbF9jb3VudBgDIAEo'
    'BFIKdG90YWxDb3VudA==');

@$core.Deprecated('Use getHardCodedEntitiesRequestDescriptor instead')
const GetHardCodedEntitiesRequest$json = {
  '1': 'GetHardCodedEntitiesRequest',
  '2': [
    {'1': 'manage_id', '3': 1, '4': 1, '5': 9, '10': 'manageId'},
  ],
};

/// Descriptor for `GetHardCodedEntitiesRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getHardCodedEntitiesRequestDescriptor = $convert.base64Decode(
    'ChtHZXRIYXJkQ29kZWRFbnRpdGllc1JlcXVlc3QSGwoJbWFuYWdlX2lkGAEgASgJUghtYW5hZ2'
    'VJZA==');

@$core.Deprecated('Use getHardCodedEntitiesResponseDescriptor instead')
const GetHardCodedEntitiesResponse$json = {
  '1': 'GetHardCodedEntitiesResponse',
  '2': [
    {'1': 'entities', '3': 1, '4': 3, '5': 12, '10': 'entities'},
  ],
};

/// Descriptor for `GetHardCodedEntitiesResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getHardCodedEntitiesResponseDescriptor = $convert.base64Decode(
    'ChxHZXRIYXJkQ29kZWRFbnRpdGllc1Jlc3BvbnNlEhoKCGVudGl0aWVzGAEgAygMUghlbnRpdG'
    'llcw==');

@$core.Deprecated('Use markEntityRemovedRequestDescriptor instead')
const MarkEntityRemovedRequest$json = {
  '1': 'MarkEntityRemovedRequest',
  '2': [
    {'1': 'manage_id', '3': 1, '4': 1, '5': 9, '10': 'manageId'},
    {'1': 'entity_id', '3': 2, '4': 1, '5': 9, '10': 'entityId'},
  ],
};

/// Descriptor for `MarkEntityRemovedRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List markEntityRemovedRequestDescriptor = $convert.base64Decode(
    'ChhNYXJrRW50aXR5UmVtb3ZlZFJlcXVlc3QSGwoJbWFuYWdlX2lkGAEgASgJUghtYW5hZ2VJZB'
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
    {'1': 'manage_id', '3': 1, '4': 1, '5': 9, '10': 'manageId'},
    {'1': 'entity_id', '3': 2, '4': 1, '5': 9, '10': 'entityId'},
  ],
};

/// Descriptor for `RecoverRemovedEntityRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List recoverRemovedEntityRequestDescriptor = $convert.base64Decode(
    'ChtSZWNvdmVyUmVtb3ZlZEVudGl0eVJlcXVlc3QSGwoJbWFuYWdlX2lkGAEgASgJUghtYW5hZ2'
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
    {'1': 'manage_id', '3': 1, '4': 1, '5': 9, '10': 'manageId'},
    {'1': 'page_index', '3': 2, '4': 1, '5': 13, '10': 'pageIndex'},
    {'1': 'conditions', '3': 3, '4': 1, '5': 12, '10': 'conditions'},
  ],
};

/// Descriptor for `GetRemovedEntitiesPageRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getRemovedEntitiesPageRequestDescriptor = $convert.base64Decode(
    'Ch1HZXRSZW1vdmVkRW50aXRpZXNQYWdlUmVxdWVzdBIbCgltYW5hZ2VfaWQYASABKAlSCG1hbm'
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
    {'1': 'manage_id', '3': 1, '4': 1, '5': 9, '10': 'manageId'},
    {'1': 'entities', '3': 2, '4': 3, '5': 11, '6': '.cashmere.EntityTimestamp', '10': 'entities'},
  ],
};

/// Descriptor for `CheckEntitiesUpdateRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List checkEntitiesUpdateRequestDescriptor = $convert.base64Decode(
    'ChpDaGVja0VudGl0aWVzVXBkYXRlUmVxdWVzdBIbCgltYW5hZ2VfaWQYASABKAlSCG1hbmFnZU'
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
    {'1': 'manage_id', '3': 1, '4': 1, '5': 9, '10': 'manageId'},
    {'1': 'timestamp', '3': 2, '4': 1, '5': 12, '10': 'timestamp'},
    {'1': 'ascending_order', '3': 3, '4': 1, '5': 8, '10': 'ascendingOrder'},
    {'1': 'filter', '3': 4, '4': 1, '5': 12, '10': 'filter'},
  ],
};

/// Descriptor for `CheckUpdatesLaterThenTimeRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List checkUpdatesLaterThenTimeRequestDescriptor = $convert.base64Decode(
    'CiBDaGVja1VwZGF0ZXNMYXRlclRoZW5UaW1lUmVxdWVzdBIbCgltYW5hZ2VfaWQYASABKAlSCG'
    '1hbmFnZUlkEhwKCXRpbWVzdGFtcBgCIAEoDFIJdGltZXN0YW1wEicKD2FzY2VuZGluZ19vcmRl'
    'chgDIAEoCFIOYXNjZW5kaW5nT3JkZXISFgoGZmlsdGVyGAQgASgMUgZmaWx0ZXI=');

@$core.Deprecated('Use checkUpdatesLaterThenTimeResponseDescriptor instead')
const CheckUpdatesLaterThenTimeResponse$json = {
  '1': 'CheckUpdatesLaterThenTimeResponse',
  '2': [
    {'1': 'results', '3': 1, '4': 3, '5': 12, '10': 'results'},
  ],
};

/// Descriptor for `CheckUpdatesLaterThenTimeResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List checkUpdatesLaterThenTimeResponseDescriptor = $convert.base64Decode(
    'CiFDaGVja1VwZGF0ZXNMYXRlclRoZW5UaW1lUmVzcG9uc2USGAoHcmVzdWx0cxgBIAMoDFIHcm'
    'VzdWx0cw==');

