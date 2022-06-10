///
//  Generated code. Do not modify.
//  source: manage.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields,deprecated_member_use_from_same_package

import 'dart:core' as $core;
import 'dart:convert' as $convert;
import 'dart:typed_data' as $typed_data;
@$core.Deprecated('Use fieldDescriptor instead')
const Field$json = const {
  '1': 'Field',
  '2': const [
    const {'1': 'id', '3': 1, '4': 1, '5': 5, '10': 'id'},
    const {'1': 'name', '3': 2, '4': 1, '5': 12, '10': 'name'},
    const {'1': 'data_type', '3': 3, '4': 1, '5': 9, '10': 'dataType'},
    const {'1': 'removed', '3': 4, '4': 1, '5': 8, '10': 'removed'},
  ],
};

/// Descriptor for `Field`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List fieldDescriptor = $convert.base64Decode('CgVGaWVsZBIOCgJpZBgBIAEoBVICaWQSEgoEbmFtZRgCIAEoDFIEbmFtZRIbCglkYXRhX3R5cGUYAyABKAlSCGRhdGFUeXBlEhgKB3JlbW92ZWQYBCABKAhSB3JlbW92ZWQ=');
@$core.Deprecated('Use getManagesRequestDescriptor instead')
const GetManagesRequest$json = const {
  '1': 'GetManagesRequest',
};

/// Descriptor for `GetManagesRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getManagesRequestDescriptor = $convert.base64Decode('ChFHZXRNYW5hZ2VzUmVxdWVzdA==');
@$core.Deprecated('Use getManageSchemaRequestDescriptor instead')
const GetManageSchemaRequest$json = const {
  '1': 'GetManageSchemaRequest',
  '2': const [
    const {'1': 'manage_id', '3': 1, '4': 1, '5': 9, '10': 'manageId'},
  ],
};

/// Descriptor for `GetManageSchemaRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getManageSchemaRequestDescriptor = $convert.base64Decode('ChZHZXRNYW5hZ2VTY2hlbWFSZXF1ZXN0EhsKCW1hbmFnZV9pZBgBIAEoCVIIbWFuYWdlSWQ=');
@$core.Deprecated('Use getManageSchemaResponseDescriptor instead')
const GetManageSchemaResponse$json = const {
  '1': 'GetManageSchemaResponse',
  '2': const [
    const {'1': 'schema', '3': 1, '4': 3, '5': 12, '10': 'schema'},
  ],
};

/// Descriptor for `GetManageSchemaResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getManageSchemaResponseDescriptor = $convert.base64Decode('ChdHZXRNYW5hZ2VTY2hlbWFSZXNwb25zZRIWCgZzY2hlbWEYASADKAxSBnNjaGVtYQ==');
@$core.Deprecated('Use newSchemaFieldRequestDescriptor instead')
const NewSchemaFieldRequest$json = const {
  '1': 'NewSchemaFieldRequest',
  '2': const [
    const {'1': 'manage_id', '3': 1, '4': 1, '5': 9, '10': 'manageId'},
    const {'1': 'field', '3': 2, '4': 1, '5': 11, '6': '.cashmere.Field', '10': 'field'},
  ],
};

