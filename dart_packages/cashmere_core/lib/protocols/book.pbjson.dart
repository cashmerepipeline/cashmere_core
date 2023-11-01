//
//  Generated code. Do not modify.
//  source: book.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use newBookRequestDescriptor instead')
const NewBookRequest$json = {
  '1': 'NewBookRequest',
  '2': [
    {'1': 'manage_id', '3': 1, '4': 1, '5': 5, '10': 'manageId'},
    {'1': 'entity_id', '3': 2, '4': 1, '5': 9, '10': 'entityId'},
    {'1': 'name', '3': 3, '4': 1, '5': 11, '6': '.cashmere.Name', '10': 'name'},
    {'1': 'description', '3': 4, '4': 1, '5': 9, '10': 'description'},
  ],
};

/// Descriptor for `NewBookRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List newBookRequestDescriptor = $convert.base64Decode(
    'Cg5OZXdCb29rUmVxdWVzdBIbCgltYW5hZ2VfaWQYASABKAVSCG1hbmFnZUlkEhsKCWVudGl0eV'
    '9pZBgCIAEoCVIIZW50aXR5SWQSIgoEbmFtZRgDIAEoCzIOLmNhc2htZXJlLk5hbWVSBG5hbWUS'
    'IAoLZGVzY3JpcHRpb24YBCABKAlSC2Rlc2NyaXB0aW9u');

@$core.Deprecated('Use newBookResponseDescriptor instead')
const NewBookResponse$json = {
  '1': 'NewBookResponse',
  '2': [
    {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `NewBookResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List newBookResponseDescriptor = $convert.base64Decode(
    'Cg9OZXdCb29rUmVzcG9uc2USFgoGcmVzdWx0GAEgASgJUgZyZXN1bHQ=');

@$core.Deprecated('Use addCalendarRequestDescriptor instead')
const AddCalendarRequest$json = {
  '1': 'AddCalendarRequest',
  '2': [
    {'1': 'book_id', '3': 1, '4': 1, '5': 9, '10': 'bookId'},
    {'1': 'calendar', '3': 2, '4': 1, '5': 11, '6': '.cashmere.Calendar', '10': 'calendar'},
    {'1': 'description', '3': 3, '4': 1, '5': 9, '10': 'description'},
  ],
};

/// Descriptor for `AddCalendarRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List addCalendarRequestDescriptor = $convert.base64Decode(
    'ChJBZGRDYWxlbmRhclJlcXVlc3QSFwoHYm9va19pZBgBIAEoCVIGYm9va0lkEi4KCGNhbGVuZG'
    'FyGAIgASgLMhIuY2FzaG1lcmUuQ2FsZW5kYXJSCGNhbGVuZGFyEiAKC2Rlc2NyaXB0aW9uGAMg'
    'ASgJUgtkZXNjcmlwdGlvbg==');

@$core.Deprecated('Use addCalendarResponseDescriptor instead')
const AddCalendarResponse$json = {
  '1': 'AddCalendarResponse',
  '2': [
    {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `AddCalendarResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List addCalendarResponseDescriptor = $convert.base64Decode(
    'ChNBZGRDYWxlbmRhclJlc3BvbnNlEhYKBnJlc3VsdBgBIAEoCVIGcmVzdWx0');

