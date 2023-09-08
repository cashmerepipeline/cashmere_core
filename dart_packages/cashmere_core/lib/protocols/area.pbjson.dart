//
//  Generated code. Do not modify.
//  source: area.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use areaLevelDescriptor instead')
const AreaLevel$json = {
  '1': 'AreaLevel',
  '2': [
    {'1': 'Country', '2': 0},
    {'1': 'Province', '2': 1},
    {'1': 'City', '2': 2},
    {'1': 'Area', '2': 3},
  ],
};

/// Descriptor for `AreaLevel`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List areaLevelDescriptor = $convert.base64Decode(
    'CglBcmVhTGV2ZWwSCwoHQ291bnRyeRAAEgwKCFByb3ZpbmNlEAESCAoEQ2l0eRACEggKBEFyZW'
    'EQAw==');

@$core.Deprecated('Use newAreaRequestDescriptor instead')
const NewAreaRequest$json = {
  '1': 'NewAreaRequest',
  '2': [
    {'1': 'name', '3': 1, '4': 1, '5': 11, '6': '.cashmere.Name', '10': 'name'},
    {'1': 'parent_id', '3': 2, '4': 1, '5': 9, '10': 'parentId'},
    {'1': 'code', '3': 3, '4': 1, '5': 9, '10': 'code'},
    {'1': 'level', '3': 4, '4': 1, '5': 14, '6': '.cashmere.AreaLevel', '10': 'level'},
  ],
};

/// Descriptor for `NewAreaRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List newAreaRequestDescriptor = $convert.base64Decode(
    'Cg5OZXdBcmVhUmVxdWVzdBIiCgRuYW1lGAEgASgLMg4uY2FzaG1lcmUuTmFtZVIEbmFtZRIbCg'
    'lwYXJlbnRfaWQYAiABKAlSCHBhcmVudElkEhIKBGNvZGUYAyABKAlSBGNvZGUSKQoFbGV2ZWwY'
    'BCABKA4yEy5jYXNobWVyZS5BcmVhTGV2ZWxSBWxldmVs');

@$core.Deprecated('Use newAreaResponseDescriptor instead')
const NewAreaResponse$json = {
  '1': 'NewAreaResponse',
  '2': [
    {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `NewAreaResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List newAreaResponseDescriptor = $convert.base64Decode(
    'Cg9OZXdBcmVhUmVzcG9uc2USFgoGcmVzdWx0GAEgASgJUgZyZXN1bHQ=');

@$core.Deprecated('Use editAreaRequestDescriptor instead')
const EditAreaRequest$json = {
  '1': 'EditAreaRequest',
  '2': [
    {'1': 'area_id', '3': 1, '4': 1, '5': 9, '10': 'areaId'},
    {'1': 'new_parent_id', '3': 3, '4': 1, '5': 9, '10': 'newParentId'},
    {'1': 'new_level', '3': 4, '4': 1, '5': 14, '6': '.cashmere.AreaLevel', '10': 'newLevel'},
  ],
};

/// Descriptor for `EditAreaRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List editAreaRequestDescriptor = $convert.base64Decode(
    'Cg9FZGl0QXJlYVJlcXVlc3QSFwoHYXJlYV9pZBgBIAEoCVIGYXJlYUlkEiIKDW5ld19wYXJlbn'
    'RfaWQYAyABKAlSC25ld1BhcmVudElkEjAKCW5ld19sZXZlbBgEIAEoDjITLmNhc2htZXJlLkFy'
    'ZWFMZXZlbFIIbmV3TGV2ZWw=');

@$core.Deprecated('Use editAreaResponseDescriptor instead')
const EditAreaResponse$json = {
  '1': 'EditAreaResponse',
  '2': [
    {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `EditAreaResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List editAreaResponseDescriptor = $convert.base64Decode(
    'ChBFZGl0QXJlYVJlc3BvbnNlEhYKBnJlc3VsdBgBIAEoCVIGcmVzdWx0');

