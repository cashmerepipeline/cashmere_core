///
//  Generated code. Do not modify.
//  source: view.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields,deprecated_member_use_from_same_package

import 'dart:core' as $core;
import 'dart:convert' as $convert;
import 'dart:typed_data' as $typed_data;
@$core.Deprecated('Use getManageViewRequestDescriptor instead')
const GetManageViewRequest$json = const {
  '1': 'GetManageViewRequest',
  '2': const [
    const {'1': 'manage_name', '3': 1, '4': 1, '5': 9, '10': 'manageName'},
  ],
};

/// Descriptor for `GetManageViewRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getManageViewRequestDescriptor = $convert.base64Decode('ChRHZXRNYW5hZ2VWaWV3UmVxdWVzdBIfCgttYW5hZ2VfbmFtZRgBIAEoCVIKbWFuYWdlTmFtZQ==');
@$core.Deprecated('Use getManageViewResponseDescriptor instead')
const GetManageViewResponse$json = const {
  '1': 'GetManageViewResponse',
  '2': const [
    const {'1': 'view_token', '3': 1, '4': 1, '5': 9, '10': 'viewToken'},
  ],
};

/// Descriptor for `GetManageViewResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getManageViewResponseDescriptor = $convert.base64Decode('ChVHZXRNYW5hZ2VWaWV3UmVzcG9uc2USHQoKdmlld190b2tlbhgBIAEoCVIJdmlld1Rva2Vu');
@$core.Deprecated('Use addGroupIntoManageReadRuleRequestDescriptor instead')
const AddGroupIntoManageReadRuleRequest$json = const {
  '1': 'AddGroupIntoManageReadRuleRequest',
  '2': const [
    const {'1': 'manage_id', '3': 1, '4': 1, '5': 5, '10': 'manageId'},
    const {'1': 'group_id', '3': 2, '4': 1, '5': 9, '10': 'groupId'},
    const {'1': 'read_rule', '3': 3, '4': 1, '5': 9, '10': 'readRule'},
  ],
};

