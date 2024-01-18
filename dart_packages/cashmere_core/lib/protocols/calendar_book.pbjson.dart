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
    {'1': 'manage_id', '3': 1, '4': 1, '5': 9, '10': 'manageId'},
    {'1': 'entity_id', '3': 2, '4': 1, '5': 9, '10': 'entityId'},
    {'1': 'name', '3': 3, '4': 1, '5': 11, '6': '.cashmere.Name', '10': 'name'},
    {'1': 'description', '3': 4, '4': 1, '5': 9, '10': 'description'},
    {'1': 'mark', '3': 5, '4': 1, '5': 9, '10': 'mark'},
  ],
};

/// Descriptor for `NewCalendarBookRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List newCalendarBookRequestDescriptor = $convert.base64Decode(
    'ChZOZXdDYWxlbmRhckJvb2tSZXF1ZXN0EhsKCW1hbmFnZV9pZBgBIAEoCVIIbWFuYWdlSWQSGw'
    'oJZW50aXR5X2lkGAIgASgJUghlbnRpdHlJZBIiCgRuYW1lGAMgASgLMg4uY2FzaG1lcmUuTmFt'
    'ZVIEbmFtZRIgCgtkZXNjcmlwdGlvbhgEIAEoCVILZGVzY3JpcHRpb24SEgoEbWFyaxgFIAEoCV'
    'IEbWFyaw==');

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

@$core.Deprecated('Use listCalendarBooksRequestDescriptor instead')
const ListCalendarBooksRequest$json = {
  '1': 'ListCalendarBooksRequest',
  '2': [
    {'1': 'manage_id', '3': 1, '4': 1, '5': 9, '10': 'manageId'},
    {'1': 'entity_id', '3': 2, '4': 1, '5': 9, '10': 'entityId'},
  ],
};

/// Descriptor for `ListCalendarBooksRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listCalendarBooksRequestDescriptor = $convert.base64Decode(
    'ChhMaXN0Q2FsZW5kYXJCb29rc1JlcXVlc3QSGwoJbWFuYWdlX2lkGAEgASgJUghtYW5hZ2VJZB'
    'IbCgllbnRpdHlfaWQYAiABKAlSCGVudGl0eUlk');

@$core.Deprecated('Use listCalendarBooksResponseDescriptor instead')
const ListCalendarBooksResponse$json = {
  '1': 'ListCalendarBooksResponse',
  '2': [
    {'1': 'books', '3': 1, '4': 3, '5': 12, '10': 'books'},
  ],
};

/// Descriptor for `ListCalendarBooksResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listCalendarBooksResponseDescriptor = $convert.base64Decode(
    'ChlMaXN0Q2FsZW5kYXJCb29rc1Jlc3BvbnNlEhQKBWJvb2tzGAEgAygMUgVib29rcw==');

@$core.Deprecated('Use listBookCalendarsRequestDescriptor instead')
const ListBookCalendarsRequest$json = {
  '1': 'ListBookCalendarsRequest',
  '2': [
    {'1': 'book_id', '3': 1, '4': 1, '5': 9, '10': 'bookId'},
  ],
};

/// Descriptor for `ListBookCalendarsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listBookCalendarsRequestDescriptor = $convert.base64Decode(
    'ChhMaXN0Qm9va0NhbGVuZGFyc1JlcXVlc3QSFwoHYm9va19pZBgBIAEoCVIGYm9va0lk');

@$core.Deprecated('Use listBookCalendarsResponseDescriptor instead')
const ListBookCalendarsResponse$json = {
  '1': 'ListBookCalendarsResponse',
  '2': [
    {'1': 'calendars', '3': 1, '4': 3, '5': 12, '10': 'calendars'},
  ],
};

/// Descriptor for `ListBookCalendarsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listBookCalendarsResponseDescriptor = $convert.base64Decode(
    'ChlMaXN0Qm9va0NhbGVuZGFyc1Jlc3BvbnNlEhwKCWNhbGVuZGFycxgBIAMoDFIJY2FsZW5kYX'
    'Jz');

