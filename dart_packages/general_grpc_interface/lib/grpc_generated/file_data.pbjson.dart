///
//  Generated code. Do not modify.
//  source: file_data.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields,deprecated_member_use_from_same_package

import 'dart:core' as $core;
import 'dart:convert' as $convert;
import 'dart:typed_data' as $typed_data;
@$core.Deprecated('Use fileInfoDescriptor instead')
const FileInfo$json = const {
  '1': 'FileInfo',
  '2': const [
    const {'1': 'file_name', '3': 1, '4': 1, '5': 9, '10': 'fileName'},
    const {'1': 'md5', '3': 2, '4': 1, '5': 9, '10': 'md5'},
    const {'1': 'size', '3': 3, '4': 1, '5': 4, '10': 'size'},
    const {'1': 'last_modified_time', '3': 4, '4': 1, '5': 3, '10': 'lastModifiedTime'},
  ],
};

/// Descriptor for `FileInfo`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List fileInfoDescriptor = $convert.base64Decode('CghGaWxlSW5mbxIbCglmaWxlX25hbWUYASABKAlSCGZpbGVOYW1lEhAKA21kNRgCIAEoCVIDbWQ1EhIKBHNpemUYAyABKARSBHNpemUSLAoSbGFzdF9tb2RpZmllZF90aW1lGAQgASgDUhBsYXN0TW9kaWZpZWRUaW1l');
@$core.Deprecated('Use fileDataUploadFileRequestDescriptor instead')
const FileDataUploadFileRequest$json = const {
  '1': 'FileDataUploadFileRequest',
  '2': const [
    const {'1': 'data_id', '3': 1, '4': 1, '5': 9, '10': 'dataId'},
    const {'1': 'total_chunks', '3': 2, '4': 1, '5': 4, '10': 'totalChunks'},
    const {'1': 'current_chunk_index', '3': 3, '4': 1, '5': 4, '10': 'currentChunkIndex'},
    const {'1': 'chunk', '3': 4, '4': 1, '5': 12, '10': 'chunk'},
    const {'1': 'file_name', '3': 5, '4': 1, '5': 9, '10': 'fileName'},
    const {'1': 'md5', '3': 6, '4': 1, '5': 9, '10': 'md5'},
    const {'1': 'last_modified_time', '3': 7, '4': 1, '5': 3, '10': 'lastModifiedTime'},
  ],
};

/// Descriptor for `FileDataUploadFileRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List fileDataUploadFileRequestDescriptor = $convert.base64Decode('ChlGaWxlRGF0YVVwbG9hZEZpbGVSZXF1ZXN0EhcKB2RhdGFfaWQYASABKAlSBmRhdGFJZBIhCgx0b3RhbF9jaHVua3MYAiABKARSC3RvdGFsQ2h1bmtzEi4KE2N1cnJlbnRfY2h1bmtfaW5kZXgYAyABKARSEWN1cnJlbnRDaHVua0luZGV4EhQKBWNodW5rGAQgASgMUgVjaHVuaxIbCglmaWxlX25hbWUYBSABKAlSCGZpbGVOYW1lEhAKA21kNRgGIAEoCVIDbWQ1EiwKEmxhc3RfbW9kaWZpZWRfdGltZRgHIAEoA1IQbGFzdE1vZGlmaWVkVGltZQ==');
@$core.Deprecated('Use fileDataUploadFileResponseDescriptor instead')
const FileDataUploadFileResponse$json = const {
  '1': 'FileDataUploadFileResponse',
  '2': const [
    const {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `FileDataUploadFileResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List fileDataUploadFileResponseDescriptor = $convert.base64Decode('ChpGaWxlRGF0YVVwbG9hZEZpbGVSZXNwb25zZRIWCgZyZXN1bHQYASABKAlSBnJlc3VsdA==');
@$core.Deprecated('Use fileDataDownloadFileRequestDescriptor instead')
const FileDataDownloadFileRequest$json = const {
  '1': 'FileDataDownloadFileRequest',
  '2': const [
    const {'1': 'data_id', '3': 1, '4': 1, '5': 9, '10': 'dataId'},
  ],
};

/// Descriptor for `FileDataDownloadFileRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List fileDataDownloadFileRequestDescriptor = $convert.base64Decode('ChtGaWxlRGF0YURvd25sb2FkRmlsZVJlcXVlc3QSFwoHZGF0YV9pZBgBIAEoCVIGZGF0YUlk');
@$core.Deprecated('Use fileDataDownloadFileResponseDescriptor instead')
const FileDataDownloadFileResponse$json = const {
  '1': 'FileDataDownloadFileResponse',
  '2': const [
    const {'1': 'data_id', '3': 1, '4': 1, '5': 9, '10': 'dataId'},
    const {'1': 'total_chunks', '3': 2, '4': 1, '5': 4, '10': 'totalChunks'},
    const {'1': 'current_chunk', '3': 3, '4': 1, '5': 4, '10': 'currentChunk'},
    const {'1': 'chunk', '3': 4, '4': 1, '5': 12, '10': 'chunk'},
    const {'1': 'file_name', '3': 5, '4': 1, '5': 9, '10': 'fileName'},
  ],
};

/// Descriptor for `FileDataDownloadFileResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List fileDataDownloadFileResponseDescriptor = $convert.base64Decode('ChxGaWxlRGF0YURvd25sb2FkRmlsZVJlc3BvbnNlEhcKB2RhdGFfaWQYASABKAlSBmRhdGFJZBIhCgx0b3RhbF9jaHVua3MYAiABKARSC3RvdGFsQ2h1bmtzEiMKDWN1cnJlbnRfY2h1bmsYAyABKARSDGN1cnJlbnRDaHVuaxIUCgVjaHVuaxgEIAEoDFIFY2h1bmsSGwoJZmlsZV9uYW1lGAUgASgJUghmaWxlTmFtZQ==');
