///
//  Generated code. Do not modify.
//  source: comment.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields,deprecated_member_use_from_same_package

import 'dart:core' as $core;
import 'dart:convert' as $convert;
import 'dart:typed_data' as $typed_data;
@$core.Deprecated('Use newCommentRequestDescriptor instead')
const NewCommentRequest$json = const {
  '1': 'NewCommentRequest',
  '2': const [
    const {'1': 'target_manage_id', '3': 1, '4': 1, '5': 9, '10': 'targetManageId'},
    const {'1': 'target_entity_id', '3': 2, '4': 1, '5': 9, '10': 'targetEntityId'},
    const {'1': 'contents', '3': 3, '4': 1, '5': 9, '10': 'contents'},
  ],
};

/// Descriptor for `NewCommentRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List newCommentRequestDescriptor = $convert.base64Decode('ChFOZXdDb21tZW50UmVxdWVzdBIoChB0YXJnZXRfbWFuYWdlX2lkGAEgASgJUg50YXJnZXRNYW5hZ2VJZBIoChB0YXJnZXRfZW50aXR5X2lkGAIgASgJUg50YXJnZXRFbnRpdHlJZBIaCghjb250ZW50cxgDIAEoCVIIY29udGVudHM=');
@$core.Deprecated('Use newCommentResponseDescriptor instead')
const NewCommentResponse$json = const {
  '1': 'NewCommentResponse',
  '2': const [
    const {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `NewCommentResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List newCommentResponseDescriptor = $convert.base64Decode('ChJOZXdDb21tZW50UmVzcG9uc2USFgoGcmVzdWx0GAEgASgJUgZyZXN1bHQ=');
@$core.Deprecated('Use editCommentRequestDescriptor instead')
const EditCommentRequest$json = const {
  '1': 'EditCommentRequest',
  '2': const [
    const {'1': 'comment_id', '3': 1, '4': 1, '5': 9, '10': 'commentId'},
    const {'1': 'new_contents', '3': 2, '4': 1, '5': 9, '10': 'newContents'},
  ],
};

/// Descriptor for `EditCommentRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List editCommentRequestDescriptor = $convert.base64Decode('ChJFZGl0Q29tbWVudFJlcXVlc3QSHQoKY29tbWVudF9pZBgBIAEoCVIJY29tbWVudElkEiEKDG5ld19jb250ZW50cxgCIAEoCVILbmV3Q29udGVudHM=');
@$core.Deprecated('Use editCommentResponseDescriptor instead')
const EditCommentResponse$json = const {
  '1': 'EditCommentResponse',
  '2': const [
    const {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `EditCommentResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List editCommentResponseDescriptor = $convert.base64Decode('ChNFZGl0Q29tbWVudFJlc3BvbnNlEhYKBnJlc3VsdBgBIAEoCVIGcmVzdWx0');
@$core.Deprecated('Use removeCommentRequestDescriptor instead')
const RemoveCommentRequest$json = const {
  '1': 'RemoveCommentRequest',
  '2': const [
    const {'1': 'target_manage_id', '3': 1, '4': 1, '5': 9, '10': 'targetManageId'},
    const {'1': 'target_entity_id', '3': 2, '4': 1, '5': 9, '10': 'targetEntityId'},
    const {'1': 'comment_id', '3': 3, '4': 1, '5': 9, '10': 'commentId'},
  ],
};

/// Descriptor for `RemoveCommentRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List removeCommentRequestDescriptor = $convert.base64Decode('ChRSZW1vdmVDb21tZW50UmVxdWVzdBIoChB0YXJnZXRfbWFuYWdlX2lkGAEgASgJUg50YXJnZXRNYW5hZ2VJZBIoChB0YXJnZXRfZW50aXR5X2lkGAIgASgJUg50YXJnZXRFbnRpdHlJZBIdCgpjb21tZW50X2lkGAMgASgJUgljb21tZW50SWQ=');
@$core.Deprecated('Use removeCommentResponseDescriptor instead')
const RemoveCommentResponse$json = const {
  '1': 'RemoveCommentResponse',
  '2': const [
    const {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `RemoveCommentResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List removeCommentResponseDescriptor = $convert.base64Decode('ChVSZW1vdmVDb21tZW50UmVzcG9uc2USFgoGcmVzdWx0GAEgASgJUgZyZXN1bHQ=');
