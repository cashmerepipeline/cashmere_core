//
//  Generated code. Do not modify.
//  source: category.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use newCategoryRequestDescriptor instead')
const NewCategoryRequest$json = {
  '1': 'NewCategoryRequest',
  '2': [
    {'1': 'manage_id', '3': 1, '4': 1, '5': 9, '10': 'manageId'},
    {'1': 'code', '3': 4, '4': 1, '5': 9, '10': 'code'},
    {'1': 'name', '3': 2, '4': 1, '5': 11, '6': '.cashmere.Name', '10': 'name'},
    {'1': 'description', '3': 3, '4': 1, '5': 9, '10': 'description'},
  ],
};

/// Descriptor for `NewCategoryRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List newCategoryRequestDescriptor = $convert.base64Decode(
    'ChJOZXdDYXRlZ29yeVJlcXVlc3QSGwoJbWFuYWdlX2lkGAEgASgJUghtYW5hZ2VJZBISCgRjb2'
    'RlGAQgASgJUgRjb2RlEiIKBG5hbWUYAiABKAsyDi5jYXNobWVyZS5OYW1lUgRuYW1lEiAKC2Rl'
    'c2NyaXB0aW9uGAMgASgJUgtkZXNjcmlwdGlvbg==');

@$core.Deprecated('Use newCategoryResponseDescriptor instead')
const NewCategoryResponse$json = {
  '1': 'NewCategoryResponse',
  '2': [
    {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `NewCategoryResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List newCategoryResponseDescriptor = $convert.base64Decode(
    'ChNOZXdDYXRlZ29yeVJlc3BvbnNlEhYKBnJlc3VsdBgBIAEoCVIGcmVzdWx0');

@$core.Deprecated('Use getCategoriesRequestDescriptor instead')
const GetCategoriesRequest$json = {
  '1': 'GetCategoriesRequest',
  '2': [
    {'1': 'manage_id', '3': 1, '4': 1, '5': 9, '10': 'manageId'},
  ],
};

/// Descriptor for `GetCategoriesRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getCategoriesRequestDescriptor = $convert.base64Decode(
    'ChRHZXRDYXRlZ29yaWVzUmVxdWVzdBIbCgltYW5hZ2VfaWQYASABKAlSCG1hbmFnZUlk');

@$core.Deprecated('Use getCategoriesResponseDescriptor instead')
const GetCategoriesResponse$json = {
  '1': 'GetCategoriesResponse',
  '2': [
    {'1': 'codes', '3': 1, '4': 3, '5': 12, '10': 'codes'},
  ],
};

/// Descriptor for `GetCategoriesResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getCategoriesResponseDescriptor = $convert.base64Decode(
    'ChVHZXRDYXRlZ29yaWVzUmVzcG9uc2USFAoFY29kZXMYASADKAxSBWNvZGVz');

@$core.Deprecated('Use markEntityCategoriesRequestDescriptor instead')
const MarkEntityCategoriesRequest$json = {
  '1': 'MarkEntityCategoriesRequest',
  '2': [
    {'1': 'manage_id', '3': 2, '4': 1, '5': 9, '10': 'manageId'},
    {'1': 'entity_id', '3': 3, '4': 1, '5': 9, '10': 'entityId'},
    {'1': 'category_ids', '3': 4, '4': 3, '5': 9, '10': 'categoryIds'},
  ],
};

/// Descriptor for `MarkEntityCategoriesRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List markEntityCategoriesRequestDescriptor = $convert.base64Decode(
    'ChtNYXJrRW50aXR5Q2F0ZWdvcmllc1JlcXVlc3QSGwoJbWFuYWdlX2lkGAIgASgJUghtYW5hZ2'
    'VJZBIbCgllbnRpdHlfaWQYAyABKAlSCGVudGl0eUlkEiEKDGNhdGVnb3J5X2lkcxgEIAMoCVIL'
    'Y2F0ZWdvcnlJZHM=');

@$core.Deprecated('Use markEntityCategoriesResponseDescriptor instead')
const MarkEntityCategoriesResponse$json = {
  '1': 'MarkEntityCategoriesResponse',
  '2': [
    {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `MarkEntityCategoriesResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List markEntityCategoriesResponseDescriptor = $convert.base64Decode(
    'ChxNYXJrRW50aXR5Q2F0ZWdvcmllc1Jlc3BvbnNlEhYKBnJlc3VsdBgBIAEoCVIGcmVzdWx0');

@$core.Deprecated('Use unmarkEntityCategoriesRequestDescriptor instead')
const UnmarkEntityCategoriesRequest$json = {
  '1': 'UnmarkEntityCategoriesRequest',
  '2': [
    {'1': 'manage_id', '3': 2, '4': 1, '5': 9, '10': 'manageId'},
    {'1': 'entity_id', '3': 3, '4': 1, '5': 9, '10': 'entityId'},
    {'1': 'category_ids', '3': 4, '4': 3, '5': 9, '10': 'categoryIds'},
  ],
};

/// Descriptor for `UnmarkEntityCategoriesRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List unmarkEntityCategoriesRequestDescriptor = $convert.base64Decode(
    'Ch1Vbm1hcmtFbnRpdHlDYXRlZ29yaWVzUmVxdWVzdBIbCgltYW5hZ2VfaWQYAiABKAlSCG1hbm'
    'FnZUlkEhsKCWVudGl0eV9pZBgDIAEoCVIIZW50aXR5SWQSIQoMY2F0ZWdvcnlfaWRzGAQgAygJ'
    'UgtjYXRlZ29yeUlkcw==');

@$core.Deprecated('Use unmarkEntityCategoriesResponseDescriptor instead')
const UnmarkEntityCategoriesResponse$json = {
  '1': 'UnmarkEntityCategoriesResponse',
  '2': [
    {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `UnmarkEntityCategoriesResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List unmarkEntityCategoriesResponseDescriptor = $convert.base64Decode(
    'Ch5Vbm1hcmtFbnRpdHlDYXRlZ29yaWVzUmVzcG9uc2USFgoGcmVzdWx0GAEgASgJUgZyZXN1bH'
    'Q=');

