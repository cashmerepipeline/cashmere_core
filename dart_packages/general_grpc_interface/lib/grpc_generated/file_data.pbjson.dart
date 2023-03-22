///
//  Generated code. Do not modify.
//  source: file_data.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,deprecated_member_use_from_same_package,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

import 'dart:core' as $core;
import 'dart:convert' as $convert;
import 'dart:typed_data' as $typed_data;
@$core.Deprecated('Use fileDataUploadFileRequestDescriptor instead')
const FileDataUploadFileRequest$json = const {
  '1': 'FileDataUploadFileRequest',
  '2': const [
    const {'1': 'data_id', '3': 1, '4': 1, '5': 9, '10': 'dataId'},
    const {'1': 'total_chunks', '3': 2, '4': 1, '5': 4, '10': 'totalChunks'},
    const {'1': 'current_chunk_index', '3': 3, '4': 1, '5': 4, '10': 'currentChunkIndex'},
    const {'1': 'chunk', '3': 4, '4': 1, '5': 12, '10': 'chunk'},
    const {'1': 'chunk_md5', '3': 6, '4': 1, '5': 9, '10': 'chunkMd5'},
    const {'1': 'file_info', '3': 5, '4': 1, '5': 11, '6': '.cashmere.FileInfo', '10': 'fileInfo'},
    const {'1': 'stage', '3': 8, '4': 1, '5': 9, '10': 'stage'},
  ],
};

/// Descriptor for `FileDataUploadFileRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List fileDataUploadFileRequestDescriptor = $convert.base64Decode('ChlGaWxlRGF0YVVwbG9hZEZpbGVSZXF1ZXN0EhcKB2RhdGFfaWQYASABKAlSBmRhdGFJZBIhCgx0b3RhbF9jaHVua3MYAiABKARSC3RvdGFsQ2h1bmtzEi4KE2N1cnJlbnRfY2h1bmtfaW5kZXgYAyABKARSEWN1cnJlbnRDaHVua0luZGV4EhQKBWNodW5rGAQgASgMUgVjaHVuaxIbCgljaHVua19tZDUYBiABKAlSCGNodW5rTWQ1Ei8KCWZpbGVfaW5mbxgFIAEoCzISLmNhc2htZXJlLkZpbGVJbmZvUghmaWxlSW5mbxIUCgVzdGFnZRgIIAEoCVIFc3RhZ2U=');
@$core.Deprecated('Use fileDataUploadFileResponseDescriptor instead')
const FileDataUploadFileResponse$json = const {
  '1': 'FileDataUploadFileResponse',
  '2': const [
    const {'1': 'next_chunk_index', '3': 1, '4': 1, '5': 4, '10': 'nextChunkIndex'},
  ],
};

/// Descriptor for `FileDataUploadFileResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List fileDataUploadFileResponseDescriptor = $convert.base64Decode('ChpGaWxlRGF0YVVwbG9hZEZpbGVSZXNwb25zZRIoChBuZXh0X2NodW5rX2luZGV4GAEgASgEUg5uZXh0Q2h1bmtJbmRleA==');
@$core.Deprecated('Use fileDataDownloadFileRequestDescriptor instead')
const FileDataDownloadFileRequest$json = const {
  '1': 'FileDataDownloadFileRequest',
  '2': const [
    const {'1': 'data_id', '3': 1, '4': 1, '5': 9, '10': 'dataId'},
    const {'1': 'stage', '3': 2, '4': 1, '5': 9, '10': 'stage'},
    const {'1': 'chunk_index', '3': 3, '4': 1, '5': 4, '10': 'chunkIndex'},
    const {'1': 'file_name', '3': 4, '4': 1, '5': 9, '10': 'fileName'},
  ],
};

/// Descriptor for `FileDataDownloadFileRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List fileDataDownloadFileRequestDescriptor = $convert.base64Decode('ChtGaWxlRGF0YURvd25sb2FkRmlsZVJlcXVlc3QSFwoHZGF0YV9pZBgBIAEoCVIGZGF0YUlkEhQKBXN0YWdlGAIgASgJUgVzdGFnZRIfCgtjaHVua19pbmRleBgDIAEoBFIKY2h1bmtJbmRleBIbCglmaWxlX25hbWUYBCABKAlSCGZpbGVOYW1l');
@$core.Deprecated('Use fileDataDownloadFileResponseDescriptor instead')
const FileDataDownloadFileResponse$json = const {
  '1': 'FileDataDownloadFileResponse',
  '2': const [
    const {'1': 'data_id', '3': 1, '4': 1, '5': 9, '10': 'dataId'},
    const {'1': 'total_chunks', '3': 2, '4': 1, '5': 4, '10': 'totalChunks'},
    const {'1': 'chunk_index', '3': 3, '4': 1, '5': 4, '10': 'chunkIndex'},
    const {'1': 'chunk', '3': 4, '4': 1, '5': 12, '10': 'chunk'},
    const {'1': 'chunk_md5', '3': 5, '4': 1, '5': 9, '10': 'chunkMd5'},
    const {'1': 'file_info', '3': 6, '4': 1, '5': 11, '6': '.cashmere.FileInfo', '10': 'fileInfo'},
  ],
};

/// Descriptor for `FileDataDownloadFileResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List fileDataDownloadFileResponseDescriptor = $convert.base64Decode('ChxGaWxlRGF0YURvd25sb2FkRmlsZVJlc3BvbnNlEhcKB2RhdGFfaWQYASABKAlSBmRhdGFJZBIhCgx0b3RhbF9jaHVua3MYAiABKARSC3RvdGFsQ2h1bmtzEh8KC2NodW5rX2luZGV4GAMgASgEUgpjaHVua0luZGV4EhQKBWNodW5rGAQgASgMUgVjaHVuaxIbCgljaHVua19tZDUYBSABKAlSCGNodW5rTWQ1Ei8KCWZpbGVfaW5mbxgGIAEoCzISLmNhc2htZXJlLkZpbGVJbmZvUghmaWxlSW5mbw==');
