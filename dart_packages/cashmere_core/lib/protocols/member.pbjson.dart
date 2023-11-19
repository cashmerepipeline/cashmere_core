//
//  Generated code. Do not modify.
//  source: member.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use addMemberRequestDescriptor instead')
const AddMemberRequest$json = {
  '1': 'AddMemberRequest',
  '2': [
    {'1': 'name', '3': 1, '4': 1, '5': 11, '6': '.cashmere.Name', '10': 'name'},
    {'1': 'owner_manage_id', '3': 2, '4': 1, '5': 5, '10': 'ownerManageId'},
    {'1': 'owner_entity_id', '3': 3, '4': 1, '5': 9, '10': 'ownerEntityId'},
    {'1': 'self_manage_id', '3': 4, '4': 1, '5': 5, '10': 'selfManageId'},
    {'1': 'self_entity_id', '3': 5, '4': 1, '5': 9, '10': 'selfEntityId'},
    {'1': 'description', '3': 6, '4': 1, '5': 9, '10': 'description'},
  ],
};

/// Descriptor for `AddMemberRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List addMemberRequestDescriptor = $convert.base64Decode(
    'ChBBZGRNZW1iZXJSZXF1ZXN0EiIKBG5hbWUYASABKAsyDi5jYXNobWVyZS5OYW1lUgRuYW1lEi'
    'YKD293bmVyX21hbmFnZV9pZBgCIAEoBVINb3duZXJNYW5hZ2VJZBImCg9vd25lcl9lbnRpdHlf'
    'aWQYAyABKAlSDW93bmVyRW50aXR5SWQSJAoOc2VsZl9tYW5hZ2VfaWQYBCABKAVSDHNlbGZNYW'
    '5hZ2VJZBIkCg5zZWxmX2VudGl0eV9pZBgFIAEoCVIMc2VsZkVudGl0eUlkEiAKC2Rlc2NyaXB0'
    'aW9uGAYgASgJUgtkZXNjcmlwdGlvbg==');

@$core.Deprecated('Use addMemberResponseDescriptor instead')
const AddMemberResponse$json = {
  '1': 'AddMemberResponse',
  '2': [
    {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `AddMemberResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List addMemberResponseDescriptor = $convert.base64Decode(
    'ChFBZGRNZW1iZXJSZXNwb25zZRIWCgZyZXN1bHQYASABKAlSBnJlc3VsdA==');