/// Descriptor for `NewSchemaFieldRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List newSchemaFieldRequestDescriptor = $convert.base64Decode('ChVOZXdTY2hlbWFGaWVsZFJlcXVlc3QSGwoJbWFuYWdlX2lkGAEgASgJUghtYW5hZ2VJZBIlCgVmaWVsZBgCIAEoCzIPLmNhc2htZXJlLkZpZWxkUgVmaWVsZA==');
@$core.Deprecated('Use newSchemaFieldResponseDescriptor instead')
const NewSchemaFieldResponse$json = const {
  '1': 'NewSchemaFieldResponse',
  '2': const [
    const {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `NewSchemaFieldResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List newSchemaFieldResponseDescriptor = $convert.base64Decode('ChZOZXdTY2hlbWFGaWVsZFJlc3BvbnNlEhYKBnJlc3VsdBgBIAEoCVIGcmVzdWx0');
@$core.Deprecated('Use removeSchemaFieldRequestDescriptor instead')
const RemoveSchemaFieldRequest$json = const {
  '1': 'RemoveSchemaFieldRequest',
  '2': const [
    const {'1': 'manage_id', '3': 1, '4': 1, '5': 9, '10': 'manageId'},
    const {'1': 'field_id', '3': 2, '4': 1, '5': 5, '10': 'fieldId'},
  ],
};

/// Descriptor for `RemoveSchemaFieldRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List removeSchemaFieldRequestDescriptor = $convert.base64Decode('ChhSZW1vdmVTY2hlbWFGaWVsZFJlcXVlc3QSGwoJbWFuYWdlX2lkGAEgASgJUghtYW5hZ2VJZBIZCghmaWVsZF9pZBgCIAEoBVIHZmllbGRJZA==');
@$core.Deprecated('Use removeSchemaFieldResponseDescriptor instead')
const RemoveSchemaFieldResponse$json = const {
  '1': 'RemoveSchemaFieldResponse',
  '2': const [
    const {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `RemoveSchemaFieldResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List removeSchemaFieldResponseDescriptor = $convert.base64Decode('ChlSZW1vdmVTY2hlbWFGaWVsZFJlc3BvbnNlEhYKBnJlc3VsdBgBIAEoCVIGcmVzdWx0');
@$core.Deprecated('Use editSchemaFieldNameRequestDescriptor instead')
const EditSchemaFieldNameRequest$json = const {
  '1': 'EditSchemaFieldNameRequest',
  '2': const [
    const {'1': 'manage_id', '3': 1, '4': 1, '5': 9, '10': 'manageId'},
    const {'1': 'field_id', '3': 2, '4': 1, '5': 5, '10': 'fieldId'},
    const {'1': 'language', '3': 3, '4': 1, '5': 9, '10': 'language'},
    const {'1': 'new_name', '3': 4, '4': 1, '5': 9, '10': 'newName'},
  ],
};

/// Descriptor for `EditSchemaFieldNameRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List editSchemaFieldNameRequestDescriptor = $convert.base64Decode('ChpFZGl0U2NoZW1hRmllbGROYW1lUmVxdWVzdBIbCgltYW5hZ2VfaWQYASABKAlSCG1hbmFnZUlkEhkKCGZpZWxkX2lkGAIgASgFUgdmaWVsZElkEhoKCGxhbmd1YWdlGAMgASgJUghsYW5ndWFnZRIZCghuZXdfbmFtZRgEIAEoCVIHbmV3TmFtZQ==');
@$core.Deprecated('Use editSchemaFieldNameResponseDescriptor instead')
const EditSchemaFieldNameResponse$json = const {
  '1': 'EditSchemaFieldNameResponse',
  '2': const [
    const {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `EditSchemaFieldNameResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List editSchemaFieldNameResponseDescriptor = $convert.base64Decode('ChtFZGl0U2NoZW1hRmllbGROYW1lUmVzcG9uc2USFgoGcmVzdWx0GAEgASgJUgZyZXN1bHQ=');
@$core.Deprecated('Use getManageEntryCountRequestDescriptor instead')
const GetManageEntryCountRequest$json = const {
  '1': 'GetManageEntryCountRequest',
  '2': const [
    const {'1': 'manage_id', '3': 1, '4': 1, '5': 9, '10': 'manageId'},
  ],
};

/// Descriptor for `GetManageEntryCountRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getManageEntryCountRequestDescriptor = $convert.base64Decode('ChpHZXRNYW5hZ2VFbnRyeUNvdW50UmVxdWVzdBIbCgltYW5hZ2VfaWQYASABKAlSCG1hbmFnZUlk');
@$core.Deprecated('Use getManageEntryCountResponseDescriptor instead')
const GetManageEntryCountResponse$json = const {
  '1': 'GetManageEntryCountResponse',
  '2': const [
    const {'1': 'count', '3': 1, '4': 1, '5': 4, '10': 'count'},
  ],
};

/// Descriptor for `GetManageEntryCountResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getManageEntryCountResponseDescriptor = $convert.base64Decode('ChtHZXRNYW5hZ2VFbnRyeUNvdW50UmVzcG9uc2USFAoFY291bnQYASABKARSBWNvdW50');
