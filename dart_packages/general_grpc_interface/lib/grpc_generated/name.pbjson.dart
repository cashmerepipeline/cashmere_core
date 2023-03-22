///
//  Generated code. Do not modify.
//  source: name.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,deprecated_member_use_from_same_package,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

import 'dart:core' as $core;
import 'dart:convert' as $convert;
import 'dart:typed_data' as $typed_data;
@$core.Deprecated('Use nameDescriptor instead')
const Name$json = const {
  '1': 'Name',
  '2': const [
    const {'1': 'language', '3': 1, '4': 1, '5': 9, '10': 'language'},
    const {'1': 'name', '3': 2, '4': 1, '5': 9, '10': 'name'},
  ],
};

/// Descriptor for `Name`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List nameDescriptor = $convert.base64Decode('CgROYW1lEhoKCGxhbmd1YWdlGAEgASgJUghsYW5ndWFnZRISCgRuYW1lGAIgASgJUgRuYW1l');
@$core.Deprecated('Use nameFieldDescriptor instead')
const NameField$json = const {
  '1': 'NameField',
  '2': const [
    const {'1': 'name_field', '3': 1, '4': 3, '5': 11, '6': '.cashmere.NameField.NameFieldEntry', '10': 'nameField'},
  ],
  '3': const [NameField_NameFieldEntry$json],
};

@$core.Deprecated('Use nameFieldDescriptor instead')
const NameField_NameFieldEntry$json = const {
  '1': 'NameFieldEntry',
  '2': const [
    const {'1': 'key', '3': 1, '4': 1, '5': 9, '10': 'key'},
    const {'1': 'value', '3': 2, '4': 1, '5': 9, '10': 'value'},
  ],
  '7': const {'7': true},
};

/// Descriptor for `NameField`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List nameFieldDescriptor = $convert.base64Decode('CglOYW1lRmllbGQSQQoKbmFtZV9maWVsZBgBIAMoCzIiLmNhc2htZXJlLk5hbWVGaWVsZC5OYW1lRmllbGRFbnRyeVIJbmFtZUZpZWxkGjwKDk5hbWVGaWVsZEVudHJ5EhAKA2tleRgBIAEoCVIDa2V5EhQKBXZhbHVlGAIgASgJUgV2YWx1ZToCOAE=');
@$core.Deprecated('Use renameRequestDescriptor instead')
const RenameRequest$json = const {
  '1': 'RenameRequest',
  '2': const [
    const {'1': 'manage_id', '3': 1, '4': 1, '5': 5, '10': 'manageId'},
    const {'1': 'entity_id', '3': 2, '4': 1, '5': 9, '10': 'entityId'},
    const {'1': 'language', '3': 3, '4': 1, '5': 9, '10': 'language'},
    const {'1': 'new_name', '3': 4, '4': 1, '5': 9, '10': 'newName'},
  ],
};

/// Descriptor for `RenameRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List renameRequestDescriptor = $convert.base64Decode('Cg1SZW5hbWVSZXF1ZXN0EhsKCW1hbmFnZV9pZBgBIAEoBVIIbWFuYWdlSWQSGwoJZW50aXR5X2lkGAIgASgJUghlbnRpdHlJZBIaCghsYW5ndWFnZRgDIAEoCVIIbGFuZ3VhZ2USGQoIbmV3X25hbWUYBCABKAlSB25ld05hbWU=');
@$core.Deprecated('Use renameResponseDescriptor instead')
const RenameResponse$json = const {
  '1': 'RenameResponse',
  '2': const [
    const {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `RenameResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List renameResponseDescriptor = $convert.base64Decode('Cg5SZW5hbWVSZXNwb25zZRIWCgZyZXN1bHQYASABKAlSBnJlc3VsdA==');
@$core.Deprecated('Use newLanguageNameRequestDescriptor instead')
const NewLanguageNameRequest$json = const {
  '1': 'NewLanguageNameRequest',
  '2': const [
    const {'1': 'manage_id', '3': 1, '4': 1, '5': 5, '10': 'manageId'},
    const {'1': 'entity_id', '3': 2, '4': 1, '5': 9, '10': 'entityId'},
    const {'1': 'language', '3': 3, '4': 1, '5': 9, '10': 'language'},
    const {'1': 'new_name', '3': 4, '4': 1, '5': 9, '10': 'newName'},
  ],
};

/// Descriptor for `NewLanguageNameRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List newLanguageNameRequestDescriptor = $convert.base64Decode('ChZOZXdMYW5ndWFnZU5hbWVSZXF1ZXN0EhsKCW1hbmFnZV9pZBgBIAEoBVIIbWFuYWdlSWQSGwoJZW50aXR5X2lkGAIgASgJUghlbnRpdHlJZBIaCghsYW5ndWFnZRgDIAEoCVIIbGFuZ3VhZ2USGQoIbmV3X25hbWUYBCABKAlSB25ld05hbWU=');
@$core.Deprecated('Use newLanguageNameResponseDescriptor instead')
const NewLanguageNameResponse$json = const {
  '1': 'NewLanguageNameResponse',
  '2': const [
    const {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `NewLanguageNameResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List newLanguageNameResponseDescriptor = $convert.base64Decode('ChdOZXdMYW5ndWFnZU5hbWVSZXNwb25zZRIWCgZyZXN1bHQYASABKAlSBnJlc3VsdA==');
@$core.Deprecated('Use removeLanguageNameRequestDescriptor instead')
const RemoveLanguageNameRequest$json = const {
  '1': 'RemoveLanguageNameRequest',
  '2': const [
    const {'1': 'manage_id', '3': 1, '4': 1, '5': 5, '10': 'manageId'},
    const {'1': 'entity_id', '3': 2, '4': 1, '5': 9, '10': 'entityId'},
    const {'1': 'language', '3': 3, '4': 1, '5': 9, '10': 'language'},
  ],
};

/// Descriptor for `RemoveLanguageNameRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List removeLanguageNameRequestDescriptor = $convert.base64Decode('ChlSZW1vdmVMYW5ndWFnZU5hbWVSZXF1ZXN0EhsKCW1hbmFnZV9pZBgBIAEoBVIIbWFuYWdlSWQSGwoJZW50aXR5X2lkGAIgASgJUghlbnRpdHlJZBIaCghsYW5ndWFnZRgDIAEoCVIIbGFuZ3VhZ2U=');
@$core.Deprecated('Use removeLanguageNameResponseDescriptor instead')
const RemoveLanguageNameResponse$json = const {
  '1': 'RemoveLanguageNameResponse',
  '2': const [
    const {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `RemoveLanguageNameResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List removeLanguageNameResponseDescriptor = $convert.base64Decode('ChpSZW1vdmVMYW5ndWFnZU5hbWVSZXNwb25zZRIWCgZyZXN1bHQYASABKAlSBnJlc3VsdA==');
