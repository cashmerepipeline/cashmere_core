//
//  Generated code. Do not modify.
//  source: tag.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use newTagRequestDescriptor instead')
const NewTagRequest$json = {
  '1': 'NewTagRequest',
  '2': [
    {'1': 'target_manage_id', '3': 1, '4': 1, '5': 5, '10': 'targetManageId'},
    {'1': 'name', '3': 2, '4': 1, '5': 11, '6': '.cashmere.Name', '10': 'name'},
    {'1': 'description', '3': 3, '4': 1, '5': 9, '10': 'description'},
  ],
};

/// Descriptor for `NewTagRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List newTagRequestDescriptor = $convert.base64Decode(
    'Cg1OZXdUYWdSZXF1ZXN0EigKEHRhcmdldF9tYW5hZ2VfaWQYASABKAVSDnRhcmdldE1hbmFnZU'
    'lkEiIKBG5hbWUYAiABKAsyDi5jYXNobWVyZS5OYW1lUgRuYW1lEiAKC2Rlc2NyaXB0aW9uGAMg'
    'ASgJUgtkZXNjcmlwdGlvbg==');

@$core.Deprecated('Use newTagResponseDescriptor instead')
const NewTagResponse$json = {
  '1': 'NewTagResponse',
  '2': [
    {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `NewTagResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List newTagResponseDescriptor = $convert.base64Decode(
    'Cg5OZXdUYWdSZXNwb25zZRIWCgZyZXN1bHQYASABKAlSBnJlc3VsdA==');

@$core.Deprecated('Use addTagsToEntityRequestDescriptor instead')
const AddTagsToEntityRequest$json = {
  '1': 'AddTagsToEntityRequest',
  '2': [
    {'1': 'tag_ids', '3': 1, '4': 3, '5': 9, '10': 'tagIds'},
    {'1': 'target_manage_id', '3': 2, '4': 1, '5': 5, '10': 'targetManageId'},
    {'1': 'target_entity_id', '3': 3, '4': 1, '5': 9, '10': 'targetEntityId'},
  ],
};

/// Descriptor for `AddTagsToEntityRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List addTagsToEntityRequestDescriptor = $convert.base64Decode(
    'ChZBZGRUYWdzVG9FbnRpdHlSZXF1ZXN0EhcKB3RhZ19pZHMYASADKAlSBnRhZ0lkcxIoChB0YX'
    'JnZXRfbWFuYWdlX2lkGAIgASgFUg50YXJnZXRNYW5hZ2VJZBIoChB0YXJnZXRfZW50aXR5X2lk'
    'GAMgASgJUg50YXJnZXRFbnRpdHlJZA==');

@$core.Deprecated('Use addTagsToEntityResponseDescriptor instead')
const AddTagsToEntityResponse$json = {
  '1': 'AddTagsToEntityResponse',
  '2': [
    {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `AddTagsToEntityResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List addTagsToEntityResponseDescriptor = $convert.base64Decode(
    'ChdBZGRUYWdzVG9FbnRpdHlSZXNwb25zZRIWCgZyZXN1bHQYASABKAlSBnJlc3VsdA==');

@$core.Deprecated('Use getTagsRequestDescriptor instead')
const GetTagsRequest$json = {
  '1': 'GetTagsRequest',
  '2': [
    {'1': 'target_manage_id', '3': 1, '4': 1, '5': 5, '10': 'targetManageId'},
  ],
};

/// Descriptor for `GetTagsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getTagsRequestDescriptor = $convert.base64Decode(
    'Cg5HZXRUYWdzUmVxdWVzdBIoChB0YXJnZXRfbWFuYWdlX2lkGAEgASgFUg50YXJnZXRNYW5hZ2'
    'VJZA==');

@$core.Deprecated('Use getTagsResponseDescriptor instead')
const GetTagsResponse$json = {
  '1': 'GetTagsResponse',
  '2': [
    {'1': 'tags', '3': 1, '4': 3, '5': 12, '10': 'tags'},
  ],
};

/// Descriptor for `GetTagsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getTagsResponseDescriptor = $convert.base64Decode(
    'Cg9HZXRUYWdzUmVzcG9uc2USEgoEdGFncxgBIAMoDFIEdGFncw==');

@$core.Deprecated('Use removeTagsFromEntityRequestDescriptor instead')
const RemoveTagsFromEntityRequest$json = {
  '1': 'RemoveTagsFromEntityRequest',
  '2': [
    {'1': 'target_manage_id', '3': 1, '4': 1, '5': 5, '10': 'targetManageId'},
    {'1': 'target_entity_id', '3': 2, '4': 1, '5': 9, '10': 'targetEntityId'},
    {'1': 'tag_ids', '3': 3, '4': 3, '5': 9, '10': 'tagIds'},
  ],
};

/// Descriptor for `RemoveTagsFromEntityRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List removeTagsFromEntityRequestDescriptor = $convert.base64Decode(
    'ChtSZW1vdmVUYWdzRnJvbUVudGl0eVJlcXVlc3QSKAoQdGFyZ2V0X21hbmFnZV9pZBgBIAEoBV'
    'IOdGFyZ2V0TWFuYWdlSWQSKAoQdGFyZ2V0X2VudGl0eV9pZBgCIAEoCVIOdGFyZ2V0RW50aXR5'
    'SWQSFwoHdGFnX2lkcxgDIAMoCVIGdGFnSWRz');

@$core.Deprecated('Use removeTagsFromEntityResponseDescriptor instead')
const RemoveTagsFromEntityResponse$json = {
  '1': 'RemoveTagsFromEntityResponse',
  '2': [
    {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `RemoveTagsFromEntityResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List removeTagsFromEntityResponseDescriptor = $convert.base64Decode(
    'ChxSZW1vdmVUYWdzRnJvbUVudGl0eVJlc3BvbnNlEhYKBnJlc3VsdBgBIAEoCVIGcmVzdWx0');

