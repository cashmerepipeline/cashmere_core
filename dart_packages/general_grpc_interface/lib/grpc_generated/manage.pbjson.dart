///
//  Generated code. Do not modify.
//  source: manage.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,deprecated_member_use_from_same_package,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

import 'dart:core' as $core;
import 'dart:convert' as $convert;
import 'dart:typed_data' as $typed_data;
@$core.Deprecated('Use manageDescriptor instead')
const Manage$json = const {
  '1': 'Manage',
  '2': const [
    const {'1': 'manage_id', '3': 1, '4': 1, '5': 5, '10': 'manageId'},
    const {'1': 'name_map', '3': 2, '4': 1, '5': 12, '10': 'nameMap'},
  ],
};

/// Descriptor for `Manage`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List manageDescriptor = $convert.base64Decode('CgZNYW5hZ2USGwoJbWFuYWdlX2lkGAEgASgFUghtYW5hZ2VJZBIZCghuYW1lX21hcBgCIAEoDFIHbmFtZU1hcA==');
@$core.Deprecated('Use getManagesRequestDescriptor instead')
const GetManagesRequest$json = const {
  '1': 'GetManagesRequest',
};

/// Descriptor for `GetManagesRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getManagesRequestDescriptor = $convert.base64Decode('ChFHZXRNYW5hZ2VzUmVxdWVzdA==');
@$core.Deprecated('Use getManagesResponseDescriptor instead')
const GetManagesResponse$json = const {
  '1': 'GetManagesResponse',
  '2': const [
    const {'1': 'manages', '3': 1, '4': 3, '5': 11, '6': '.cashmere.Manage', '10': 'manages'},
  ],
};

/// Descriptor for `GetManagesResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getManagesResponseDescriptor = $convert.base64Decode('ChJHZXRNYW5hZ2VzUmVzcG9uc2USKgoHbWFuYWdlcxgBIAMoCzIQLmNhc2htZXJlLk1hbmFnZVIHbWFuYWdlcw==');
@$core.Deprecated('Use getManageEntryCountRequestDescriptor instead')
const GetManageEntryCountRequest$json = const {
  '1': 'GetManageEntryCountRequest',
  '2': const [
    const {'1': 'manage_id', '3': 1, '4': 1, '5': 5, '10': 'manageId'},
  ],
};

/// Descriptor for `GetManageEntryCountRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getManageEntryCountRequestDescriptor = $convert.base64Decode('ChpHZXRNYW5hZ2VFbnRyeUNvdW50UmVxdWVzdBIbCgltYW5hZ2VfaWQYASABKAVSCG1hbmFnZUlk');
@$core.Deprecated('Use getManageEntryCountResponseDescriptor instead')
const GetManageEntryCountResponse$json = const {
  '1': 'GetManageEntryCountResponse',
  '2': const [
    const {'1': 'count', '3': 1, '4': 1, '5': 4, '10': 'count'},
  ],
};

/// Descriptor for `GetManageEntryCountResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getManageEntryCountResponseDescriptor = $convert.base64Decode('ChtHZXRNYW5hZ2VFbnRyeUNvdW50UmVzcG9uc2USFAoFY291bnQYASABKARSBWNvdW50');
