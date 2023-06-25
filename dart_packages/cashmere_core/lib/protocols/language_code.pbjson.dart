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
    const {'1': 'name_map', '3': 2, '4': 3, '5': 11, '6': '.cashmere.LanguageCode.NameMapEntry', '10': 'nameMap'},
    const {'1': 'native_name', '3': 3, '4': 1, '5': 9, '10': 'nativeName'},
  ],
  '3': const [LanguageCode_NameMapEntry$json],
};

@$core.Deprecated('Use languageCodeDescriptor instead')
const LanguageCode_NameMapEntry$json = const {
  '1': 'NameMapEntry',
  '2': const [
    const {'1': 'key', '3': 1, '4': 1, '5': 9, '10': 'key'},
    const {'1': 'value', '3': 2, '4': 1, '5': 9, '10': 'value'},
  ],
  '7': const {'7': true},
};

/// Descriptor for `LanguageCode`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List languageCodeDescriptor = $convert.base64Decode('CgxMYW5ndWFnZUNvZGUSEgoEY29kZRgBIAEoCVIEY29kZRI+CghuYW1lX21hcBgCIAMoCzIjLmNhc2htZXJlLkxhbmd1YWdlQ29kZS5OYW1lTWFwRW50cnlSB25hbWVNYXASHwoLbmF0aXZlX25hbWUYAyABKAlSCm5hdGl2ZU5hbWUaOgoMTmFtZU1hcEVudHJ5EhAKA2tleRgBIAEoCVIDa2V5EhQKBXZhbHVlGAIgASgJUgV2YWx1ZToCOAE=');
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
@$core.Deprecated('Use updateLanguageCodeRequestDescriptor instead')
const UpdateLanguageCodeRequest$json = const {
  '1': 'UpdateLanguageCodeRequest',
  '2': const [
    const {'1': 'id', '3': 1, '4': 1, '5': 9, '10': 'id'},
    const {'1': 'new_code', '3': 2, '4': 1, '5': 9, '10': 'newCode'},
    const {'1': 'new_native', '3': 3, '4': 1, '5': 9, '10': 'newNative'},
  ],
};

/// Descriptor for `UpdateLanguageCodeRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List updateLanguageCodeRequestDescriptor = $convert.base64Decode('ChlVcGRhdGVMYW5ndWFnZUNvZGVSZXF1ZXN0Eg4KAmlkGAEgASgJUgJpZBIZCghuZXdfY29kZRgCIAEoCVIHbmV3Q29kZRIdCgpuZXdfbmF0aXZlGAMgASgJUgluZXdOYXRpdmU=');
@$core.Deprecated('Use updateLanguageCodeResponseDescriptor instead')
const UpdateLanguageCodeResponse$json = const {
  '1': 'UpdateLanguageCodeResponse',
  '2': const [
    const {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `UpdateLanguageCodeResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List updateLanguageCodeResponseDescriptor = $convert.base64Decode('ChpVcGRhdGVMYW5ndWFnZUNvZGVSZXNwb25zZRIWCgZyZXN1bHQYASABKAlSBnJlc3VsdA==');
