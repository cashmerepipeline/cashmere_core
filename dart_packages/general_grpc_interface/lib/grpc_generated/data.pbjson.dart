///
//  Generated code. Do not modify.
//  source: data.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields,deprecated_member_use_from_same_package

import 'dart:core' as $core;
import 'dart:convert' as $convert;
import 'dart:typed_data' as $typed_data;
@$core.Deprecated('Use dataTypeDescriptor instead')
const DataType$json = const {
  '1': 'DataType',
  '2': const [
    const {'1': 'File', '2': 0},
    const {'1': 'Sequence', '2': 1},
    const {'1': 'Set', '2': 2},
    const {'1': 'Message', '2': 3},
    const {'1': 'Document', '2': 4},
  ],
};

/// Descriptor for `DataType`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List dataTypeDescriptor = $convert.base64Decode('CghEYXRhVHlwZRIICgRGaWxlEAASDAoIU2VxdWVuY2UQARIHCgNTZXQQAhILCgdNZXNzYWdlEAMSDAoIRG9jdW1lbnQQBA==');
@$core.Deprecated('Use newDataRequestDescriptor instead')
const NewDataRequest$json = const {
  '1': 'NewDataRequest',
  '2': const [
    const {'1': 'name', '3': 1, '4': 1, '5': 11, '6': '.cashmere.Name', '10': 'name'},
    const {'1': 'data_type', '3': 2, '4': 1, '5': 14, '6': '.cashmere.DataType', '10': 'dataType'},
    const {'1': 'manage_id', '3': 3, '4': 1, '5': 5, '10': 'manageId'},
    const {'1': 'entity_id', '3': 4, '4': 1, '5': 9, '10': 'entityId'},
  ],
};

