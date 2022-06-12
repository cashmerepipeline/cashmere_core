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
@$core.Deprecated('Use dataUploadFileRequestDescriptor instead')
const DataUploadFileRequest$json = const {
  '1': 'DataUploadFileRequest',
  '2': const [
    const {'1': 'name', '3': 6, '4': 1, '5': 11, '6': '.cashmere.Name', '10': 'name'},
    const {'1': 'data_id', '3': 1, '4': 1, '5': 9, '10': 'dataId'},
    const {'1': 'total_chuncks', '3': 2, '4': 1, '5': 4, '10': 'totalChuncks'},
    const {'1': 'current_chunck', '3': 3, '4': 1, '5': 4, '10': 'currentChunck'},
    const {'1': 'chunck', '3': 4, '4': 1, '5': 12, '10': 'chunck'},
    const {'1': 'file_name', '3': 5, '4': 1, '5': 9, '10': 'fileName'},
  ],
};

/// Descriptor for `DataUploadFileRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List dataUploadFileRequestDescriptor = $convert.base64Decode('ChVEYXRhVXBsb2FkRmlsZVJlcXVlc3QSIgoEbmFtZRgGIAEoCzIOLmNhc2htZXJlLk5hbWVSBG5hbWUSFwoHZGF0YV9pZBgBIAEoCVIGZGF0YUlkEiMKDXRvdGFsX2NodW5ja3MYAiABKARSDHRvdGFsQ2h1bmNrcxIlCg5jdXJyZW50X2NodW5jaxgDIAEoBFINY3VycmVudENodW5jaxIWCgZjaHVuY2sYBCABKAxSBmNodW5jaxIbCglmaWxlX25hbWUYBSABKAlSCGZpbGVOYW1l');
@$core.Deprecated('Use dataUploadFileResponseDescriptor instead')
const DataUploadFileResponse$json = const {
  '1': 'DataUploadFileResponse',
  '2': const [
    const {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `DataUploadFileResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List dataUploadFileResponseDescriptor = $convert.base64Decode('ChZEYXRhVXBsb2FkRmlsZVJlc3BvbnNlEhYKBnJlc3VsdBgBIAEoCVIGcmVzdWx0');
@$core.Deprecated('Use dataDownloadFileRequestDescriptor instead')
const DataDownloadFileRequest$json = const {
  '1': 'DataDownloadFileRequest',
  '2': const [
    const {'1': 'data_id', '3': 1, '4': 1, '5': 9, '10': 'dataId'},
  ],
};

/// Descriptor for `DataDownloadFileRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List dataDownloadFileRequestDescriptor = $convert.base64Decode('ChdEYXRhRG93bmxvYWRGaWxlUmVxdWVzdBIXCgdkYXRhX2lkGAEgASgJUgZkYXRhSWQ=');
@$core.Deprecated('Use dataDownloadFileResponseDescriptor instead')
const DataDownloadFileResponse$json = const {
  '1': 'DataDownloadFileResponse',
  '2': const [
    const {'1': 'data_id', '3': 1, '4': 1, '5': 9, '10': 'dataId'},
    const {'1': 'total_chuncks', '3': 2, '4': 1, '5': 4, '10': 'totalChuncks'},
    const {'1': 'current_chunck', '3': 3, '4': 1, '5': 4, '10': 'currentChunck'},
    const {'1': 'chunck', '3': 4, '4': 1, '5': 12, '10': 'chunck'},
    const {'1': 'file_name', '3': 5, '4': 1, '5': 9, '10': 'fileName'},
  ],
};

/// Descriptor for `DataDownloadFileResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List dataDownloadFileResponseDescriptor = $convert.base64Decode('ChhEYXRhRG93bmxvYWRGaWxlUmVzcG9uc2USFwoHZGF0YV9pZBgBIAEoCVIGZGF0YUlkEiMKDXRvdGFsX2NodW5ja3MYAiABKARSDHRvdGFsQ2h1bmNrcxIlCg5jdXJyZW50X2NodW5jaxgDIAEoBFINY3VycmVudENodW5jaxIWCgZjaHVuY2sYBCABKAxSBmNodW5jaxIbCglmaWxlX25hbWUYBSABKAlSCGZpbGVOYW1l');
@$core.Deprecated('Use dataUploadSequenceRequestDescriptor instead')
const DataUploadSequenceRequest$json = const {
  '1': 'DataUploadSequenceRequest',
  '2': const [
    const {'1': 'sequence_name', '3': 1, '4': 1, '5': 9, '10': 'sequenceName'},
    const {'1': 'serial_pattern', '3': 2, '4': 1, '5': 9, '10': 'serialPattern'},
    const {'1': 'sequence_length', '3': 3, '4': 1, '5': 13, '10': 'sequenceLength'},
    const {'1': 'current_file', '3': 4, '4': 1, '5': 9, '10': 'currentFile'},
    const {'1': 'total_chancks', '3': 5, '4': 1, '5': 4, '10': 'totalChancks'},
    const {'1': 'current_chunck', '3': 6, '4': 1, '5': 4, '10': 'currentChunck'},
    const {'1': 'chunck', '3': 7, '4': 1, '5': 12, '10': 'chunck'},
    const {'1': 'data_id', '3': 8, '4': 1, '5': 9, '10': 'dataId'},
  ],
};

/// Descriptor for `DataUploadSequenceRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List dataUploadSequenceRequestDescriptor = $convert.base64Decode('ChlEYXRhVXBsb2FkU2VxdWVuY2VSZXF1ZXN0EiMKDXNlcXVlbmNlX25hbWUYASABKAlSDHNlcXVlbmNlTmFtZRIlCg5zZXJpYWxfcGF0dGVybhgCIAEoCVINc2VyaWFsUGF0dGVybhInCg9zZXF1ZW5jZV9sZW5ndGgYAyABKA1SDnNlcXVlbmNlTGVuZ3RoEiEKDGN1cnJlbnRfZmlsZRgEIAEoCVILY3VycmVudEZpbGUSIwoNdG90YWxfY2hhbmNrcxgFIAEoBFIMdG90YWxDaGFuY2tzEiUKDmN1cnJlbnRfY2h1bmNrGAYgASgEUg1jdXJyZW50Q2h1bmNrEhYKBmNodW5jaxgHIAEoDFIGY2h1bmNrEhcKB2RhdGFfaWQYCCABKAlSBmRhdGFJZA==');
@$core.Deprecated('Use dataUploadSequenceResponseDescriptor instead')
const DataUploadSequenceResponse$json = const {
  '1': 'DataUploadSequenceResponse',
  '2': const [
    const {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `DataUploadSequenceResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List dataUploadSequenceResponseDescriptor = $convert.base64Decode('ChpEYXRhVXBsb2FkU2VxdWVuY2VSZXNwb25zZRIWCgZyZXN1bHQYASABKAlSBnJlc3VsdA==');
@$core.Deprecated('Use dataDownloadSequenceRequestDescriptor instead')
const DataDownloadSequenceRequest$json = const {
  '1': 'DataDownloadSequenceRequest',
  '2': const [
    const {'1': 'data_id', '3': 1, '4': 1, '5': 9, '10': 'dataId'},
  ],
};

/// Descriptor for `DataDownloadSequenceRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List dataDownloadSequenceRequestDescriptor = $convert.base64Decode('ChtEYXRhRG93bmxvYWRTZXF1ZW5jZVJlcXVlc3QSFwoHZGF0YV9pZBgBIAEoCVIGZGF0YUlk');
@$core.Deprecated('Use dataDownloadSequenceResponseDescriptor instead')
const DataDownloadSequenceResponse$json = const {
  '1': 'DataDownloadSequenceResponse',
  '2': const [
    const {'1': 'data_id', '3': 8, '4': 1, '5': 9, '10': 'dataId'},
    const {'1': 'sequence_name', '3': 1, '4': 1, '5': 9, '10': 'sequenceName'},
    const {'1': 'serial_pattern', '3': 2, '4': 1, '5': 9, '10': 'serialPattern'},
    const {'1': 'sequence_length', '3': 3, '4': 1, '5': 13, '10': 'sequenceLength'},
    const {'1': 'current_file', '3': 4, '4': 1, '5': 9, '10': 'currentFile'},
    const {'1': 'total_chancks', '3': 5, '4': 1, '5': 4, '10': 'totalChancks'},
    const {'1': 'current_chunck', '3': 6, '4': 1, '5': 4, '10': 'currentChunck'},
    const {'1': 'chunck', '3': 7, '4': 1, '5': 12, '10': 'chunck'},
  ],
};

/// Descriptor for `DataDownloadSequenceResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List dataDownloadSequenceResponseDescriptor = $convert.base64Decode('ChxEYXRhRG93bmxvYWRTZXF1ZW5jZVJlc3BvbnNlEhcKB2RhdGFfaWQYCCABKAlSBmRhdGFJZBIjCg1zZXF1ZW5jZV9uYW1lGAEgASgJUgxzZXF1ZW5jZU5hbWUSJQoOc2VyaWFsX3BhdHRlcm4YAiABKAlSDXNlcmlhbFBhdHRlcm4SJwoPc2VxdWVuY2VfbGVuZ3RoGAMgASgNUg5zZXF1ZW5jZUxlbmd0aBIhCgxjdXJyZW50X2ZpbGUYBCABKAlSC2N1cnJlbnRGaWxlEiMKDXRvdGFsX2NoYW5ja3MYBSABKARSDHRvdGFsQ2hhbmNrcxIlCg5jdXJyZW50X2NodW5jaxgGIAEoBFINY3VycmVudENodW5jaxIWCgZjaHVuY2sYByABKAxSBmNodW5jaw==');
@$core.Deprecated('Use dataUploadSetRequestDescriptor instead')
const DataUploadSetRequest$json = const {
  '1': 'DataUploadSetRequest',
  '2': const [
    const {'1': 'data_id', '3': 1, '4': 1, '5': 9, '10': 'dataId'},
    const {'1': 'set_name', '3': 2, '4': 1, '5': 9, '10': 'setName'},
    const {'1': 'file_counts', '3': 3, '4': 1, '5': 13, '10': 'fileCounts'},
    const {'1': 'current_file', '3': 4, '4': 1, '5': 9, '10': 'currentFile'},
    const {'1': 'total_chancks', '3': 5, '4': 1, '5': 4, '10': 'totalChancks'},
    const {'1': 'current_chunck', '3': 6, '4': 1, '5': 13, '10': 'currentChunck'},
    const {'1': 'chunck', '3': 7, '4': 1, '5': 12, '10': 'chunck'},
  ],
};

/// Descriptor for `DataUploadSetRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List dataUploadSetRequestDescriptor = $convert.base64Decode('ChREYXRhVXBsb2FkU2V0UmVxdWVzdBIXCgdkYXRhX2lkGAEgASgJUgZkYXRhSWQSGQoIc2V0X25hbWUYAiABKAlSB3NldE5hbWUSHwoLZmlsZV9jb3VudHMYAyABKA1SCmZpbGVDb3VudHMSIQoMY3VycmVudF9maWxlGAQgASgJUgtjdXJyZW50RmlsZRIjCg10b3RhbF9jaGFuY2tzGAUgASgEUgx0b3RhbENoYW5ja3MSJQoOY3VycmVudF9jaHVuY2sYBiABKA1SDWN1cnJlbnRDaHVuY2sSFgoGY2h1bmNrGAcgASgMUgZjaHVuY2s=');
@$core.Deprecated('Use dataUploadSetResponseDescriptor instead')
const DataUploadSetResponse$json = const {
  '1': 'DataUploadSetResponse',
  '2': const [
    const {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `DataUploadSetResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List dataUploadSetResponseDescriptor = $convert.base64Decode('ChVEYXRhVXBsb2FkU2V0UmVzcG9uc2USFgoGcmVzdWx0GAEgASgJUgZyZXN1bHQ=');
@$core.Deprecated('Use dataDownloadSetRequestDescriptor instead')
const DataDownloadSetRequest$json = const {
  '1': 'DataDownloadSetRequest',
  '2': const [
    const {'1': 'data_id', '3': 1, '4': 1, '5': 9, '10': 'dataId'},
  ],
};

/// Descriptor for `DataDownloadSetRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List dataDownloadSetRequestDescriptor = $convert.base64Decode('ChZEYXRhRG93bmxvYWRTZXRSZXF1ZXN0EhcKB2RhdGFfaWQYASABKAlSBmRhdGFJZA==');
@$core.Deprecated('Use dataDownloadSetResponseDescriptor instead')
const DataDownloadSetResponse$json = const {
  '1': 'DataDownloadSetResponse',
  '2': const [
    const {'1': 'data_id', '3': 1, '4': 1, '5': 9, '10': 'dataId'},
    const {'1': 'set_name', '3': 2, '4': 1, '5': 9, '10': 'setName'},
    const {'1': 'file_counts', '3': 3, '4': 1, '5': 13, '10': 'fileCounts'},
    const {'1': 'current_file', '3': 4, '4': 1, '5': 9, '10': 'currentFile'},
    const {'1': 'total_chancks', '3': 5, '4': 1, '5': 4, '10': 'totalChancks'},
    const {'1': 'current_chunck', '3': 6, '4': 1, '5': 13, '10': 'currentChunck'},
    const {'1': 'chunck', '3': 7, '4': 1, '5': 12, '10': 'chunck'},
  ],
};

/// Descriptor for `DataDownloadSetResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List dataDownloadSetResponseDescriptor = $convert.base64Decode('ChdEYXRhRG93bmxvYWRTZXRSZXNwb25zZRIXCgdkYXRhX2lkGAEgASgJUgZkYXRhSWQSGQoIc2V0X25hbWUYAiABKAlSB3NldE5hbWUSHwoLZmlsZV9jb3VudHMYAyABKA1SCmZpbGVDb3VudHMSIQoMY3VycmVudF9maWxlGAQgASgJUgtjdXJyZW50RmlsZRIjCg10b3RhbF9jaGFuY2tzGAUgASgEUgx0b3RhbENoYW5ja3MSJQoOY3VycmVudF9jaHVuY2sYBiABKA1SDWN1cnJlbnRDaHVuY2sSFgoGY2h1bmNrGAcgASgMUgZjaHVuY2s=');
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
    const {'1': 'data_id', '3': 1, '4': 1, '5': 9, '10': 'dataId'},
  ],
};

/// Descriptor for `MarkDataRemovedRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List markDataRemovedRequestDescriptor = $convert.base64Decode('ChZNYXJrRGF0YVJlbW92ZWRSZXF1ZXN0EhcKB2RhdGFfaWQYASABKAlSBmRhdGFJZA==');
@$core.Deprecated('Use markDataRemovedResponseDescriptor instead')
const MarkDataRemovedResponse$json = const {
  '1': 'MarkDataRemovedResponse',
  '2': const [
    const {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `MarkDataRemovedResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List markDataRemovedResponseDescriptor = $convert.base64Decode('ChdNYXJrRGF0YVJlbW92ZWRSZXNwb25zZRIWCgZyZXN1bHQYASABKAlSBnJlc3VsdA==');
