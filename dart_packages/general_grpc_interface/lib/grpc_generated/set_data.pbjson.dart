///
//  Generated code. Do not modify.
//  source: set_data.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields,deprecated_member_use_from_same_package

import 'dart:core' as $core;
import 'dart:convert' as $convert;
import 'dart:typed_data' as $typed_data;
@$core.Deprecated('Use setDataInfoDescriptor instead')
const SetDataInfo$json = const {
  '1': 'SetDataInfo',
  '2': const [
    const {'1': 'files', '3': 1, '4': 3, '5': 9, '10': 'files'},
    const {'1': 'total_size', '3': 2, '4': 1, '5': 4, '10': 'totalSize'},
    const {'1': 'md5', '3': 3, '4': 1, '5': 9, '10': 'md5'},
  ],
};

/// Descriptor for `SetDataInfo`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List setDataInfoDescriptor = $convert.base64Decode('CgtTZXREYXRhSW5mbxIUCgVmaWxlcxgBIAMoCVIFZmlsZXMSHQoKdG90YWxfc2l6ZRgCIAEoBFIJdG90YWxTaXplEhAKA21kNRgDIAEoCVIDbWQ1');
@$core.Deprecated('Use setDataUploadSetRequestDescriptor instead')
const SetDataUploadSetRequest$json = const {
  '1': 'SetDataUploadSetRequest',
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

/// Descriptor for `SetDataUploadSetRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List setDataUploadSetRequestDescriptor = $convert.base64Decode('ChdTZXREYXRhVXBsb2FkU2V0UmVxdWVzdBIXCgdkYXRhX2lkGAEgASgJUgZkYXRhSWQSGQoIc2V0X25hbWUYAiABKAlSB3NldE5hbWUSHwoLZmlsZV9jb3VudHMYAyABKA1SCmZpbGVDb3VudHMSIQoMY3VycmVudF9maWxlGAQgASgJUgtjdXJyZW50RmlsZRIjCg10b3RhbF9jaGFuY2tzGAUgASgEUgx0b3RhbENoYW5ja3MSJQoOY3VycmVudF9jaHVuY2sYBiABKA1SDWN1cnJlbnRDaHVuY2sSFgoGY2h1bmNrGAcgASgMUgZjaHVuY2s=');
@$core.Deprecated('Use setDataUploadSetResponseDescriptor instead')
const SetDataUploadSetResponse$json = const {
  '1': 'SetDataUploadSetResponse',
  '2': const [
    const {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `SetDataUploadSetResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List setDataUploadSetResponseDescriptor = $convert.base64Decode('ChhTZXREYXRhVXBsb2FkU2V0UmVzcG9uc2USFgoGcmVzdWx0GAEgASgJUgZyZXN1bHQ=');
@$core.Deprecated('Use setDataDownloadSetRequestDescriptor instead')
const SetDataDownloadSetRequest$json = const {
  '1': 'SetDataDownloadSetRequest',
  '2': const [
    const {'1': 'data_id', '3': 1, '4': 1, '5': 9, '10': 'dataId'},
  ],
};

/// Descriptor for `SetDataDownloadSetRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List setDataDownloadSetRequestDescriptor = $convert.base64Decode('ChlTZXREYXRhRG93bmxvYWRTZXRSZXF1ZXN0EhcKB2RhdGFfaWQYASABKAlSBmRhdGFJZA==');
@$core.Deprecated('Use setDataDownloadSetResponseDescriptor instead')
const SetDataDownloadSetResponse$json = const {
  '1': 'SetDataDownloadSetResponse',
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

/// Descriptor for `SetDataDownloadSetResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List setDataDownloadSetResponseDescriptor = $convert.base64Decode('ChpTZXREYXRhRG93bmxvYWRTZXRSZXNwb25zZRIXCgdkYXRhX2lkGAEgASgJUgZkYXRhSWQSGQoIc2V0X25hbWUYAiABKAlSB3NldE5hbWUSHwoLZmlsZV9jb3VudHMYAyABKA1SCmZpbGVDb3VudHMSIQoMY3VycmVudF9maWxlGAQgASgJUgtjdXJyZW50RmlsZRIjCg10b3RhbF9jaGFuY2tzGAUgASgEUgx0b3RhbENoYW5ja3MSJQoOY3VycmVudF9jaHVuY2sYBiABKA1SDWN1cnJlbnRDaHVuY2sSFgoGY2h1bmNrGAcgASgMUgZjaHVuY2s=');
