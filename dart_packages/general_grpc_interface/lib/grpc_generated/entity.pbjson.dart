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
    const {'1': 'data', '3': 1, '4': 1, '5': 12, '10': 'data'},
  ],
};

/// Descriptor for `Entity`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List entityDescriptor = $convert.base64Decode('CgZFbnRpdHkSEgoEZGF0YRgBIAEoDFIEZGF0YQ==');
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
