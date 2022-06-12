///
//  Generated code. Do not modify.
//  source: account.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields,deprecated_member_use_from_same_package

import 'dart:core' as $core;
import 'dart:convert' as $convert;
import 'dart:typed_data' as $typed_data;
@$core.Deprecated('Use newAccountRequestDescriptor instead')
const NewAccountRequest$json = const {
  '1': 'NewAccountRequest',
  '2': const [
    const {'1': 'organization_id', '3': 1, '4': 1, '5': 9, '10': 'organizationId'},
    const {'1': 'department_id', '3': 2, '4': 1, '5': 9, '10': 'departmentId'},
    const {'1': 'group_id', '3': 3, '4': 1, '5': 9, '10': 'groupId'},
    const {'1': 'phone', '3': 4, '4': 1, '5': 9, '10': 'phone'},
    const {'1': 'password', '3': 5, '4': 1, '5': 9, '10': 'password'},
  ],
};

/// Descriptor for `NewAccountRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List newAccountRequestDescriptor = $convert.base64Decode('ChFOZXdBY2NvdW50UmVxdWVzdBInCg9vcmdhbml6YXRpb25faWQYASABKAlSDm9yZ2FuaXphdGlvbklkEiMKDWRlcGFydG1lbnRfaWQYAiABKAlSDGRlcGFydG1lbnRJZBIZCghncm91cF9pZBgDIAEoCVIHZ3JvdXBJZBIUCgVwaG9uZRgEIAEoCVIFcGhvbmUSGgoIcGFzc3dvcmQYBSABKAlSCHBhc3N3b3Jk');
@$core.Deprecated('Use newAccountResponseDescriptor instead')
const NewAccountResponse$json = const {
  '1': 'NewAccountResponse',
  '2': const [
    const {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `NewAccountResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List newAccountResponseDescriptor = $convert.base64Decode('ChJOZXdBY2NvdW50UmVzcG9uc2USFgoGcmVzdWx0GAEgASgJUgZyZXN1bHQ=');
@$core.Deprecated('Use registerRequestDescriptor instead')
const RegisterRequest$json = const {
  '1': 'RegisterRequest',
  '2': const [
    const {'1': 'organization_id', '3': 1, '4': 1, '5': 9, '10': 'organizationId'},
    const {'1': 'department_id', '3': 2, '4': 1, '5': 9, '10': 'departmentId'},
    const {'1': 'phone', '3': 4, '4': 1, '5': 9, '10': 'phone'},
    const {'1': 'password', '3': 5, '4': 1, '5': 9, '10': 'password'},
  ],
};

/// Descriptor for `RegisterRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List registerRequestDescriptor = $convert.base64Decode('Cg9SZWdpc3RlclJlcXVlc3QSJwoPb3JnYW5pemF0aW9uX2lkGAEgASgJUg5vcmdhbml6YXRpb25JZBIjCg1kZXBhcnRtZW50X2lkGAIgASgJUgxkZXBhcnRtZW50SWQSFAoFcGhvbmUYBCABKAlSBXBob25lEhoKCHBhc3N3b3JkGAUgASgJUghwYXNzd29yZA==');
@$core.Deprecated('Use registerResponseDescriptor instead')
const RegisterResponse$json = const {
  '1': 'RegisterResponse',
  '2': const [
    const {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `RegisterResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List registerResponseDescriptor = $convert.base64Decode('ChBSZWdpc3RlclJlc3BvbnNlEhYKBnJlc3VsdBgBIAEoCVIGcmVzdWx0');
@$core.Deprecated('Use changeOwnPhoneRequestDescriptor instead')
const ChangeOwnPhoneRequest$json = const {
  '1': 'ChangeOwnPhoneRequest',
  '2': const [
    const {'1': 'old_phone', '3': 1, '4': 1, '5': 9, '10': 'oldPhone'},
    const {'1': 'new_phone', '3': 2, '4': 1, '5': 9, '10': 'newPhone'},
    const {'1': 'password', '3': 3, '4': 1, '5': 9, '10': 'password'},
  ],
};

/// Descriptor for `ChangeOwnPhoneRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List changeOwnPhoneRequestDescriptor = $convert.base64Decode('ChVDaGFuZ2VPd25QaG9uZVJlcXVlc3QSGwoJb2xkX3Bob25lGAEgASgJUghvbGRQaG9uZRIbCgluZXdfcGhvbmUYAiABKAlSCG5ld1Bob25lEhoKCHBhc3N3b3JkGAMgASgJUghwYXNzd29yZA==');
@$core.Deprecated('Use changePhoneOwnResponseDescriptor instead')
const ChangePhoneOwnResponse$json = const {
  '1': 'ChangePhoneOwnResponse',
  '2': const [
    const {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `ChangePhoneOwnResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List changePhoneOwnResponseDescriptor = $convert.base64Decode('ChZDaGFuZ2VQaG9uZU93blJlc3BvbnNlEhYKBnJlc3VsdBgBIAEoCVIGcmVzdWx0');
