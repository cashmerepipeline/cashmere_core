///
//  Generated code. Do not modify.
//  source: status.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields,deprecated_member_use_from_same_package

import 'dart:core' as $core;
import 'dart:convert' as $convert;
import 'dart:typed_data' as $typed_data;
@$core.Deprecated('Use loginStatusDescriptor instead')
const LoginStatus$json = const {
  '1': 'LoginStatus',
  '2': const [
    const {'1': 'LoggedIn', '2': 0},
    const {'1': 'LoggedOut', '2': 1},
    const {'1': 'LogginFailed', '2': 2},
  ],
};

/// Descriptor for `LoginStatus`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List loginStatusDescriptor = $convert.base64Decode('CgtMb2dpblN0YXR1cxIMCghMb2dnZWRJbhAAEg0KCUxvZ2dlZE91dBABEhAKDExvZ2dpbkZhaWxlZBAC');
@$core.Deprecated('Use accountStatusDescriptor instead')
const AccountStatus$json = const {
  '1': 'AccountStatus',
  '2': const [
    const {'1': 'Stopped', '2': 0},
    const {'1': 'Actived', '2': 1},
  ],
};

/// Descriptor for `AccountStatus`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List accountStatusDescriptor = $convert.base64Decode('Cg1BY2NvdW50U3RhdHVzEgsKB1N0b3BwZWQQABILCgdBY3RpdmVkEAE=');
