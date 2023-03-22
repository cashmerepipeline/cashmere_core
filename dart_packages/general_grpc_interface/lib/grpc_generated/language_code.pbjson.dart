///
//  Generated code. Do not modify.
//  source: language_code.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,deprecated_member_use_from_same_package,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

import 'dart:core' as $core;
import 'dart:convert' as $convert;
import 'dart:typed_data' as $typed_data;
@$core.Deprecated('Use languageCodeDescriptor instead')
const LanguageCode$json = const {
  '1': 'LanguageCode',
  '2': const [
    const {'1': 'code', '3': 1, '4': 1, '5': 9, '10': 'code'},
    const {'1': 'name_map', '3': 2, '4': 1, '5': 12, '10': 'nameMap'},
  ],
};

/// Descriptor for `LanguageCode`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List languageCodeDescriptor = $convert.base64Decode('CgxMYW5ndWFnZUNvZGUSEgoEY29kZRgBIAEoCVIEY29kZRIZCghuYW1lX21hcBgCIAEoDFIHbmFtZU1hcA==');
@$core.Deprecated('Use newLanguageCodeRequestDescriptor instead')
const NewLanguageCodeRequest$json = const {
  '1': 'NewLanguageCodeRequest',
  '2': const [
    const {'1': 'name', '3': 1, '4': 1, '5': 11, '6': '.cashmere.Name', '10': 'name'},
    const {'1': 'code', '3': 2, '4': 1, '5': 9, '10': 'code'},
    const {'1': 'native_name', '3': 3, '4': 1, '5': 9, '10': 'nativeName'},
  ],
};

/// Descriptor for `NewLanguageCodeRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List newLanguageCodeRequestDescriptor = $convert.base64Decode('ChZOZXdMYW5ndWFnZUNvZGVSZXF1ZXN0EiIKBG5hbWUYASABKAsyDi5jYXNobWVyZS5OYW1lUgRuYW1lEhIKBGNvZGUYAiABKAlSBGNvZGUSHwoLbmF0aXZlX25hbWUYAyABKAlSCm5hdGl2ZU5hbWU=');
@$core.Deprecated('Use newLanguageCodeResponseDescriptor instead')
const NewLanguageCodeResponse$json = const {
  '1': 'NewLanguageCodeResponse',
  '2': const [
    const {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `NewLanguageCodeResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List newLanguageCodeResponseDescriptor = $convert.base64Decode('ChdOZXdMYW5ndWFnZUNvZGVSZXNwb25zZRIWCgZyZXN1bHQYASABKAlSBnJlc3VsdA==');
@$core.Deprecated('Use editLanguageCodeRequestDescriptor instead')
const EditLanguageCodeRequest$json = const {
  '1': 'EditLanguageCodeRequest',
  '2': const [
    const {'1': 'id', '3': 1, '4': 1, '5': 9, '10': 'id'},
    const {'1': 'new_code', '3': 2, '4': 1, '5': 9, '10': 'newCode'},
    const {'1': 'new_native', '3': 3, '4': 1, '5': 9, '10': 'newNative'},
  ],
};

/// Descriptor for `EditLanguageCodeRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List editLanguageCodeRequestDescriptor = $convert.base64Decode('ChdFZGl0TGFuZ3VhZ2VDb2RlUmVxdWVzdBIOCgJpZBgBIAEoCVICaWQSGQoIbmV3X2NvZGUYAiABKAlSB25ld0NvZGUSHQoKbmV3X25hdGl2ZRgDIAEoCVIJbmV3TmF0aXZl');
@$core.Deprecated('Use editLanguageCodeResponseDescriptor instead')
const EditLanguageCodeResponse$json = const {
  '1': 'EditLanguageCodeResponse',
  '2': const [
    const {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `EditLanguageCodeResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List editLanguageCodeResponseDescriptor = $convert.base64Decode('ChhFZGl0TGFuZ3VhZ2VDb2RlUmVzcG9uc2USFgoGcmVzdWx0GAEgASgJUgZyZXN1bHQ=');
