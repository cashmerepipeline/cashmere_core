//
//  Generated code. Do not modify.
//  source: language_code.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use newLanguageCodeRequestDescriptor instead')
const NewLanguageCodeRequest$json = {
  '1': 'NewLanguageCodeRequest',
  '2': [
    {'1': 'name', '3': 1, '4': 1, '5': 11, '6': '.cashmere.Name', '10': 'name'},
    {'1': 'code', '3': 2, '4': 1, '5': 9, '10': 'code'},
    {'1': 'native_name', '3': 3, '4': 1, '5': 9, '10': 'nativeName'},
  ],
};

/// Descriptor for `NewLanguageCodeRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List newLanguageCodeRequestDescriptor = $convert.base64Decode(
    'ChZOZXdMYW5ndWFnZUNvZGVSZXF1ZXN0EiIKBG5hbWUYASABKAsyDi5jYXNobWVyZS5OYW1lUg'
    'RuYW1lEhIKBGNvZGUYAiABKAlSBGNvZGUSHwoLbmF0aXZlX25hbWUYAyABKAlSCm5hdGl2ZU5h'
    'bWU=');

@$core.Deprecated('Use newLanguageCodeResponseDescriptor instead')
const NewLanguageCodeResponse$json = {
  '1': 'NewLanguageCodeResponse',
  '2': [
    {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `NewLanguageCodeResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List newLanguageCodeResponseDescriptor = $convert.base64Decode(
    'ChdOZXdMYW5ndWFnZUNvZGVSZXNwb25zZRIWCgZyZXN1bHQYASABKAlSBnJlc3VsdA==');

@$core.Deprecated('Use getLanguageCodesRequestDescriptor instead')
const GetLanguageCodesRequest$json = {
  '1': 'GetLanguageCodesRequest',
};

/// Descriptor for `GetLanguageCodesRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getLanguageCodesRequestDescriptor = $convert.base64Decode(
    'ChdHZXRMYW5ndWFnZUNvZGVzUmVxdWVzdA==');

@$core.Deprecated('Use getLanguageCodesResponseDescriptor instead')
const GetLanguageCodesResponse$json = {
  '1': 'GetLanguageCodesResponse',
  '2': [
    {'1': 'language_codes', '3': 1, '4': 3, '5': 12, '10': 'languageCodes'},
  ],
};

/// Descriptor for `GetLanguageCodesResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getLanguageCodesResponseDescriptor = $convert.base64Decode(
    'ChhHZXRMYW5ndWFnZUNvZGVzUmVzcG9uc2USJQoObGFuZ3VhZ2VfY29kZXMYASADKAxSDWxhbm'
    'd1YWdlQ29kZXM=');

@$core.Deprecated('Use updateLanguageCodeRequestDescriptor instead')
const UpdateLanguageCodeRequest$json = {
  '1': 'UpdateLanguageCodeRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 9, '10': 'id'},
    {'1': 'new_code', '3': 2, '4': 1, '5': 9, '10': 'newCode'},
    {'1': 'new_native', '3': 3, '4': 1, '5': 9, '10': 'newNative'},
  ],
};

/// Descriptor for `UpdateLanguageCodeRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List updateLanguageCodeRequestDescriptor = $convert.base64Decode(
    'ChlVcGRhdGVMYW5ndWFnZUNvZGVSZXF1ZXN0Eg4KAmlkGAEgASgJUgJpZBIZCghuZXdfY29kZR'
    'gCIAEoCVIHbmV3Q29kZRIdCgpuZXdfbmF0aXZlGAMgASgJUgluZXdOYXRpdmU=');

@$core.Deprecated('Use updateLanguageCodeResponseDescriptor instead')
const UpdateLanguageCodeResponse$json = {
  '1': 'UpdateLanguageCodeResponse',
  '2': [
    {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `UpdateLanguageCodeResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List updateLanguageCodeResponseDescriptor = $convert.base64Decode(
    'ChpVcGRhdGVMYW5ndWFnZUNvZGVSZXNwb25zZRIWCgZyZXN1bHQYASABKAlSBnJlc3VsdA==');

