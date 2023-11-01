//
//  Generated code. Do not modify.
//  source: name.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use nameDescriptor instead')
const Name$json = {
  '1': 'Name',
  '2': [
    {'1': 'language', '3': 1, '4': 1, '5': 9, '10': 'language'},
    {'1': 'name', '3': 2, '4': 1, '5': 9, '10': 'name'},
  ],
};

/// Descriptor for `Name`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List nameDescriptor = $convert.base64Decode(
    'CgROYW1lEhoKCGxhbmd1YWdlGAEgASgJUghsYW5ndWFnZRISCgRuYW1lGAIgASgJUgRuYW1l');

@$core.Deprecated('Use renameRequestDescriptor instead')
const RenameRequest$json = {
  '1': 'RenameRequest',
  '2': [
    {'1': 'manage_id', '3': 1, '4': 1, '5': 5, '10': 'manageId'},
    {'1': 'entity_id', '3': 2, '4': 1, '5': 9, '10': 'entityId'},
    {'1': 'new_name', '3': 3, '4': 1, '5': 11, '6': '.cashmere.Name', '10': 'newName'},
  ],
};

/// Descriptor for `RenameRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List renameRequestDescriptor = $convert.base64Decode(
    'Cg1SZW5hbWVSZXF1ZXN0EhsKCW1hbmFnZV9pZBgBIAEoBVIIbWFuYWdlSWQSGwoJZW50aXR5X2'
    'lkGAIgASgJUghlbnRpdHlJZBIpCghuZXdfbmFtZRgDIAEoCzIOLmNhc2htZXJlLk5hbWVSB25l'
    'd05hbWU=');

@$core.Deprecated('Use renameResponseDescriptor instead')
const RenameResponse$json = {
  '1': 'RenameResponse',
  '2': [
    {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `RenameResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List renameResponseDescriptor = $convert.base64Decode(
    'Cg5SZW5hbWVSZXNwb25zZRIWCgZyZXN1bHQYASABKAlSBnJlc3VsdA==');

@$core.Deprecated('Use newLanguageNameRequestDescriptor instead')
const NewLanguageNameRequest$json = {
  '1': 'NewLanguageNameRequest',
  '2': [
    {'1': 'manage_id', '3': 1, '4': 1, '5': 5, '10': 'manageId'},
    {'1': 'entity_id', '3': 2, '4': 1, '5': 9, '10': 'entityId'},
    {'1': 'new_name', '3': 3, '4': 1, '5': 11, '6': '.cashmere.Name', '10': 'newName'},
  ],
};

/// Descriptor for `NewLanguageNameRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List newLanguageNameRequestDescriptor = $convert.base64Decode(
    'ChZOZXdMYW5ndWFnZU5hbWVSZXF1ZXN0EhsKCW1hbmFnZV9pZBgBIAEoBVIIbWFuYWdlSWQSGw'
    'oJZW50aXR5X2lkGAIgASgJUghlbnRpdHlJZBIpCghuZXdfbmFtZRgDIAEoCzIOLmNhc2htZXJl'
    'Lk5hbWVSB25ld05hbWU=');

@$core.Deprecated('Use newLanguageNameResponseDescriptor instead')
const NewLanguageNameResponse$json = {
  '1': 'NewLanguageNameResponse',
  '2': [
    {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `NewLanguageNameResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List newLanguageNameResponseDescriptor = $convert.base64Decode(
    'ChdOZXdMYW5ndWFnZU5hbWVSZXNwb25zZRIWCgZyZXN1bHQYASABKAlSBnJlc3VsdA==');

@$core.Deprecated('Use removeLanguageNameRequestDescriptor instead')
const RemoveLanguageNameRequest$json = {
  '1': 'RemoveLanguageNameRequest',
  '2': [
    {'1': 'manage_id', '3': 1, '4': 1, '5': 5, '10': 'manageId'},
    {'1': 'entity_id', '3': 2, '4': 1, '5': 9, '10': 'entityId'},
    {'1': 'language', '3': 3, '4': 1, '5': 9, '10': 'language'},
  ],
};

/// Descriptor for `RemoveLanguageNameRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List removeLanguageNameRequestDescriptor = $convert.base64Decode(
    'ChlSZW1vdmVMYW5ndWFnZU5hbWVSZXF1ZXN0EhsKCW1hbmFnZV9pZBgBIAEoBVIIbWFuYWdlSW'
    'QSGwoJZW50aXR5X2lkGAIgASgJUghlbnRpdHlJZBIaCghsYW5ndWFnZRgDIAEoCVIIbGFuZ3Vh'
    'Z2U=');

@$core.Deprecated('Use removeLanguageNameResponseDescriptor instead')
const RemoveLanguageNameResponse$json = {
  '1': 'RemoveLanguageNameResponse',
  '2': [
    {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `RemoveLanguageNameResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List removeLanguageNameResponseDescriptor = $convert.base64Decode(
    'ChpSZW1vdmVMYW5ndWFnZU5hbWVSZXNwb25zZRIWCgZyZXN1bHQYASABKAlSBnJlc3VsdA==');

