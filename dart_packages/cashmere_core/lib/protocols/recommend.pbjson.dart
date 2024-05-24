//
//  Generated code. Do not modify.
//  source: recommend.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use toggleRecommendRequestDescriptor instead')
const ToggleRecommendRequest$json = {
  '1': 'ToggleRecommendRequest',
  '2': [
    {'1': 'manage_id', '3': 1, '4': 1, '5': 9, '10': 'manageId'},
    {'1': 'entity_id', '3': 2, '4': 1, '5': 9, '10': 'entityId'},
  ],
};

/// Descriptor for `ToggleRecommendRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List toggleRecommendRequestDescriptor = $convert.base64Decode(
    'ChZUb2dnbGVSZWNvbW1lbmRSZXF1ZXN0EhsKCW1hbmFnZV9pZBgBIAEoCVIIbWFuYWdlSWQSGw'
    'oJZW50aXR5X2lkGAIgASgJUghlbnRpdHlJZA==');

@$core.Deprecated('Use toggleRecommendResponseDescriptor instead')
const ToggleRecommendResponse$json = {
  '1': 'ToggleRecommendResponse',
  '2': [
    {'1': 'result', '3': 1, '4': 1, '5': 8, '10': 'result'},
  ],
};

/// Descriptor for `ToggleRecommendResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List toggleRecommendResponseDescriptor = $convert.base64Decode(
    'ChdUb2dnbGVSZWNvbW1lbmRSZXNwb25zZRIWCgZyZXN1bHQYASABKAhSBnJlc3VsdA==');

@$core.Deprecated('Use getTopRecommendsRequestDescriptor instead')
const GetTopRecommendsRequest$json = {
  '1': 'GetTopRecommendsRequest',
  '2': [
    {'1': 'manage_id', '3': 1, '4': 1, '5': 9, '10': 'manageId'},
    {'1': 'count', '3': 3, '4': 1, '5': 5, '10': 'count'},
  ],
};

/// Descriptor for `GetTopRecommendsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getTopRecommendsRequestDescriptor = $convert.base64Decode(
    'ChdHZXRUb3BSZWNvbW1lbmRzUmVxdWVzdBIbCgltYW5hZ2VfaWQYASABKAlSCG1hbmFnZUlkEh'
    'QKBWNvdW50GAMgASgFUgVjb3VudA==');

@$core.Deprecated('Use getTopRecommendsResponseDescriptor instead')
const GetTopRecommendsResponse$json = {
  '1': 'GetTopRecommendsResponse',
  '2': [
    {'1': 'recommends', '3': 1, '4': 3, '5': 12, '10': 'recommends'},
  ],
};

/// Descriptor for `GetTopRecommendsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getTopRecommendsResponseDescriptor = $convert.base64Decode(
    'ChhHZXRUb3BSZWNvbW1lbmRzUmVzcG9uc2USHgoKcmVjb21tZW5kcxgBIAMoDFIKcmVjb21tZW'
    '5kcw==');

@$core.Deprecated('Use getRecommendAccountsRequestDescriptor instead')
const GetRecommendAccountsRequest$json = {
  '1': 'GetRecommendAccountsRequest',
  '2': [
    {'1': 'manage_id', '3': 1, '4': 1, '5': 9, '10': 'manageId'},
    {'1': 'entity_id', '3': 2, '4': 1, '5': 9, '10': 'entityId'},
  ],
};

/// Descriptor for `GetRecommendAccountsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getRecommendAccountsRequestDescriptor = $convert.base64Decode(
    'ChtHZXRSZWNvbW1lbmRBY2NvdW50c1JlcXVlc3QSGwoJbWFuYWdlX2lkGAEgASgJUghtYW5hZ2'
    'VJZBIbCgllbnRpdHlfaWQYAiABKAlSCGVudGl0eUlk');

@$core.Deprecated('Use getRecommendAccountsResponseDescriptor instead')
const GetRecommendAccountsResponse$json = {
  '1': 'GetRecommendAccountsResponse',
  '2': [
    {'1': 'accounts', '3': 1, '4': 3, '5': 9, '10': 'accounts'},
  ],
};

/// Descriptor for `GetRecommendAccountsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getRecommendAccountsResponseDescriptor = $convert.base64Decode(
    'ChxHZXRSZWNvbW1lbmRBY2NvdW50c1Jlc3BvbnNlEhoKCGFjY291bnRzGAEgAygJUghhY2NvdW'
    '50cw==');

@$core.Deprecated('Use getAccountRecommendedEntitiesRequestDescriptor instead')
const GetAccountRecommendedEntitiesRequest$json = {
  '1': 'GetAccountRecommendedEntitiesRequest',
  '2': [
    {'1': 'manage_id', '3': 1, '4': 1, '5': 9, '10': 'manageId'},
  ],
};

/// Descriptor for `GetAccountRecommendedEntitiesRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getAccountRecommendedEntitiesRequestDescriptor = $convert.base64Decode(
    'CiRHZXRBY2NvdW50UmVjb21tZW5kZWRFbnRpdGllc1JlcXVlc3QSGwoJbWFuYWdlX2lkGAEgAS'
    'gJUghtYW5hZ2VJZA==');

@$core.Deprecated('Use getAccountRecommendedEntitiesResponseDescriptor instead')
const GetAccountRecommendedEntitiesResponse$json = {
  '1': 'GetAccountRecommendedEntitiesResponse',
  '2': [
    {'1': 'entities', '3': 1, '4': 3, '5': 9, '10': 'entities'},
  ],
};

/// Descriptor for `GetAccountRecommendedEntitiesResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getAccountRecommendedEntitiesResponseDescriptor = $convert.base64Decode(
    'CiVHZXRBY2NvdW50UmVjb21tZW5kZWRFbnRpdGllc1Jlc3BvbnNlEhoKCGVudGl0aWVzGAEgAy'
    'gJUghlbnRpdGllcw==');

