///
//  Generated code. Do not modify.
//  source: country.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields,deprecated_member_use_from_same_package

import 'dart:core' as $core;
import 'dart:convert' as $convert;
import 'dart:typed_data' as $typed_data;
@$core.Deprecated('Use newCountryRequestDescriptor instead')
const NewCountryRequest$json = const {
  '1': 'NewCountryRequest',
  '2': const [
    const {'1': 'name', '3': 1, '4': 1, '5': 11, '6': '.cashmere.Name', '10': 'name'},
    const {'1': 'native', '3': 2, '4': 1, '5': 9, '10': 'native'},
    const {'1': 'code', '3': 3, '4': 1, '5': 9, '10': 'code'},
    const {'1': 'phone_area_code', '3': 4, '4': 1, '5': 9, '10': 'phoneAreaCode'},
    const {'1': 'languages', '3': 5, '4': 3, '5': 9, '10': 'languages'},
  ],
};

/// Descriptor for `NewCountryRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List newCountryRequestDescriptor = $convert.base64Decode('ChFOZXdDb3VudHJ5UmVxdWVzdBIiCgRuYW1lGAEgASgLMg4uY2FzaG1lcmUuTmFtZVIEbmFtZRIWCgZuYXRpdmUYAiABKAlSBm5hdGl2ZRISCgRjb2RlGAMgASgJUgRjb2RlEiYKD3Bob25lX2FyZWFfY29kZRgEIAEoCVINcGhvbmVBcmVhQ29kZRIcCglsYW5ndWFnZXMYBSADKAlSCWxhbmd1YWdlcw==');
@$core.Deprecated('Use newCountryResponseDescriptor instead')
const NewCountryResponse$json = const {
  '1': 'NewCountryResponse',
  '2': const [
    const {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `NewCountryResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List newCountryResponseDescriptor = $convert.base64Decode('ChJOZXdDb3VudHJ5UmVzcG9uc2USFgoGcmVzdWx0GAEgASgJUgZyZXN1bHQ=');