/// Descriptor for `NewDataRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List newDataRequestDescriptor = $convert.base64Decode('Cg5OZXdEYXRhUmVxdWVzdBIiCgRuYW1lGAEgASgLMg4uY2FzaG1lcmUuTmFtZVIEbmFtZRIvCglkYXRhX3R5cGUYAiABKA4yEi5jYXNobWVyZS5EYXRhVHlwZVIIZGF0YVR5cGUSGwoJbWFuYWdlX2lkGAMgASgFUghtYW5hZ2VJZBIbCgllbnRpdHlfaWQYBCABKAlSCGVudGl0eUlk');
@$core.Deprecated('Use newDataResponseDescriptor instead')
const NewDataResponse$json = const {
  '1': 'NewDataResponse',
  '2': const [
    const {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `NewDataResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List newDataResponseDescriptor = $convert.base64Decode('Cg9OZXdEYXRhUmVzcG9uc2USFgoGcmVzdWx0GAEgASgJUgZyZXN1bHQ=');
@$core.Deprecated('Use associateDataRequestDescriptor instead')
const AssociateDataRequest$json = const {
  '1': 'AssociateDataRequest',
  '2': const [
    const {'1': 'data_id', '3': 1, '4': 1, '5': 9, '10': 'dataId'},
    const {'1': 'manage_id', '3': 2, '4': 1, '5': 5, '10': 'manageId'},
    const {'1': 'entity_id', '3': 3, '4': 1, '5': 9, '10': 'entityId'},
  ],
};

/// Descriptor for `AssociateDataRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List associateDataRequestDescriptor = $convert.base64Decode('ChRBc3NvY2lhdGVEYXRhUmVxdWVzdBIXCgdkYXRhX2lkGAEgASgJUgZkYXRhSWQSGwoJbWFuYWdlX2lkGAIgASgFUghtYW5hZ2VJZBIbCgllbnRpdHlfaWQYAyABKAlSCGVudGl0eUlk');
@$core.Deprecated('Use associateDataResponseDescriptor instead')
const AssociateDataResponse$json = const {
  '1': 'AssociateDataResponse',
  '2': const [
    const {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `AssociateDataResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List associateDataResponseDescriptor = $convert.base64Decode('ChVBc3NvY2lhdGVEYXRhUmVzcG9uc2USFgoGcmVzdWx0GAEgASgJUgZyZXN1bHQ=');
@$core.Deprecated('Use disassociateDataRequestDescriptor instead')
const DisassociateDataRequest$json = const {
  '1': 'DisassociateDataRequest',
  '2': const [
    const {'1': 'data_id', '3': 1, '4': 1, '5': 9, '10': 'dataId'},
    const {'1': 'manage_id', '3': 2, '4': 1, '5': 5, '10': 'manageId'},
    const {'1': 'entity_id', '3': 3, '4': 1, '5': 9, '10': 'entityId'},
  ],
};

/// Descriptor for `DisassociateDataRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List disassociateDataRequestDescriptor = $convert.base64Decode('ChdEaXNhc3NvY2lhdGVEYXRhUmVxdWVzdBIXCgdkYXRhX2lkGAEgASgJUgZkYXRhSWQSGwoJbWFuYWdlX2lkGAIgASgFUghtYW5hZ2VJZBIbCgllbnRpdHlfaWQYAyABKAlSCGVudGl0eUlk');
@$core.Deprecated('Use disassociateDataResponseDescriptor instead')
const DisassociateDataResponse$json = const {
  '1': 'DisassociateDataResponse',
  '2': const [
    const {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `DisassociateDataResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List disassociateDataResponseDescriptor = $convert.base64Decode('ChhEaXNhc3NvY2lhdGVEYXRhUmVzcG9uc2USFgoGcmVzdWx0GAEgASgJUgZyZXN1bHQ=');
@$core.Deprecated('Use getDataInfoRequestDescriptor instead')
const GetDataInfoRequest$json = const {
  '1': 'GetDataInfoRequest',
  '2': const [
    const {'1': 'data_id', '3': 1, '4': 1, '5': 9, '10': 'dataId'},
  ],
};

/// Descriptor for `GetDataInfoRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getDataInfoRequestDescriptor = $convert.base64Decode('ChJHZXREYXRhSW5mb1JlcXVlc3QSFwoHZGF0YV9pZBgBIAEoCVIGZGF0YUlk');
@$core.Deprecated('Use getDataInfoResponseDescriptor instead')
const GetDataInfoResponse$json = const {
  '1': 'GetDataInfoResponse',
  '2': const [
    const {'1': 'data', '3': 1, '4': 1, '5': 12, '10': 'data'},
  ],
};

/// Descriptor for `GetDataInfoResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getDataInfoResponseDescriptor = $convert.base64Decode('ChNHZXREYXRhSW5mb1Jlc3BvbnNlEhIKBGRhdGEYASABKAxSBGRhdGE=');
@$core.Deprecated('Use markDataRemovedRequestDescriptor instead')
const MarkDataRemovedRequest$json = const {
  '1': 'MarkDataRemovedRequest',
  '2': const [
    const {'1': 'manage_id', '3': 1, '4': 1, '5': 5, '10': 'manageId'},
    const {'1': 'entity_id', '3': 2, '4': 1, '5': 9, '10': 'entityId'},
    const {'1': 'data_id', '3': 3, '4': 1, '5': 9, '10': 'dataId'},
  ],
};

/// Descriptor for `MarkDataRemovedRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List markDataRemovedRequestDescriptor = $convert.base64Decode('ChZNYXJrRGF0YVJlbW92ZWRSZXF1ZXN0EhsKCW1hbmFnZV9pZBgBIAEoBVIIbWFuYWdlSWQSGwoJZW50aXR5X2lkGAIgASgJUghlbnRpdHlJZBIXCgdkYXRhX2lkGAMgASgJUgZkYXRhSWQ=');
@$core.Deprecated('Use markDataRemovedResponseDescriptor instead')
const MarkDataRemovedResponse$json = const {
  '1': 'MarkDataRemovedResponse',
  '2': const [
    const {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `MarkDataRemovedResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List markDataRemovedResponseDescriptor = $convert.base64Decode('ChdNYXJrRGF0YVJlbW92ZWRSZXNwb25zZRIWCgZyZXN1bHQYASABKAlSBnJlc3VsdA==');
