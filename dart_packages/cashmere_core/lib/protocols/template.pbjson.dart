//
//  Generated code. Do not modify.
//  source: template.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use newTemplateRequestDescriptor instead')
const NewTemplateRequest$json = {
  '1': 'NewTemplateRequest',
  '2': [
    {'1': 'name', '3': 3, '4': 1, '5': 11, '6': '.cashmere.Name', '10': 'name'},
    {'1': 'manage_id', '3': 1, '4': 1, '5': 5, '10': 'manageId'},
    {'1': 'fields', '3': 2, '4': 3, '5': 12, '10': 'fields'},
  ],
};

/// Descriptor for `NewTemplateRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List newTemplateRequestDescriptor = $convert.base64Decode(
    'ChJOZXdUZW1wbGF0ZVJlcXVlc3QSIgoEbmFtZRgDIAEoCzIOLmNhc2htZXJlLk5hbWVSBG5hbW'
    'USGwoJbWFuYWdlX2lkGAEgASgFUghtYW5hZ2VJZBIWCgZmaWVsZHMYAiADKAxSBmZpZWxkcw==');

@$core.Deprecated('Use newTemplateResponseDescriptor instead')
const NewTemplateResponse$json = {
  '1': 'NewTemplateResponse',
  '2': [
    {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `NewTemplateResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List newTemplateResponseDescriptor = $convert.base64Decode(
    'ChNOZXdUZW1wbGF0ZVJlc3BvbnNlEhYKBnJlc3VsdBgBIAEoCVIGcmVzdWx0');

@$core.Deprecated('Use editTemplateRequestDescriptor instead')
const EditTemplateRequest$json = {
  '1': 'EditTemplateRequest',
  '2': [
    {'1': 'template_id', '3': 1, '4': 1, '5': 9, '10': 'templateId'},
    {'1': 'fields', '3': 2, '4': 3, '5': 12, '10': 'fields'},
  ],
};

/// Descriptor for `EditTemplateRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List editTemplateRequestDescriptor = $convert.base64Decode(
    'ChNFZGl0VGVtcGxhdGVSZXF1ZXN0Eh8KC3RlbXBsYXRlX2lkGAEgASgJUgp0ZW1wbGF0ZUlkEh'
    'YKBmZpZWxkcxgCIAMoDFIGZmllbGRz');

@$core.Deprecated('Use editTemplateResponseDescriptor instead')
const EditTemplateResponse$json = {
  '1': 'EditTemplateResponse',
  '2': [
    {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `EditTemplateResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List editTemplateResponseDescriptor = $convert.base64Decode(
    'ChRFZGl0VGVtcGxhdGVSZXNwb25zZRIWCgZyZXN1bHQYASABKAlSBnJlc3VsdA==');

@$core.Deprecated('Use removeTemplateRequestDescriptor instead')
const RemoveTemplateRequest$json = {
  '1': 'RemoveTemplateRequest',
  '2': [
    {'1': 'template_id', '3': 1, '4': 1, '5': 9, '10': 'templateId'},
  ],
};

/// Descriptor for `RemoveTemplateRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List removeTemplateRequestDescriptor = $convert.base64Decode(
    'ChVSZW1vdmVUZW1wbGF0ZVJlcXVlc3QSHwoLdGVtcGxhdGVfaWQYASABKAlSCnRlbXBsYXRlSW'
    'Q=');

@$core.Deprecated('Use removeTemplateResponseDescriptor instead')
const RemoveTemplateResponse$json = {
  '1': 'RemoveTemplateResponse',
  '2': [
    {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `RemoveTemplateResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List removeTemplateResponseDescriptor = $convert.base64Decode(
    'ChZSZW1vdmVUZW1wbGF0ZVJlc3BvbnNlEhYKBnJlc3VsdBgBIAEoCVIGcmVzdWx0');

