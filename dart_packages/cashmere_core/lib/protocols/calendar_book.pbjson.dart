//
//  Generated code. Do not modify.
//  source: calendar_book.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use newCalendarBookRequestDescriptor instead')
const NewCalendarBookRequest$json = {
  '1': 'NewCalendarBookRequest',
  '2': [
    {'1': 'manage_id', '3': 1, '4': 1, '5': 5, '10': 'manageId'},
    {'1': 'entity_id', '3': 2, '4': 1, '5': 9, '10': 'entityId'},
    {'1': 'name', '3': 3, '4': 1, '5': 11, '6': '.cashmere.Name', '10': 'name'},
    {'1': 'description', '3': 4, '4': 1, '5': 9, '10': 'description'},
  ],
};

/// Descriptor for `NewCalendarBookRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List newCalendarBookRequestDescriptor = $convert.base64Decode(
    'ChZOZXdDYWxlbmRhckJvb2tSZXF1ZXN0EhsKCW1hbmFnZV9pZBgBIAEoBVIIbWFuYWdlSWQSGw'
    'oJZW50aXR5X2lkGAIgASgJUghlbnRpdHlJZBIiCgRuYW1lGAMgASgLMg4uY2FzaG1lcmUuTmFt'
    'ZVIEbmFtZRIgCgtkZXNjcmlwdGlvbhgEIAEoCVILZGVzY3JpcHRpb24=');

@$core.Deprecated('Use newCalendarBookResponseDescriptor instead')
const NewCalendarBookResponse$json = {
  '1': 'NewCalendarBookResponse',
  '2': [
    {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `NewCalendarBookResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List newCalendarBookResponseDescriptor = $convert.base64Decode(
    'ChdOZXdDYWxlbmRhckJvb2tSZXNwb25zZRIWCgZyZXN1bHQYASABKAlSBnJlc3VsdA==');
