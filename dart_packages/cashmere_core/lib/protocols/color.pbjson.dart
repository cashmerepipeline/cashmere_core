//
//  Generated code. Do not modify.
//  source: color.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use newColorRequestDescriptor instead')
const NewColorRequest$json = {
  '1': 'NewColorRequest',
  '2': [
    {'1': 'name', '3': 1, '4': 1, '5': 11, '6': '.cashmere.Name', '10': 'name'},
    {'1': 'color', '3': 2, '4': 3, '5': 13, '10': 'color'},
    {'1': 'description', '3': 3, '4': 1, '5': 9, '10': 'description'},
  ],
};

/// Descriptor for `NewColorRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List newColorRequestDescriptor = $convert.base64Decode(
    'Cg9OZXdDb2xvclJlcXVlc3QSIgoEbmFtZRgBIAEoCzIOLmNhc2htZXJlLk5hbWVSBG5hbWUSFA'
    'oFY29sb3IYAiADKA1SBWNvbG9yEiAKC2Rlc2NyaXB0aW9uGAMgASgJUgtkZXNjcmlwdGlvbg==');

@$core.Deprecated('Use newColorResponseDescriptor instead')
const NewColorResponse$json = {
  '1': 'NewColorResponse',
  '2': [
    {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `NewColorResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List newColorResponseDescriptor = $convert.base64Decode(
    'ChBOZXdDb2xvclJlc3BvbnNlEhYKBnJlc3VsdBgBIAEoCVIGcmVzdWx0');

@$core.Deprecated('Use getColorsRequestDescriptor instead')
const GetColorsRequest$json = {
  '1': 'GetColorsRequest',
};

/// Descriptor for `GetColorsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getColorsRequestDescriptor = $convert.base64Decode(
    'ChBHZXRDb2xvcnNSZXF1ZXN0');

@$core.Deprecated('Use getColorsResponseDescriptor instead')
const GetColorsResponse$json = {
  '1': 'GetColorsResponse',
  '2': [
    {'1': 'colors', '3': 1, '4': 3, '5': 12, '10': 'colors'},
  ],
};

/// Descriptor for `GetColorsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getColorsResponseDescriptor = $convert.base64Decode(
    'ChFHZXRDb2xvcnNSZXNwb25zZRIWCgZjb2xvcnMYASADKAxSBmNvbG9ycw==');

