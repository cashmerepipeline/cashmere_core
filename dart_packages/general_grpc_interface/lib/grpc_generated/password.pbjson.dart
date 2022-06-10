///
//  Generated code. Do not modify.
//  source: password.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields,deprecated_member_use_from_same_package

import 'dart:core' as $core;
import 'dart:convert' as $convert;
import 'dart:typed_data' as $typed_data;
@$core.Deprecated('Use changeOwnPasswordRequestDescriptor instead')
const ChangeOwnPasswordRequest$json = const {
  '1': 'ChangeOwnPasswordRequest',
  '2': const [
    const {'1': 'old_password', '3': 1, '4': 1, '5': 9, '10': 'oldPassword'},
    const {'1': 'new_password', '3': 2, '4': 1, '5': 9, '10': 'newPassword'},
  ],
};

/// Descriptor for `ChangeOwnPasswordRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List changeOwnPasswordRequestDescriptor = $convert.base64Decode('ChhDaGFuZ2VPd25QYXNzd29yZFJlcXVlc3QSIQoMb2xkX3Bhc3N3b3JkGAEgASgJUgtvbGRQYXNzd29yZBIhCgxuZXdfcGFzc3dvcmQYAiABKAlSC25ld1Bhc3N3b3Jk');
@$core.Deprecated('Use changeOwnPasswordResponseDescriptor instead')
const ChangeOwnPasswordResponse$json = const {
  '1': 'ChangeOwnPasswordResponse',
  '2': const [
    const {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `ChangeOwnPasswordResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List changeOwnPasswordResponseDescriptor = $convert.base64Decode('ChlDaGFuZ2VPd25QYXNzd29yZFJlc3BvbnNlEhYKBnJlc3VsdBgBIAEoCVIGcmVzdWx0');
@$core.Deprecated('Use resetPasswordRequestDescriptor instead')
const ResetPasswordRequest$json = const {
  '1': 'ResetPasswordRequest',
  '2': const [
    const {'1': 'account_id', '3': 1, '4': 1, '5': 9, '10': 'accountId'},
    const {'1': 'new_password', '3': 2, '4': 1, '5': 9, '10': 'newPassword'},
  ],
};

/// Descriptor for `ResetPasswordRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List resetPasswordRequestDescriptor = $convert.base64Decode('ChRSZXNldFBhc3N3b3JkUmVxdWVzdBIdCgphY2NvdW50X2lkGAEgASgJUglhY2NvdW50SWQSIQoMbmV3X3Bhc3N3b3JkGAIgASgJUgtuZXdQYXNzd29yZA==');
@$core.Deprecated('Use resetPasswordResponseDescriptor instead')
const ResetPasswordResponse$json = const {
  '1': 'ResetPasswordResponse',
  '2': const [
    const {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `ResetPasswordResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List resetPasswordResponseDescriptor = $convert.base64Decode('ChVSZXNldFBhc3N3b3JkUmVzcG9uc2USFgoGcmVzdWx0GAEgASgJUgZyZXN1bHQ=');
