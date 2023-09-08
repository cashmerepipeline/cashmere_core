//
//  Generated code. Do not modify.
//  source: comment.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use newCommentRequestDescriptor instead')
const NewCommentRequest$json = {
  '1': 'NewCommentRequest',
  '2': [
    {'1': 'target_manage_id', '3': 1, '4': 1, '5': 9, '10': 'targetManageId'},
    {'1': 'target_entity_id', '3': 2, '4': 1, '5': 9, '10': 'targetEntityId'},
    {'1': 'contents', '3': 3, '4': 1, '5': 9, '10': 'contents'},
  ],
};

/// Descriptor for `NewCommentRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List newCommentRequestDescriptor = $convert.base64Decode(
    'ChFOZXdDb21tZW50UmVxdWVzdBIoChB0YXJnZXRfbWFuYWdlX2lkGAEgASgJUg50YXJnZXRNYW'
    '5hZ2VJZBIoChB0YXJnZXRfZW50aXR5X2lkGAIgASgJUg50YXJnZXRFbnRpdHlJZBIaCghjb250'
    'ZW50cxgDIAEoCVIIY29udGVudHM=');

@$core.Deprecated('Use newCommentResponseDescriptor instead')
const NewCommentResponse$json = {
  '1': 'NewCommentResponse',
  '2': [
    {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `NewCommentResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List newCommentResponseDescriptor = $convert.base64Decode(
    'ChJOZXdDb21tZW50UmVzcG9uc2USFgoGcmVzdWx0GAEgASgJUgZyZXN1bHQ=');

@$core.Deprecated('Use editCommentRequestDescriptor instead')
const EditCommentRequest$json = {
  '1': 'EditCommentRequest',
  '2': [
    {'1': 'comment_id', '3': 1, '4': 1, '5': 9, '10': 'commentId'},
    {'1': 'new_contents', '3': 2, '4': 1, '5': 9, '10': 'newContents'},
  ],
};

/// Descriptor for `EditCommentRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List editCommentRequestDescriptor = $convert.base64Decode(
    'ChJFZGl0Q29tbWVudFJlcXVlc3QSHQoKY29tbWVudF9pZBgBIAEoCVIJY29tbWVudElkEiEKDG'
    '5ld19jb250ZW50cxgCIAEoCVILbmV3Q29udGVudHM=');

@$core.Deprecated('Use editCommentResponseDescriptor instead')
const EditCommentResponse$json = {
  '1': 'EditCommentResponse',
  '2': [
    {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `EditCommentResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List editCommentResponseDescriptor = $convert.base64Decode(
    'ChNFZGl0Q29tbWVudFJlc3BvbnNlEhYKBnJlc3VsdBgBIAEoCVIGcmVzdWx0');

@$core.Deprecated('Use removeCommentRequestDescriptor instead')
const RemoveCommentRequest$json = {
  '1': 'RemoveCommentRequest',
  '2': [
    {'1': 'target_manage_id', '3': 1, '4': 1, '5': 9, '10': 'targetManageId'},
    {'1': 'target_entity_id', '3': 2, '4': 1, '5': 9, '10': 'targetEntityId'},
    {'1': 'comment_id', '3': 3, '4': 1, '5': 9, '10': 'commentId'},
  ],
};

/// Descriptor for `RemoveCommentRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List removeCommentRequestDescriptor = $convert.base64Decode(
    'ChRSZW1vdmVDb21tZW50UmVxdWVzdBIoChB0YXJnZXRfbWFuYWdlX2lkGAEgASgJUg50YXJnZX'
    'RNYW5hZ2VJZBIoChB0YXJnZXRfZW50aXR5X2lkGAIgASgJUg50YXJnZXRFbnRpdHlJZBIdCgpj'
    'b21tZW50X2lkGAMgASgJUgljb21tZW50SWQ=');

@$core.Deprecated('Use removeCommentResponseDescriptor instead')
const RemoveCommentResponse$json = {
  '1': 'RemoveCommentResponse',
  '2': [
    {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `RemoveCommentResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List removeCommentResponseDescriptor = $convert.base64Decode(
    'ChVSZW1vdmVDb21tZW50UmVzcG9uc2USFgoGcmVzdWx0GAEgASgJUgZyZXN1bHQ=');

