//
//  Generated code. Do not modify.
//  source: view.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use getManageViewRequestDescriptor instead')
const GetManageViewRequest$json = {
  '1': 'GetManageViewRequest',
  '2': [
    {'1': 'manage_name', '3': 1, '4': 1, '5': 9, '10': 'manageName'},
  ],
};

/// Descriptor for `GetManageViewRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getManageViewRequestDescriptor = $convert.base64Decode(
    'ChRHZXRNYW5hZ2VWaWV3UmVxdWVzdBIfCgttYW5hZ2VfbmFtZRgBIAEoCVIKbWFuYWdlTmFtZQ'
    '==');

@$core.Deprecated('Use getManageViewResponseDescriptor instead')
const GetManageViewResponse$json = {
  '1': 'GetManageViewResponse',
  '2': [
    {'1': 'view_token', '3': 1, '4': 1, '5': 9, '10': 'viewToken'},
  ],
};

/// Descriptor for `GetManageViewResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getManageViewResponseDescriptor = $convert.base64Decode(
    'ChVHZXRNYW5hZ2VWaWV3UmVzcG9uc2USHQoKdmlld190b2tlbhgBIAEoCVIJdmlld1Rva2Vu');

@$core.Deprecated('Use getSchemaViewRulesMapRequestDescriptor instead')
const GetSchemaViewRulesMapRequest$json = {
  '1': 'GetSchemaViewRulesMapRequest',
  '2': [
    {'1': 'manage_id', '3': 1, '4': 1, '5': 5, '10': 'manageId'},
    {'1': 'group_id', '3': 2, '4': 1, '5': 9, '10': 'groupId'},
  ],
};

/// Descriptor for `GetSchemaViewRulesMapRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getSchemaViewRulesMapRequestDescriptor = $convert.base64Decode(
    'ChxHZXRTY2hlbWFWaWV3UnVsZXNNYXBSZXF1ZXN0EhsKCW1hbmFnZV9pZBgBIAEoBVIIbWFuYW'
    'dlSWQSGQoIZ3JvdXBfaWQYAiABKAlSB2dyb3VwSWQ=');

@$core.Deprecated('Use getSchemaViewRulesMapResponseDescriptor instead')
const GetSchemaViewRulesMapResponse$json = {
  '1': 'GetSchemaViewRulesMapResponse',
  '2': [
    {'1': 'rules_map', '3': 1, '4': 1, '5': 12, '10': 'rulesMap'},
  ],
};

/// Descriptor for `GetSchemaViewRulesMapResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getSchemaViewRulesMapResponseDescriptor = $convert.base64Decode(
    'Ch1HZXRTY2hlbWFWaWV3UnVsZXNNYXBSZXNwb25zZRIbCglydWxlc19tYXAYASABKAxSCHJ1bG'
    'VzTWFw');

@$core.Deprecated('Use changeManageReadRuleRequestDescriptor instead')
const ChangeManageReadRuleRequest$json = {
  '1': 'ChangeManageReadRuleRequest',
  '2': [
    {'1': 'manage_id', '3': 1, '4': 1, '5': 5, '10': 'manageId'},
    {'1': 'group_id', '3': 2, '4': 1, '5': 9, '10': 'groupId'},
    {'1': 'read_rule', '3': 3, '4': 1, '5': 9, '10': 'readRule'},
  ],
};

/// Descriptor for `ChangeManageReadRuleRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List changeManageReadRuleRequestDescriptor = $convert.base64Decode(
    'ChtDaGFuZ2VNYW5hZ2VSZWFkUnVsZVJlcXVlc3QSGwoJbWFuYWdlX2lkGAEgASgFUghtYW5hZ2'
    'VJZBIZCghncm91cF9pZBgCIAEoCVIHZ3JvdXBJZBIbCglyZWFkX3J1bGUYAyABKAlSCHJlYWRS'
    'dWxl');

