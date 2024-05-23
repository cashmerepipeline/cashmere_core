//
//  Generated code. Do not modify.
//  source: manage_schema.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use schemaFieldDescriptor instead')
const SchemaField$json = {
  '1': 'SchemaField',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 5, '10': 'id'},
    {'1': 'name_map', '3': 2, '4': 3, '5': 11, '6': '.cashmere.SchemaField.NameMapEntry', '10': 'nameMap'},
    {'1': 'data_type', '3': 3, '4': 1, '5': 9, '10': 'dataType'},
    {'1': 'removed', '3': 4, '4': 1, '5': 8, '10': 'removed'},
    {'1': 'editable', '3': 5, '4': 1, '5': 8, '10': 'editable'},
  ],
  '3': [SchemaField_NameMapEntry$json],
};

@$core.Deprecated('Use schemaFieldDescriptor instead')
const SchemaField_NameMapEntry$json = {
  '1': 'NameMapEntry',
  '2': [
    {'1': 'key', '3': 1, '4': 1, '5': 9, '10': 'key'},
    {'1': 'value', '3': 2, '4': 1, '5': 9, '10': 'value'},
  ],
  '7': {'7': true},
};

/// Descriptor for `SchemaField`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List schemaFieldDescriptor = $convert.base64Decode(
    'CgtTY2hlbWFGaWVsZBIOCgJpZBgBIAEoBVICaWQSPQoIbmFtZV9tYXAYAiADKAsyIi5jYXNobW'
    'VyZS5TY2hlbWFGaWVsZC5OYW1lTWFwRW50cnlSB25hbWVNYXASGwoJZGF0YV90eXBlGAMgASgJ'
    'UghkYXRhVHlwZRIYCgdyZW1vdmVkGAQgASgIUgdyZW1vdmVkEhoKCGVkaXRhYmxlGAUgASgIUg'
    'hlZGl0YWJsZRo6CgxOYW1lTWFwRW50cnkSEAoDa2V5GAEgASgJUgNrZXkSFAoFdmFsdWUYAiAB'
    'KAlSBXZhbHVlOgI4AQ==');

@$core.Deprecated('Use getManageSchemaRequestDescriptor instead')
const GetManageSchemaRequest$json = {
  '1': 'GetManageSchemaRequest',
  '2': [
    {'1': 'manage_id', '3': 1, '4': 1, '5': 9, '10': 'manageId'},
  ],
};

/// Descriptor for `GetManageSchemaRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getManageSchemaRequestDescriptor = $convert.base64Decode(
    'ChZHZXRNYW5hZ2VTY2hlbWFSZXF1ZXN0EhsKCW1hbmFnZV9pZBgBIAEoCVIIbWFuYWdlSWQ=');

@$core.Deprecated('Use getManageSchemaResponseDescriptor instead')
const GetManageSchemaResponse$json = {
  '1': 'GetManageSchemaResponse',
  '2': [
    {'1': 'fields', '3': 1, '4': 3, '5': 11, '6': '.cashmere.SchemaField', '10': 'fields'},
  ],
};

/// Descriptor for `GetManageSchemaResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getManageSchemaResponseDescriptor = $convert.base64Decode(
    'ChdHZXRNYW5hZ2VTY2hlbWFSZXNwb25zZRItCgZmaWVsZHMYASADKAsyFS5jYXNobWVyZS5TY2'
    'hlbWFGaWVsZFIGZmllbGRz');

@$core.Deprecated('Use newSchemaFieldRequestDescriptor instead')
const NewSchemaFieldRequest$json = {
  '1': 'NewSchemaFieldRequest',
  '2': [
    {'1': 'manage_id', '3': 1, '4': 1, '5': 9, '10': 'manageId'},
    {'1': 'new_field', '3': 2, '4': 1, '5': 11, '6': '.cashmere.SchemaField', '10': 'newField'},
  ],
};

