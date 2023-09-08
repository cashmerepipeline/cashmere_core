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

@$core.Deprecated('Use phoneAreaCodeDescriptor instead')
const PhoneAreaCode$json = {
  '1': 'PhoneAreaCode',
  '2': [
    {'1': 'code', '3': 1, '4': 1, '5': 9, '10': 'code'},
    {'1': 'name_map', '3': 2, '4': 3, '5': 11, '6': '.cashmere.PhoneAreaCode.NameMapEntry', '10': 'nameMap'},
    {'1': 'using_areas', '3': 3, '4': 3, '5': 9, '10': 'usingAreas'},
  ],
  '3': [PhoneAreaCode_NameMapEntry$json],
};

@$core.Deprecated('Use phoneAreaCodeDescriptor instead')
const PhoneAreaCode_NameMapEntry$json = {
  '1': 'NameMapEntry',
  '2': [
    {'1': 'key', '3': 1, '4': 1, '5': 9, '10': 'key'},
    {'1': 'value', '3': 2, '4': 1, '5': 9, '10': 'value'},
  ],
  '7': {'7': true},
};

/// Descriptor for `PhoneAreaCode`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List phoneAreaCodeDescriptor = $convert.base64Decode(
    'Cg1QaG9uZUFyZWFDb2RlEhIKBGNvZGUYASABKAlSBGNvZGUSPwoIbmFtZV9tYXAYAiADKAsyJC'
    '5jYXNobWVyZS5QaG9uZUFyZWFDb2RlLk5hbWVNYXBFbnRyeVIHbmFtZU1hcBIfCgt1c2luZ19h'
    'cmVhcxgDIAMoCVIKdXNpbmdBcmVhcxo6CgxOYW1lTWFwRW50cnkSEAoDa2V5GAEgASgJUgNrZX'
    'kSFAoFdmFsdWUYAiABKAlSBXZhbHVlOgI4AQ==');

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
    {'1': 'phone_area_codes', '3': 1, '4': 3, '5': 11, '6': '.cashmere.PhoneAreaCode', '10': 'phoneAreaCodes'},
  ],
};

/// Descriptor for `GetPhoneAreaCodesResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getPhoneAreaCodesResponseDescriptor = $convert.base64Decode(
    'ChlHZXRQaG9uZUFyZWFDb2Rlc1Jlc3BvbnNlEkEKEHBob25lX2FyZWFfY29kZXMYASADKAsyFy'
    '5jYXNobWVyZS5QaG9uZUFyZWFDb2RlUg5waG9uZUFyZWFDb2Rlcw==');

