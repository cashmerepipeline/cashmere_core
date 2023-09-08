//
//  Generated code. Do not modify.
//  source: group.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use newGroupRequestDescriptor instead')
const NewGroupRequest$json = {
  '1': 'NewGroupRequest',
  '2': [
    {'1': 'name', '3': 1, '4': 1, '5': 11, '6': '.cashmere.Name', '10': 'name'},
    {'1': 'new_group_id', '3': 2, '4': 1, '5': 9, '10': 'newGroupId'},
  ],
};

/// Descriptor for `NewGroupRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List newGroupRequestDescriptor = $convert.base64Decode(
    'Cg9OZXdHcm91cFJlcXVlc3QSIgoEbmFtZRgBIAEoCzIOLmNhc2htZXJlLk5hbWVSBG5hbWUSIA'
    'oMbmV3X2dyb3VwX2lkGAIgASgJUgpuZXdHcm91cElk');

@$core.Deprecated('Use newGroupResponseDescriptor instead')
const NewGroupResponse$json = {
  '1': 'NewGroupResponse',
  '2': [
    {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `NewGroupResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List newGroupResponseDescriptor = $convert.base64Decode(
    'ChBOZXdHcm91cFJlc3BvbnNlEhYKBnJlc3VsdBgBIAEoCVIGcmVzdWx0');