/// Descriptor for `AddGroupIntoManageReadRuleRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List addGroupIntoManageReadRuleRequestDescriptor = $convert.base64Decode('CiFBZGRHcm91cEludG9NYW5hZ2VSZWFkUnVsZVJlcXVlc3QSGwoJbWFuYWdlX2lkGAEgASgFUghtYW5hZ2VJZBIZCghncm91cF9pZBgCIAEoCVIHZ3JvdXBJZBIbCglyZWFkX3J1bGUYAyABKAlSCHJlYWRSdWxl');
@$core.Deprecated('Use addGroupIntoManageReadRuleResponseDescriptor instead')
const AddGroupIntoManageReadRuleResponse$json = const {
  '1': 'AddGroupIntoManageReadRuleResponse',
  '2': const [
    const {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `AddGroupIntoManageReadRuleResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List addGroupIntoManageReadRuleResponseDescriptor = $convert.base64Decode('CiJBZGRHcm91cEludG9NYW5hZ2VSZWFkUnVsZVJlc3BvbnNlEhYKBnJlc3VsdBgBIAEoCVIGcmVzdWx0');
@$core.Deprecated('Use addGroupIntoManageWriteRuleRequestDescriptor instead')
const AddGroupIntoManageWriteRuleRequest$json = const {
  '1': 'AddGroupIntoManageWriteRuleRequest',
  '2': const [
    const {'1': 'manage_id', '3': 1, '4': 1, '5': 5, '10': 'manageId'},
    const {'1': 'group_id', '3': 2, '4': 1, '5': 9, '10': 'groupId'},
    const {'1': 'read_rule', '3': 3, '4': 1, '5': 9, '10': 'readRule'},
  ],
};

/// Descriptor for `AddGroupIntoManageWriteRuleRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List addGroupIntoManageWriteRuleRequestDescriptor = $convert.base64Decode('CiJBZGRHcm91cEludG9NYW5hZ2VXcml0ZVJ1bGVSZXF1ZXN0EhsKCW1hbmFnZV9pZBgBIAEoBVIIbWFuYWdlSWQSGQoIZ3JvdXBfaWQYAiABKAlSB2dyb3VwSWQSGwoJcmVhZF9ydWxlGAMgASgJUghyZWFkUnVsZQ==');
@$core.Deprecated('Use addGroupIntoManageWriteRuleResponseDescriptor instead')
const AddGroupIntoManageWriteRuleResponse$json = const {
  '1': 'AddGroupIntoManageWriteRuleResponse',
  '2': const [
    const {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `AddGroupIntoManageWriteRuleResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List addGroupIntoManageWriteRuleResponseDescriptor = $convert.base64Decode('CiNBZGRHcm91cEludG9NYW5hZ2VXcml0ZVJ1bGVSZXNwb25zZRIWCgZyZXN1bHQYASABKAlSBnJlc3VsdA==');
@$core.Deprecated('Use removeGroupFromManageReadRuleRequestDescriptor instead')
const RemoveGroupFromManageReadRuleRequest$json = const {
  '1': 'RemoveGroupFromManageReadRuleRequest',
  '2': const [
    const {'1': 'manage_id', '3': 1, '4': 1, '5': 5, '10': 'manageId'},
    const {'1': 'group_id', '3': 2, '4': 1, '5': 9, '10': 'groupId'},
    const {'1': 'read_rule', '3': 3, '4': 1, '5': 9, '10': 'readRule'},
  ],
};

/// Descriptor for `RemoveGroupFromManageReadRuleRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List removeGroupFromManageReadRuleRequestDescriptor = $convert.base64Decode('CiRSZW1vdmVHcm91cEZyb21NYW5hZ2VSZWFkUnVsZVJlcXVlc3QSGwoJbWFuYWdlX2lkGAEgASgFUghtYW5hZ2VJZBIZCghncm91cF9pZBgCIAEoCVIHZ3JvdXBJZBIbCglyZWFkX3J1bGUYAyABKAlSCHJlYWRSdWxl');
@$core.Deprecated('Use removeGroupFromManageReadRuleResponseDescriptor instead')
const RemoveGroupFromManageReadRuleResponse$json = const {
  '1': 'RemoveGroupFromManageReadRuleResponse',
  '2': const [
    const {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `RemoveGroupFromManageReadRuleResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List removeGroupFromManageReadRuleResponseDescriptor = $convert.base64Decode('CiVSZW1vdmVHcm91cEZyb21NYW5hZ2VSZWFkUnVsZVJlc3BvbnNlEhYKBnJlc3VsdBgBIAEoCVIGcmVzdWx0');
@$core.Deprecated('Use removeGroupFromManageWriteRuleRequestDescriptor instead')
const RemoveGroupFromManageWriteRuleRequest$json = const {
  '1': 'RemoveGroupFromManageWriteRuleRequest',
  '2': const [
    const {'1': 'manage_id', '3': 1, '4': 1, '5': 5, '10': 'manageId'},
    const {'1': 'group_id', '3': 2, '4': 1, '5': 9, '10': 'groupId'},
    const {'1': 'read_rule', '3': 3, '4': 1, '5': 9, '10': 'readRule'},
  ],
};

/// Descriptor for `RemoveGroupFromManageWriteRuleRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List removeGroupFromManageWriteRuleRequestDescriptor = $convert.base64Decode('CiVSZW1vdmVHcm91cEZyb21NYW5hZ2VXcml0ZVJ1bGVSZXF1ZXN0EhsKCW1hbmFnZV9pZBgBIAEoBVIIbWFuYWdlSWQSGQoIZ3JvdXBfaWQYAiABKAlSB2dyb3VwSWQSGwoJcmVhZF9ydWxlGAMgASgJUghyZWFkUnVsZQ==');
@$core.Deprecated('Use removeGroupFromManageWriteRuleResponseDescriptor instead')
const RemoveGroupFromManageWriteRuleResponse$json = const {
  '1': 'RemoveGroupFromManageWriteRuleResponse',
  '2': const [
    const {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `RemoveGroupFromManageWriteRuleResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List removeGroupFromManageWriteRuleResponseDescriptor = $convert.base64Decode('CiZSZW1vdmVHcm91cEZyb21NYW5hZ2VXcml0ZVJ1bGVSZXNwb25zZRIWCgZyZXN1bHQYASABKAlSBnJlc3VsdA==');
@$core.Deprecated('Use addGroupIntoCollectionReadRuleRequestDescriptor instead')
const AddGroupIntoCollectionReadRuleRequest$json = const {
  '1': 'AddGroupIntoCollectionReadRuleRequest',
  '2': const [
    const {'1': 'manage_id', '3': 1, '4': 1, '5': 5, '10': 'manageId'},
    const {'1': 'group_id', '3': 2, '4': 1, '5': 9, '10': 'groupId'},
    const {'1': 'read_rule', '3': 3, '4': 1, '5': 9, '10': 'readRule'},
  ],
};

/// Descriptor for `AddGroupIntoCollectionReadRuleRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List addGroupIntoCollectionReadRuleRequestDescriptor = $convert.base64Decode('CiVBZGRHcm91cEludG9Db2xsZWN0aW9uUmVhZFJ1bGVSZXF1ZXN0EhsKCW1hbmFnZV9pZBgBIAEoBVIIbWFuYWdlSWQSGQoIZ3JvdXBfaWQYAiABKAlSB2dyb3VwSWQSGwoJcmVhZF9ydWxlGAMgASgJUghyZWFkUnVsZQ==');
@$core.Deprecated('Use addGroupIntoCollectionReadRuleResponseDescriptor instead')
const AddGroupIntoCollectionReadRuleResponse$json = const {
  '1': 'AddGroupIntoCollectionReadRuleResponse',
  '2': const [
    const {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `AddGroupIntoCollectionReadRuleResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List addGroupIntoCollectionReadRuleResponseDescriptor = $convert.base64Decode('CiZBZGRHcm91cEludG9Db2xsZWN0aW9uUmVhZFJ1bGVSZXNwb25zZRIWCgZyZXN1bHQYASABKAlSBnJlc3VsdA==');
@$core.Deprecated('Use addGroupIntoCollectionRuleRequestDescriptor instead')
const AddGroupIntoCollectionRuleRequest$json = const {
  '1': 'AddGroupIntoCollectionRuleRequest',
  '2': const [
    const {'1': 'manage_id', '3': 1, '4': 1, '5': 5, '10': 'manageId'},
    const {'1': 'group_id', '3': 2, '4': 1, '5': 9, '10': 'groupId'},
    const {'1': 'read_rule', '3': 3, '4': 1, '5': 9, '10': 'readRule'},
  ],
};

/// Descriptor for `AddGroupIntoCollectionRuleRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List addGroupIntoCollectionRuleRequestDescriptor = $convert.base64Decode('CiFBZGRHcm91cEludG9Db2xsZWN0aW9uUnVsZVJlcXVlc3QSGwoJbWFuYWdlX2lkGAEgASgFUghtYW5hZ2VJZBIZCghncm91cF9pZBgCIAEoCVIHZ3JvdXBJZBIbCglyZWFkX3J1bGUYAyABKAlSCHJlYWRSdWxl');
@$core.Deprecated('Use addGroupIntoCollectionWriteRuleResponseDescriptor instead')
const AddGroupIntoCollectionWriteRuleResponse$json = const {
  '1': 'AddGroupIntoCollectionWriteRuleResponse',
  '2': const [
    const {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `AddGroupIntoCollectionWriteRuleResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List addGroupIntoCollectionWriteRuleResponseDescriptor = $convert.base64Decode('CidBZGRHcm91cEludG9Db2xsZWN0aW9uV3JpdGVSdWxlUmVzcG9uc2USFgoGcmVzdWx0GAEgASgJUgZyZXN1bHQ=');
