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
    {'1': 'country_codes', '3': 1, '4': 3, '5': 12, '10': 'countryCodes'},
  ],
};

/// Descriptor for `GetCountryCodesResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getCountryCodesResponseDescriptor = $convert.base64Decode(
    'ChdHZXRDb3VudHJ5Q29kZXNSZXNwb25zZRIjCg1jb3VudHJ5X2NvZGVzGAEgAygMUgxjb3VudH'
    'J5Q29kZXM=');

