//
//  Generated code. Do not modify.
//  source: calendar.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use calendarTypeDescriptor instead')
const CalendarType$json = {
  '1': 'CalendarType',
  '2': [
    {'1': 'Specified', '2': 0},
    {'1': 'Scripted', '2': 1},
  ],
};

/// Descriptor for `CalendarType`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List calendarTypeDescriptor = $convert.base64Decode(
    'CgxDYWxlbmRhclR5cGUSDQoJU3BlY2lmaWVkEAASDAoIU2NyaXB0ZWQQAQ==');

@$core.Deprecated('Use calendarDescriptor instead')
const Calendar$json = {
  '1': 'Calendar',
  '2': [
    {'1': 'type', '3': 1, '4': 1, '5': 14, '6': '.cashmere.CalendarType', '10': 'type'},
    {'1': 'every', '3': 2, '4': 1, '5': 9, '10': 'every'},
    {'1': 'daytime', '3': 3, '4': 3, '5': 11, '6': '.cashmere.Calendar.DaytimeEntry', '10': 'daytime'},
  ],
  '3': [Calendar_DaytimeEntry$json],
};

@$core.Deprecated('Use calendarDescriptor instead')
const Calendar_DaytimeEntry$json = {
  '1': 'DaytimeEntry',
  '2': [
    {'1': 'key', '3': 1, '4': 1, '5': 9, '10': 'key'},
    {'1': 'value', '3': 2, '4': 1, '5': 9, '10': 'value'},
  ],
  '7': {'7': true},
};

/// Descriptor for `Calendar`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List calendarDescriptor = $convert.base64Decode(
    'CghDYWxlbmRhchIqCgR0eXBlGAEgASgOMhYuY2FzaG1lcmUuQ2FsZW5kYXJUeXBlUgR0eXBlEh'
    'QKBWV2ZXJ5GAIgASgJUgVldmVyeRI5CgdkYXl0aW1lGAMgAygLMh8uY2FzaG1lcmUuQ2FsZW5k'
    'YXIuRGF5dGltZUVudHJ5UgdkYXl0aW1lGjoKDERheXRpbWVFbnRyeRIQCgNrZXkYASABKAlSA2'
    'tleRIUCgV2YWx1ZRgCIAEoCVIFdmFsdWU6AjgB');

@$core.Deprecated('Use newCalendarRequestDescriptor instead')
const NewCalendarRequest$json = {
  '1': 'NewCalendarRequest',
  '2': [
    {'1': 'manage_id', '3': 1, '4': 1, '5': 5, '10': 'manageId'},
    {'1': 'entity_id', '3': 2, '4': 1, '5': 9, '10': 'entityId'},
    {'1': 'calendar', '3': 3, '4': 1, '5': 11, '6': '.cashmere.Calendar', '10': 'calendar'},
    {'1': 'description', '3': 4, '4': 1, '5': 9, '10': 'description'},
  ],
};

/// Descriptor for `NewCalendarRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List newCalendarRequestDescriptor = $convert.base64Decode(
    'ChJOZXdDYWxlbmRhclJlcXVlc3QSGwoJbWFuYWdlX2lkGAEgASgFUghtYW5hZ2VJZBIbCgllbn'
    'RpdHlfaWQYAiABKAlSCGVudGl0eUlkEi4KCGNhbGVuZGFyGAMgASgLMhIuY2FzaG1lcmUuQ2Fs'
    'ZW5kYXJSCGNhbGVuZGFyEiAKC2Rlc2NyaXB0aW9uGAQgASgJUgtkZXNjcmlwdGlvbg==');

@$core.Deprecated('Use newCalendarResponseDescriptor instead')
const NewCalendarResponse$json = {
  '1': 'NewCalendarResponse',
  '2': [
    {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `NewCalendarResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List newCalendarResponseDescriptor = $convert.base64Decode(
    'ChNOZXdDYWxlbmRhclJlc3BvbnNlEhYKBnJlc3VsdBgBIAEoCVIGcmVzdWx0');

@$core.Deprecated('Use getCalendarsRequestDescriptor instead')
const GetCalendarsRequest$json = {
  '1': 'GetCalendarsRequest',
  '2': [
    {'1': 'manage_id', '3': 1, '4': 1, '5': 5, '10': 'manageId'},
    {'1': 'entity_id', '3': 2, '4': 1, '5': 9, '10': 'entityId'},
  ],
};

/// Descriptor for `GetCalendarsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getCalendarsRequestDescriptor = $convert.base64Decode(
    'ChNHZXRDYWxlbmRhcnNSZXF1ZXN0EhsKCW1hbmFnZV9pZBgBIAEoBVIIbWFuYWdlSWQSGwoJZW'
    '50aXR5X2lkGAIgASgJUghlbnRpdHlJZA==');

@$core.Deprecated('Use getCalendarsResponseDescriptor instead')
const GetCalendarsResponse$json = {
  '1': 'GetCalendarsResponse',
  '2': [
    {'1': 'calendars', '3': 1, '4': 3, '5': 12, '10': 'calendars'},
  ],
};

/// Descriptor for `GetCalendarsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getCalendarsResponseDescriptor = $convert.base64Decode(
    'ChRHZXRDYWxlbmRhcnNSZXNwb25zZRIcCgljYWxlbmRhcnMYASADKAxSCWNhbGVuZGFycw==');