/// Descriptor for `NewSchemaFieldRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List newSchemaFieldRequestDescriptor = $convert.base64Decode(
    'ChVOZXdTY2hlbWFGaWVsZFJlcXVlc3QSGwoJbWFuYWdlX2lkGAEgASgJUghtYW5hZ2VJZBIyCg'
    'luZXdfZmllbGQYAiABKAsyFS5jYXNobWVyZS5TY2hlbWFGaWVsZFIIbmV3RmllbGQ=');

@$core.Deprecated('Use newSchemaFieldResponseDescriptor instead')
const NewSchemaFieldResponse$json = {
  '1': 'NewSchemaFieldResponse',
  '2': [
    {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `NewSchemaFieldResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List newSchemaFieldResponseDescriptor = $convert.base64Decode(
    'ChZOZXdTY2hlbWFGaWVsZFJlc3BvbnNlEhYKBnJlc3VsdBgBIAEoCVIGcmVzdWx0');

@$core.Deprecated('Use markSchemaFieldRemovedRequestDescriptor instead')
const MarkSchemaFieldRemovedRequest$json = {
  '1': 'MarkSchemaFieldRemovedRequest',
  '2': [
    {'1': 'manage_id', '3': 1, '4': 1, '5': 9, '10': 'manageId'},
    {'1': 'field_id', '3': 2, '4': 1, '5': 5, '10': 'fieldId'},
  ],
};

/// Descriptor for `MarkSchemaFieldRemovedRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List markSchemaFieldRemovedRequestDescriptor = $convert.base64Decode(
    'Ch1NYXJrU2NoZW1hRmllbGRSZW1vdmVkUmVxdWVzdBIbCgltYW5hZ2VfaWQYASABKAlSCG1hbm'
    'FnZUlkEhkKCGZpZWxkX2lkGAIgASgFUgdmaWVsZElk');

@$core.Deprecated('Use markSchemaFieldRemovedResponseDescriptor instead')
const MarkSchemaFieldRemovedResponse$json = {
  '1': 'MarkSchemaFieldRemovedResponse',
  '2': [
    {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `MarkSchemaFieldRemovedResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List markSchemaFieldRemovedResponseDescriptor = $convert.base64Decode(
    'Ch5NYXJrU2NoZW1hRmllbGRSZW1vdmVkUmVzcG9uc2USFgoGcmVzdWx0GAEgASgJUgZyZXN1bH'
    'Q=');

@$core.Deprecated('Use editSchemaFieldNameRequestDescriptor instead')
const EditSchemaFieldNameRequest$json = {
  '1': 'EditSchemaFieldNameRequest',
  '2': [
    {'1': 'manage_id', '3': 1, '4': 1, '5': 9, '10': 'manageId'},
    {'1': 'field_id', '3': 2, '4': 1, '5': 5, '10': 'fieldId'},
    {'1': 'language', '3': 3, '4': 1, '5': 9, '10': 'language'},
    {'1': 'new_name', '3': 4, '4': 1, '5': 9, '10': 'newName'},
  ],
};

/// Descriptor for `EditSchemaFieldNameRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List editSchemaFieldNameRequestDescriptor = $convert.base64Decode(
    'ChpFZGl0U2NoZW1hRmllbGROYW1lUmVxdWVzdBIbCgltYW5hZ2VfaWQYASABKAlSCG1hbmFnZU'
    'lkEhkKCGZpZWxkX2lkGAIgASgFUgdmaWVsZElkEhoKCGxhbmd1YWdlGAMgASgJUghsYW5ndWFn'
    'ZRIZCghuZXdfbmFtZRgEIAEoCVIHbmV3TmFtZQ==');

@$core.Deprecated('Use editSchemaFieldNameResponseDescriptor instead')
const EditSchemaFieldNameResponse$json = {
  '1': 'EditSchemaFieldNameResponse',
  '2': [
    {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `EditSchemaFieldNameResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List editSchemaFieldNameResponseDescriptor = $convert.base64Decode(
    'ChtFZGl0U2NoZW1hRmllbGROYW1lUmVzcG9uc2USFgoGcmVzdWx0GAEgASgJUgZyZXN1bHQ=');

