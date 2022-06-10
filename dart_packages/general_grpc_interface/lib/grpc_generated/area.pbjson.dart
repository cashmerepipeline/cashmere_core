///
//  Generated code. Do not modify.
//  source: area.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields,deprecated_member_use_from_same_package

import 'dart:core' as $core;
import 'dart:convert' as $convert;
import 'dart:typed_data' as $typed_data;
@$core.Deprecated('Use areaLevelDescriptor instead')
const AreaLevel$json = const {
  '1': 'AreaLevel',
  '2': const [
    const {'1': 'Country', '2': 0},
    const {'1': 'Province', '2': 1},
    const {'1': 'City', '2': 2},
    const {'1': 'Area', '2': 3},
  ],
};

/// Descriptor for `AreaLevel`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List areaLevelDescriptor = $convert.base64Decode('CglBcmVhTGV2ZWwSCwoHQ291bnRyeRAAEgwKCFByb3ZpbmNlEAESCAoEQ2l0eRACEggKBEFyZWEQAw==');
@$core.Deprecated('Use newAreaRequestDescriptor instead')
const NewAreaRequest$json = const {
  '1': 'NewAreaRequest',
  '2': const [
    const {'1': 'language', '3': 1, '4': 1, '5': 9, '10': 'language'},
    const {'1': 'name', '3': 2, '4': 1, '5': 9, '10': 'name'},
    const {'1': 'parent_id', '3': 3, '4': 1, '5': 9, '10': 'parentId'},
    const {'1': 'code', '3': 4, '4': 1, '5': 9, '10': 'code'},
    const {'1': 'level', '3': 5, '4': 1, '5': 14, '6': '.cashmere.AreaLevel', '10': 'level'},
  ],
};

/// Descriptor for `NewAreaRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List newAreaRequestDescriptor = $convert.base64Decode('Cg5OZXdBcmVhUmVxdWVzdBIaCghsYW5ndWFnZRgBIAEoCVIIbGFuZ3VhZ2USEgoEbmFtZRgCIAEoCVIEbmFtZRIbCglwYXJlbnRfaWQYAyABKAlSCHBhcmVudElkEhIKBGNvZGUYBCABKAlSBGNvZGUSKQoFbGV2ZWwYBSABKA4yEy5jYXNobWVyZS5BcmVhTGV2ZWxSBWxldmVs');
@$core.Deprecated('Use newAreaResponseDescriptor instead')
const NewAreaResponse$json = const {
  '1': 'NewAreaResponse',
  '2': const [
    const {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `NewAreaResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List newAreaResponseDescriptor = $convert.base64Decode('Cg9OZXdBcmVhUmVzcG9uc2USFgoGcmVzdWx0GAEgASgJUgZyZXN1bHQ=');
@$core.Deprecated('Use editAreaRequestDescriptor instead')
const EditAreaRequest$json = const {
  '1': 'EditAreaRequest',
  '2': const [
    const {'1': 'area_id', '3': 1, '4': 1, '5': 9, '10': 'areaId'},
    const {'1': 'new_parent_id', '3': 3, '4': 1, '5': 9, '10': 'newParentId'},
    const {'1': 'new_level', '3': 4, '4': 1, '5': 14, '6': '.cashmere.AreaLevel', '10': 'newLevel'},
  ],
};

/// Descriptor for `EditAreaRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List editAreaRequestDescriptor = $convert.base64Decode('Cg9FZGl0QXJlYVJlcXVlc3QSFwoHYXJlYV9pZBgBIAEoCVIGYXJlYUlkEiIKDW5ld19wYXJlbnRfaWQYAyABKAlSC25ld1BhcmVudElkEjAKCW5ld19sZXZlbBgEIAEoDjITLmNhc2htZXJlLkFyZWFMZXZlbFIIbmV3TGV2ZWw=');
@$core.Deprecated('Use editAreaResponseDescriptor instead')
const EditAreaResponse$json = const {
  '1': 'EditAreaResponse',
  '2': const [
    const {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `EditAreaResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List editAreaResponseDescriptor = $convert.base64Decode('ChBFZGl0QXJlYVJlc3BvbnNlEhYKBnJlc3VsdBgBIAEoCVIGcmVzdWx0');