@$core.Deprecated('Use changeManageReadRuleResponseDescriptor instead')
const ChangeManageReadRuleResponse$json = {
  '1': 'ChangeManageReadRuleResponse',
  '2': [
    {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `ChangeManageReadRuleResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List changeManageReadRuleResponseDescriptor = $convert.base64Decode(
    'ChxDaGFuZ2VNYW5hZ2VSZWFkUnVsZVJlc3BvbnNlEhYKBnJlc3VsdBgBIAEoCVIGcmVzdWx0');

@$core.Deprecated('Use changeManageWriteRuleRequestDescriptor instead')
const ChangeManageWriteRuleRequest$json = {
  '1': 'ChangeManageWriteRuleRequest',
  '2': [
    {'1': 'manage_id', '3': 1, '4': 1, '5': 5, '10': 'manageId'},
    {'1': 'group_id', '3': 2, '4': 1, '5': 9, '10': 'groupId'},
    {'1': 'write_rule', '3': 3, '4': 1, '5': 9, '10': 'writeRule'},
  ],
};

/// Descriptor for `ChangeManageWriteRuleRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List changeManageWriteRuleRequestDescriptor = $convert.base64Decode(
    'ChxDaGFuZ2VNYW5hZ2VXcml0ZVJ1bGVSZXF1ZXN0EhsKCW1hbmFnZV9pZBgBIAEoBVIIbWFuYW'
    'dlSWQSGQoIZ3JvdXBfaWQYAiABKAlSB2dyb3VwSWQSHQoKd3JpdGVfcnVsZRgDIAEoCVIJd3Jp'
    'dGVSdWxl');

@$core.Deprecated('Use changeManageWriteRuleResponseDescriptor instead')
const ChangeManageWriteRuleResponse$json = {
  '1': 'ChangeManageWriteRuleResponse',
  '2': [
    {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `ChangeManageWriteRuleResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List changeManageWriteRuleResponseDescriptor = $convert.base64Decode(
    'Ch1DaGFuZ2VNYW5hZ2VXcml0ZVJ1bGVSZXNwb25zZRIWCgZyZXN1bHQYASABKAlSBnJlc3VsdA'
    '==');

@$core.Deprecated('Use changeCollectionReadRuleRequestDescriptor instead')
const ChangeCollectionReadRuleRequest$json = {
  '1': 'ChangeCollectionReadRuleRequest',
  '2': [
    {'1': 'manage_id', '3': 1, '4': 1, '5': 5, '10': 'manageId'},
    {'1': 'group_id', '3': 2, '4': 1, '5': 9, '10': 'groupId'},
    {'1': 'read_rule', '3': 3, '4': 1, '5': 9, '10': 'readRule'},
  ],
};

/// Descriptor for `ChangeCollectionReadRuleRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List changeCollectionReadRuleRequestDescriptor = $convert.base64Decode(
    'Ch9DaGFuZ2VDb2xsZWN0aW9uUmVhZFJ1bGVSZXF1ZXN0EhsKCW1hbmFnZV9pZBgBIAEoBVIIbW'
    'FuYWdlSWQSGQoIZ3JvdXBfaWQYAiABKAlSB2dyb3VwSWQSGwoJcmVhZF9ydWxlGAMgASgJUghy'
    'ZWFkUnVsZQ==');

@$core.Deprecated('Use changeCollectionReadRuleResponseDescriptor instead')
const ChangeCollectionReadRuleResponse$json = {
  '1': 'ChangeCollectionReadRuleResponse',
  '2': [
    {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `ChangeCollectionReadRuleResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List changeCollectionReadRuleResponseDescriptor = $convert.base64Decode(
    'CiBDaGFuZ2VDb2xsZWN0aW9uUmVhZFJ1bGVSZXNwb25zZRIWCgZyZXN1bHQYASABKAlSBnJlc3'
    'VsdA==');

@$core.Deprecated('Use changeCollectionWriteRuleRequestDescriptor instead')
const ChangeCollectionWriteRuleRequest$json = {
  '1': 'ChangeCollectionWriteRuleRequest',
  '2': [
    {'1': 'manage_id', '3': 1, '4': 1, '5': 5, '10': 'manageId'},
    {'1': 'group_id', '3': 2, '4': 1, '5': 9, '10': 'groupId'},
    {'1': 'write_rule', '3': 3, '4': 1, '5': 9, '10': 'writeRule'},
  ],
};

/// Descriptor for `ChangeCollectionWriteRuleRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List changeCollectionWriteRuleRequestDescriptor = $convert.base64Decode(
    'CiBDaGFuZ2VDb2xsZWN0aW9uV3JpdGVSdWxlUmVxdWVzdBIbCgltYW5hZ2VfaWQYASABKAVSCG'
    '1hbmFnZUlkEhkKCGdyb3VwX2lkGAIgASgJUgdncm91cElkEh0KCndyaXRlX3J1bGUYAyABKAlS'
    'CXdyaXRlUnVsZQ==');

@$core.Deprecated('Use changeCollectionWriteRuleResponseDescriptor instead')
const ChangeCollectionWriteRuleResponse$json = {
  '1': 'ChangeCollectionWriteRuleResponse',
  '2': [
    {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `ChangeCollectionWriteRuleResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List changeCollectionWriteRuleResponseDescriptor = $convert.base64Decode(
    'CiFDaGFuZ2VDb2xsZWN0aW9uV3JpdGVSdWxlUmVzcG9uc2USFgoGcmVzdWx0GAEgASgJUgZyZX'
    'N1bHQ=');

@$core.Deprecated('Use changeFieldReadRuleRequestDescriptor instead')
const ChangeFieldReadRuleRequest$json = {
  '1': 'ChangeFieldReadRuleRequest',
  '2': [
    {'1': 'manage_id', '3': 1, '4': 1, '5': 5, '10': 'manageId'},
    {'1': 'group_id', '3': 2, '4': 1, '5': 9, '10': 'groupId'},
    {'1': 'field_id', '3': 3, '4': 1, '5': 9, '10': 'fieldId'},
    {'1': 'read_rule', '3': 4, '4': 1, '5': 9, '10': 'readRule'},
  ],
};

/// Descriptor for `ChangeFieldReadRuleRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List changeFieldReadRuleRequestDescriptor = $convert.base64Decode(
    'ChpDaGFuZ2VGaWVsZFJlYWRSdWxlUmVxdWVzdBIbCgltYW5hZ2VfaWQYASABKAVSCG1hbmFnZU'
    'lkEhkKCGdyb3VwX2lkGAIgASgJUgdncm91cElkEhkKCGZpZWxkX2lkGAMgASgJUgdmaWVsZElk'
    'EhsKCXJlYWRfcnVsZRgEIAEoCVIIcmVhZFJ1bGU=');

@$core.Deprecated('Use changeFieldReadRuleResponseDescriptor instead')
const ChangeFieldReadRuleResponse$json = {
  '1': 'ChangeFieldReadRuleResponse',
  '2': [
    {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `ChangeFieldReadRuleResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List changeFieldReadRuleResponseDescriptor = $convert.base64Decode(
    'ChtDaGFuZ2VGaWVsZFJlYWRSdWxlUmVzcG9uc2USFgoGcmVzdWx0GAEgASgJUgZyZXN1bHQ=');

@$core.Deprecated('Use changeFieldWriteRuleRequestDescriptor instead')
const ChangeFieldWriteRuleRequest$json = {
  '1': 'ChangeFieldWriteRuleRequest',
  '2': [
    {'1': 'manage_id', '3': 1, '4': 1, '5': 5, '10': 'manageId'},
    {'1': 'group_id', '3': 2, '4': 1, '5': 9, '10': 'groupId'},
    {'1': 'field_id', '3': 3, '4': 1, '5': 9, '10': 'fieldId'},
    {'1': 'write_rule', '3': 4, '4': 1, '5': 9, '10': 'writeRule'},
  ],
};

/// Descriptor for `ChangeFieldWriteRuleRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List changeFieldWriteRuleRequestDescriptor = $convert.base64Decode(
    'ChtDaGFuZ2VGaWVsZFdyaXRlUnVsZVJlcXVlc3QSGwoJbWFuYWdlX2lkGAEgASgFUghtYW5hZ2'
    'VJZBIZCghncm91cF9pZBgCIAEoCVIHZ3JvdXBJZBIZCghmaWVsZF9pZBgDIAEoCVIHZmllbGRJ'
    'ZBIdCgp3cml0ZV9ydWxlGAQgASgJUgl3cml0ZVJ1bGU=');

@$core.Deprecated('Use changeFieldWriteRuleResponseDescriptor instead')
const ChangeFieldWriteRuleResponse$json = {
  '1': 'ChangeFieldWriteRuleResponse',
  '2': [
    {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `ChangeFieldWriteRuleResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List changeFieldWriteRuleResponseDescriptor = $convert.base64Decode(
    'ChxDaGFuZ2VGaWVsZFdyaXRlUnVsZVJlc3BvbnNlEhYKBnJlc3VsdBgBIAEoCVIGcmVzdWx0');

