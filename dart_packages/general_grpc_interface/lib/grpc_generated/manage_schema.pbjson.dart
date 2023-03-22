///
//  Generated code. Do not modify.
//  source: manage_schema.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,deprecated_member_use_from_same_package,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

import 'dart:core' as $core;
import 'dart:convert' as $convert;
import 'dart:typed_data' as $typed_data;
@$core.Deprecated('Use schemaFieldDescriptor instead')
const SchemaField$json = const {
  '1': 'SchemaField',
  '2': const [
    const {'1': 'id', '3': 1, '4': 1, '5': 5, '10': 'id'},
    const {'1': 'name_map', '3': 2, '4': 1, '5': 12, '10': 'nameMap'},
    const {'1': 'data_type', '3': 3, '4': 1, '5': 9, '10': 'dataType'},
    const {'1': 'removed', '3': 4, '4': 1, '5': 8, '10': 'removed'},
  ],
};

/// Descriptor for `SchemaField`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List schemaFieldDescriptor = $convert.base64Decode('CgtTY2hlbWFGaWVsZBIOCgJpZBgBIAEoBVICaWQSGQoIbmFtZV9tYXAYAiABKAxSB25hbWVNYXASGwoJZGF0YV90eXBlGAMgASgJUghkYXRhVHlwZRIYCgdyZW1vdmVkGAQgASgIUgdyZW1vdmVk');
@$core.Deprecated('Use getManageSchemaRequestDescriptor instead')
const GetManageSchemaRequest$json = const {
  '1': 'GetManageSchemaRequest',
  '2': const [
    const {'1': 'manage_id', '3': 1, '4': 1, '5': 5, '10': 'manageId'},
  ],
};

/// Descriptor for `GetManageSchemaRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getManageSchemaRequestDescriptor = $convert.base64Decode('ChZHZXRNYW5hZ2VTY2hlbWFSZXF1ZXN0EhsKCW1hbmFnZV9pZBgBIAEoBVIIbWFuYWdlSWQ=');
@$core.Deprecated('Use getManageSchemaResponseDescriptor instead')
const GetManageSchemaResponse$json = const {
  '1': 'GetManageSchemaResponse',
  '2': const [
    const {'1': 'fields', '3': 1, '4': 3, '5': 11, '6': '.cashmere.SchemaField', '10': 'fields'},
  ],
};

/// Descriptor for `GetManageSchemaResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getManageSchemaResponseDescriptor = $convert.base64Decode('ChdHZXRNYW5hZ2VTY2hlbWFSZXNwb25zZRItCgZmaWVsZHMYASADKAsyFS5jYXNobWVyZS5TY2hlbWFGaWVsZFIGZmllbGRz');
@$core.Deprecated('Use newSchemaFieldRequestDescriptor instead')
const NewSchemaFieldRequest$json = const {
  '1': 'NewSchemaFieldRequest',
  '2': const [
    const {'1': 'manage_id', '3': 1, '4': 1, '5': 5, '10': 'manageId'},
    const {'1': 'field', '3': 2, '4': 1, '5': 11, '6': '.cashmere.SchemaField', '10': 'field'},
  ],
};

/// Descriptor for `NewSchemaFieldRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List newSchemaFieldRequestDescriptor = $convert.base64Decode('ChVOZXdTY2hlbWFGaWVsZFJlcXVlc3QSGwoJbWFuYWdlX2lkGAEgASgFUghtYW5hZ2VJZBIrCgVmaWVsZBgCIAEoCzIVLmNhc2htZXJlLlNjaGVtYUZpZWxkUgVmaWVsZA==');
@$core.Deprecated('Use newSchemaFieldResponseDescriptor instead')
const NewSchemaFieldResponse$json = const {
  '1': 'NewSchemaFieldResponse',
  '2': const [
    const {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `NewSchemaFieldResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List newSchemaFieldResponseDescriptor = $convert.base64Decode('ChZOZXdTY2hlbWFGaWVsZFJlc3BvbnNlEhYKBnJlc3VsdBgBIAEoCVIGcmVzdWx0');
@$core.Deprecated('Use markSchemaFieldRemovedRequestDescriptor instead')
const MarkSchemaFieldRemovedRequest$json = const {
  '1': 'MarkSchemaFieldRemovedRequest',
  '2': const [
    const {'1': 'manage_id', '3': 1, '4': 1, '5': 5, '10': 'manageId'},
    const {'1': 'field_id', '3': 2, '4': 1, '5': 5, '10': 'fieldId'},
  ],
};

/// Descriptor for `MarkSchemaFieldRemovedRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List markSchemaFieldRemovedRequestDescriptor = $convert.base64Decode('Ch1NYXJrU2NoZW1hRmllbGRSZW1vdmVkUmVxdWVzdBIbCgltYW5hZ2VfaWQYASABKAVSCG1hbmFnZUlkEhkKCGZpZWxkX2lkGAIgASgFUgdmaWVsZElk');
@$core.Deprecated('Use markSchemaFieldRemovedResponseDescriptor instead')
const MarkSchemaFieldRemovedResponse$json = const {
  '1': 'MarkSchemaFieldRemovedResponse',
  '2': const [
    const {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `MarkSchemaFieldRemovedResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List markSchemaFieldRemovedResponseDescriptor = $convert.base64Decode('Ch5NYXJrU2NoZW1hRmllbGRSZW1vdmVkUmVzcG9uc2USFgoGcmVzdWx0GAEgASgJUgZyZXN1bHQ=');
@$core.Deprecated('Use editSchemaFieldNameRequestDescriptor instead')
const EditSchemaFieldNameRequest$json = const {
  '1': 'EditSchemaFieldNameRequest',
  '2': const [
    const {'1': 'manage_id', '3': 1, '4': 1, '5': 5, '10': 'manageId'},
    const {'1': 'field_id', '3': 2, '4': 1, '5': 5, '10': 'fieldId'},
    const {'1': 'language', '3': 3, '4': 1, '5': 9, '10': 'language'},
    const {'1': 'new_name', '3': 4, '4': 1, '5': 9, '10': 'newName'},
  ],
};

/// Descriptor for `EditSchemaFieldNameRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List editSchemaFieldNameRequestDescriptor = $convert.base64Decode('ChpFZGl0U2NoZW1hRmllbGROYW1lUmVxdWVzdBIbCgltYW5hZ2VfaWQYASABKAVSCG1hbmFnZUlkEhkKCGZpZWxkX2lkGAIgASgFUgdmaWVsZElkEhoKCGxhbmd1YWdlGAMgASgJUghsYW5ndWFnZRIZCghuZXdfbmFtZRgEIAEoCVIHbmV3TmFtZQ==');
@$core.Deprecated('Use editSchemaFieldNameResponseDescriptor instead')
const EditSchemaFieldNameResponse$json = const {
  '1': 'EditSchemaFieldNameResponse',
  '2': const [
    const {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `EditSchemaFieldNameResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List editSchemaFieldNameResponseDescriptor = $convert.base64Decode('ChtFZGl0U2NoZW1hRmllbGROYW1lUmVzcG9uc2USFgoGcmVzdWx0GAEgASgJUgZyZXN1bHQ=');
