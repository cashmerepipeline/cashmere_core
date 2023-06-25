///
//  Generated code. Do not modify.
//  source: country_code.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,deprecated_member_use_from_same_package,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

import 'dart:core' as $core;
import 'dart:convert' as $convert;
import 'dart:typed_data' as $typed_data;
@$core.Deprecated('Use countryCodeDescriptor instead')
const CountryCode$json = const {
  '1': 'CountryCode',
  '2': const [
    const {'1': 'code', '3': 1, '4': 1, '5': 9, '10': 'code'},
    const {'1': 'name_map', '3': 2, '4': 3, '5': 11, '6': '.cashmere.CountryCode.NameMapEntry', '10': 'nameMap'},
    const {'1': 'native', '3': 3, '4': 1, '5': 9, '10': 'native'},
    const {'1': 'phone_area_code', '3': 4, '4': 1, '5': 9, '10': 'phoneAreaCode'},
    const {'1': 'languages', '3': 5, '4': 3, '5': 9, '10': 'languages'},
  ],
  '3': const [CountryCode_NameMapEntry$json],
};

@$core.Deprecated('Use countryCodeDescriptor instead')
const CountryCode_NameMapEntry$json = const {
  '1': 'NameMapEntry',
  '2': const [
    const {'1': 'key', '3': 1, '4': 1, '5': 9, '10': 'key'},
    const {'1': 'value', '3': 2, '4': 1, '5': 9, '10': 'value'},
  ],
  '7': const {'7': true},
};

/// Descriptor for `CountryCode`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List countryCodeDescriptor = $convert.base64Decode('CgtDb3VudHJ5Q29kZRISCgRjb2RlGAEgASgJUgRjb2RlEj0KCG5hbWVfbWFwGAIgAygLMiIuY2FzaG1lcmUuQ291bnRyeUNvZGUuTmFtZU1hcEVudHJ5UgduYW1lTWFwEhYKBm5hdGl2ZRgDIAEoCVIGbmF0aXZlEiYKD3Bob25lX2FyZWFfY29kZRgEIAEoCVINcGhvbmVBcmVhQ29kZRIcCglsYW5ndWFnZXMYBSADKAlSCWxhbmd1YWdlcxo6CgxOYW1lTWFwRW50cnkSEAoDa2V5GAEgASgJUgNrZXkSFAoFdmFsdWUYAiABKAlSBXZhbHVlOgI4AQ==');
@$core.Deprecated('Use newCountryCodeRequestDescriptor instead')
const NewCountryCodeRequest$json = const {
  '1': 'NewCountryCodeRequest',
  '2': const [
    const {'1': 'name', '3': 1, '4': 1, '5': 11, '6': '.cashmere.Name', '10': 'name'},
    const {'1': 'code', '3': 3, '4': 1, '5': 9, '10': 'code'},
    const {'1': 'native', '3': 2, '4': 1, '5': 9, '10': 'native'},
    const {'1': 'phone_area_code', '3': 4, '4': 1, '5': 9, '10': 'phoneAreaCode'},
    const {'1': 'languages', '3': 5, '4': 3, '5': 9, '10': 'languages'},
  ],
};

/// Descriptor for `NewCountryCodeRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List newCountryCodeRequestDescriptor = $convert.base64Decode('ChVOZXdDb3VudHJ5Q29kZVJlcXVlc3QSIgoEbmFtZRgBIAEoCzIOLmNhc2htZXJlLk5hbWVSBG5hbWUSEgoEY29kZRgDIAEoCVIEY29kZRIWCgZuYXRpdmUYAiABKAlSBm5hdGl2ZRImCg9waG9uZV9hcmVhX2NvZGUYBCABKAlSDXBob25lQXJlYUNvZGUSHAoJbGFuZ3VhZ2VzGAUgAygJUglsYW5ndWFnZXM=');
@$core.Deprecated('Use newCountryCodeResponseDescriptor instead')
const NewCountryCodeResponse$json = const {
  '1': 'NewCountryCodeResponse',
  '2': const [
    const {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `NewCountryCodeResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List newCountryCodeResponseDescriptor = $convert.base64Decode('ChZOZXdDb3VudHJ5Q29kZVJlc3BvbnNlEhYKBnJlc3VsdBgBIAEoCVIGcmVzdWx0');
