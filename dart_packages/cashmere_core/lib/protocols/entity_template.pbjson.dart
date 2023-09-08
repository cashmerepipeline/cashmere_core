//
//  Generated code. Do not modify.
//  source: entity_template.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use newEntityTemplateRequestDescriptor instead')
const NewEntityTemplateRequest$json = {
  '1': 'NewEntityTemplateRequest',
  '2': [
    {'1': 'manage_id', '3': 1, '4': 1, '5': 5, '10': 'manageId'},
    {'1': 'fields', '3': 2, '4': 3, '5': 12, '10': 'fields'},
  ],
};

/// Descriptor for `NewEntityTemplateRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List newEntityTemplateRequestDescriptor = $convert.base64Decode(
    'ChhOZXdFbnRpdHlUZW1wbGF0ZVJlcXVlc3QSGwoJbWFuYWdlX2lkGAEgASgFUghtYW5hZ2VJZB'
    'IWCgZmaWVsZHMYAiADKAxSBmZpZWxkcw==');

@$core.Deprecated('Use newEntityTemplateResponseDescriptor instead')
const NewEntityTemplateResponse$json = {
  '1': 'NewEntityTemplateResponse',
  '2': [
    {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `NewEntityTemplateResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List newEntityTemplateResponseDescriptor = $convert.base64Decode(
    'ChlOZXdFbnRpdHlUZW1wbGF0ZVJlc3BvbnNlEhYKBnJlc3VsdBgBIAEoCVIGcmVzdWx0');

@$core.Deprecated('Use editEntityTemplateRequestDescriptor instead')
const EditEntityTemplateRequest$json = {
  '1': 'EditEntityTemplateRequest',
  '2': [
    {'1': 'template_id', '3': 1, '4': 1, '5': 9, '10': 'templateId'},
    {'1': 'fields', '3': 2, '4': 3, '5': 12, '10': 'fields'},
  ],
};

/// Descriptor for `EditEntityTemplateRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List editEntityTemplateRequestDescriptor = $convert.base64Decode(
    'ChlFZGl0RW50aXR5VGVtcGxhdGVSZXF1ZXN0Eh8KC3RlbXBsYXRlX2lkGAEgASgJUgp0ZW1wbG'
    'F0ZUlkEhYKBmZpZWxkcxgCIAMoDFIGZmllbGRz');

@$core.Deprecated('Use editEntityTemplateResponseDescriptor instead')
const EditEntityTemplateResponse$json = {
  '1': 'EditEntityTemplateResponse',
  '2': [
    {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `EditEntityTemplateResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List editEntityTemplateResponseDescriptor = $convert.base64Decode(
    'ChpFZGl0RW50aXR5VGVtcGxhdGVSZXNwb25zZRIWCgZyZXN1bHQYASABKAlSBnJlc3VsdA==');

@$core.Deprecated('Use removeEntityTemplateRequestDescriptor instead')
const RemoveEntityTemplateRequest$json = {
  '1': 'RemoveEntityTemplateRequest',
  '2': [
    {'1': 'template_id', '3': 1, '4': 1, '5': 9, '10': 'templateId'},
  ],
};

/// Descriptor for `RemoveEntityTemplateRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List removeEntityTemplateRequestDescriptor = $convert.base64Decode(
    'ChtSZW1vdmVFbnRpdHlUZW1wbGF0ZVJlcXVlc3QSHwoLdGVtcGxhdGVfaWQYASABKAlSCnRlbX'
    'BsYXRlSWQ=');

@$core.Deprecated('Use removeEntityTemplateResponseDescriptor instead')
const RemoveEntityTemplateResponse$json = {
  '1': 'RemoveEntityTemplateResponse',
  '2': [
    {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `RemoveEntityTemplateResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List removeEntityTemplateResponseDescriptor = $convert.base64Decode(
    'ChxSZW1vdmVFbnRpdHlUZW1wbGF0ZVJlc3BvbnNlEhYKBnJlc3VsdBgBIAEoCVIGcmVzdWx0');

