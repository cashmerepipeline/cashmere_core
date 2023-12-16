//
//  Generated code. Do not modify.
//  source: search.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use searchRequestDescriptor instead')
const SearchRequest$json = {
  '1': 'SearchRequest',
  '2': [
    {'1': 'manage_id', '3': 1, '4': 1, '5': 9, '10': 'manageId'},
    {'1': 'search_params', '3': 2, '4': 3, '5': 11, '6': '.cashmere.SearchRequest.SearchParamsEntry', '10': 'searchParams'},
  ],
  '3': [SearchRequest_SearchParamsEntry$json],
};

@$core.Deprecated('Use searchRequestDescriptor instead')
const SearchRequest_SearchParamsEntry$json = {
  '1': 'SearchParamsEntry',
  '2': [
    {'1': 'key', '3': 1, '4': 1, '5': 9, '10': 'key'},
    {'1': 'value', '3': 2, '4': 1, '5': 9, '10': 'value'},
  ],
  '7': {'7': true},
};

/// Descriptor for `SearchRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List searchRequestDescriptor = $convert.base64Decode(
    'Cg1TZWFyY2hSZXF1ZXN0EhsKCW1hbmFnZV9pZBgBIAEoCVIIbWFuYWdlSWQSTgoNc2VhcmNoX3'
    'BhcmFtcxgCIAMoCzIpLmNhc2htZXJlLlNlYXJjaFJlcXVlc3QuU2VhcmNoUGFyYW1zRW50cnlS'
    'DHNlYXJjaFBhcmFtcxo/ChFTZWFyY2hQYXJhbXNFbnRyeRIQCgNrZXkYASABKAlSA2tleRIUCg'
    'V2YWx1ZRgCIAEoCVIFdmFsdWU6AjgB');

@$core.Deprecated('Use searchResponseDescriptor instead')
const SearchResponse$json = {
  '1': 'SearchResponse',
  '2': [
    {'1': 'results', '3': 1, '4': 3, '5': 9, '10': 'results'},
  ],
};

/// Descriptor for `SearchResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List searchResponseDescriptor = $convert.base64Decode(
    'Cg5TZWFyY2hSZXNwb25zZRIYCgdyZXN1bHRzGAEgAygJUgdyZXN1bHRz');

