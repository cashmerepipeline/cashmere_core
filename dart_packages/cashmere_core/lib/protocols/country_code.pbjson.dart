//
//  Generated code. Do not modify.
//  source: country_code.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use countryCodeDescriptor instead')
const CountryCode$json = {
  '1': 'CountryCode',
  '2': [
    {'1': 'code', '3': 1, '4': 1, '5': 9, '10': 'code'},
    {'1': 'name_map', '3': 2, '4': 3, '5': 11, '6': '.cashmere.CountryCode.NameMapEntry', '10': 'nameMap'},
    {'1': 'native', '3': 3, '4': 1, '5': 9, '10': 'native'},
    {'1': 'phone_area_code', '3': 4, '4': 1, '5': 9, '10': 'phoneAreaCode'},
    {'1': 'languages', '3': 5, '4': 3, '5': 9, '10': 'languages'},
  ],
  '3': [CountryCode_NameMapEntry$json],
};

@$core.Deprecated('Use countryCodeDescriptor instead')
const CountryCode_NameMapEntry$json = {
  '1': 'NameMapEntry',
  '2': [
    {'1': 'key', '3': 1, '4': 1, '5': 9, '10': 'key'},
    {'1': 'value', '3': 2, '4': 1, '5': 9, '10': 'value'},
  ],
  '7': {'7': true},
};

/// Descriptor for `CountryCode`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List countryCodeDescriptor = $convert.base64Decode(
    'CgtDb3VudHJ5Q29kZRISCgRjb2RlGAEgASgJUgRjb2RlEj0KCG5hbWVfbWFwGAIgAygLMiIuY2'
    'FzaG1lcmUuQ291bnRyeUNvZGUuTmFtZU1hcEVudHJ5UgduYW1lTWFwEhYKBm5hdGl2ZRgDIAEo'
    'CVIGbmF0aXZlEiYKD3Bob25lX2FyZWFfY29kZRgEIAEoCVINcGhvbmVBcmVhQ29kZRIcCglsYW'
    '5ndWFnZXMYBSADKAlSCWxhbmd1YWdlcxo6CgxOYW1lTWFwRW50cnkSEAoDa2V5GAEgASgJUgNr'
    'ZXkSFAoFdmFsdWUYAiABKAlSBXZhbHVlOgI4AQ==');

@$core.Deprecated('Use newCountryCodeRequestDescriptor instead')
const NewCountryCodeRequest$json = {
  '1': 'NewCountryCodeRequest',
  '2': [
    {'1': 'name', '3': 1, '4': 1, '5': 11, '6': '.cashmere.Name', '10': 'name'},
    {'1': 'code', '3': 3, '4': 1, '5': 9, '10': 'code'},
    {'1': 'native', '3': 2, '4': 1, '5': 9, '10': 'native'},
    {'1': 'phone_area_code', '3': 4, '4': 1, '5': 9, '10': 'phoneAreaCode'},
    {'1': 'languages', '3': 5, '4': 3, '5': 9, '10': 'languages'},
  ],
};

/// Descriptor for `NewCountryCodeRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List newCountryCodeRequestDescriptor = $convert.base64Decode(
    'ChVOZXdDb3VudHJ5Q29kZVJlcXVlc3QSIgoEbmFtZRgBIAEoCzIOLmNhc2htZXJlLk5hbWVSBG'
    '5hbWUSEgoEY29kZRgDIAEoCVIEY29kZRIWCgZuYXRpdmUYAiABKAlSBm5hdGl2ZRImCg9waG9u'
    'ZV9hcmVhX2NvZGUYBCABKAlSDXBob25lQXJlYUNvZGUSHAoJbGFuZ3VhZ2VzGAUgAygJUglsYW'
    '5ndWFnZXM=');

@$core.Deprecated('Use newCountryCodeResponseDescriptor instead')
const NewCountryCodeResponse$json = {
  '1': 'NewCountryCodeResponse',
  '2': [
    {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `NewCountryCodeResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List newCountryCodeResponseDescriptor = $convert.base64Decode(
    'ChZOZXdDb3VudHJ5Q29kZVJlc3BvbnNlEhYKBnJlc3VsdBgBIAEoCVIGcmVzdWx0');

@$core.Deprecated('Use getCountryCodesRequestDescriptor instead')
const GetCountryCodesRequest$json = {
  '1': 'GetCountryCodesRequest',
};

/// Descriptor for `GetCountryCodesRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getCountryCodesRequestDescriptor = $convert.base64Decode(
    'ChZHZXRDb3VudHJ5Q29kZXNSZXF1ZXN0');

@$core.Deprecated('Use getCountryCodesResponseDescriptor instead')
const GetCountryCodesResponse$json = {
  '1': 'GetCountryCodesResponse',
  '2': [
    {'1': 'country_codes', '3': 1, '4': 3, '5': 11, '6': '.cashmere.CountryCode', '10': 'countryCodes'},
  ],
};

/// Descriptor for `GetCountryCodesResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getCountryCodesResponseDescriptor = $convert.base64Decode(
    'ChdHZXRDb3VudHJ5Q29kZXNSZXNwb25zZRI6Cg1jb3VudHJ5X2NvZGVzGAEgAygLMhUuY2FzaG'
    '1lcmUuQ291bnRyeUNvZGVSDGNvdW50cnlDb2Rlcw==');

