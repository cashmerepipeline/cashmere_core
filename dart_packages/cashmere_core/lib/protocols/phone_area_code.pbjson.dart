//
//  Generated code. Do not modify.
//  source: phone_area_code.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use newPhoneAreaCodeRequestDescriptor instead')
const NewPhoneAreaCodeRequest$json = {
  '1': 'NewPhoneAreaCodeRequest',
  '2': [
    {'1': 'name', '3': 1, '4': 1, '5': 11, '6': '.cashmere.Name', '10': 'name'},
    {'1': 'code', '3': 2, '4': 1, '5': 9, '10': 'code'},
    {'1': 'areas', '3': 3, '4': 3, '5': 9, '10': 'areas'},
  ],
};

/// Descriptor for `NewPhoneAreaCodeRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List newPhoneAreaCodeRequestDescriptor = $convert.base64Decode(
    'ChdOZXdQaG9uZUFyZWFDb2RlUmVxdWVzdBIiCgRuYW1lGAEgASgLMg4uY2FzaG1lcmUuTmFtZV'
    'IEbmFtZRISCgRjb2RlGAIgASgJUgRjb2RlEhQKBWFyZWFzGAMgAygJUgVhcmVhcw==');

@$core.Deprecated('Use newPhoneAreaCodeResponseDescriptor instead')
const NewPhoneAreaCodeResponse$json = {
  '1': 'NewPhoneAreaCodeResponse',
  '2': [
    {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `NewPhoneAreaCodeResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List newPhoneAreaCodeResponseDescriptor = $convert.base64Decode(
    'ChhOZXdQaG9uZUFyZWFDb2RlUmVzcG9uc2USFgoGcmVzdWx0GAEgASgJUgZyZXN1bHQ=');

@$core.Deprecated('Use getPhoneAreaCodesRequestDescriptor instead')
const GetPhoneAreaCodesRequest$json = {
  '1': 'GetPhoneAreaCodesRequest',
};

/// Descriptor for `GetPhoneAreaCodesRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getPhoneAreaCodesRequestDescriptor = $convert.base64Decode(
    'ChhHZXRQaG9uZUFyZWFDb2Rlc1JlcXVlc3Q=');

@$core.Deprecated('Use getPhoneAreaCodesResponseDescriptor instead')
const GetPhoneAreaCodesResponse$json = {
  '1': 'GetPhoneAreaCodesResponse',
  '2': [
    {'1': 'codes', '3': 1, '4': 3, '5': 12, '10': 'codes'},
  ],
};

/// Descriptor for `GetPhoneAreaCodesResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getPhoneAreaCodesResponseDescriptor = $convert.base64Decode(
    'ChlHZXRQaG9uZUFyZWFDb2Rlc1Jlc3BvbnNlEhQKBWNvZGVzGAEgAygMUgVjb2Rlcw==');

