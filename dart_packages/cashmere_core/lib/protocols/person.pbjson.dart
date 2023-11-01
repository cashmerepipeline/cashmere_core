//
//  Generated code. Do not modify.
//  source: person.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use newPersonRequestDescriptor instead')
const NewPersonRequest$json = {
  '1': 'NewPersonRequest',
  '2': [
    {'1': 'name', '3': 1, '4': 1, '5': 11, '6': '.cashmere.Name', '10': 'name'},
    {'1': 'gender', '3': 2, '4': 1, '5': 14, '6': '.cashmere.Gender', '10': 'gender'},
    {'1': 'birthday', '3': 3, '4': 1, '5': 4, '10': 'birthday'},
    {'1': 'portrait', '3': 4, '4': 1, '5': 12, '10': 'portrait'},
    {'1': 'description', '3': 5, '4': 1, '5': 9, '10': 'description'},
    {'1': 'address', '3': 6, '4': 1, '5': 9, '10': 'address'},
  ],
};

/// Descriptor for `NewPersonRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List newPersonRequestDescriptor = $convert.base64Decode(
    'ChBOZXdQZXJzb25SZXF1ZXN0EiIKBG5hbWUYASABKAsyDi5jYXNobWVyZS5OYW1lUgRuYW1lEi'
    'gKBmdlbmRlchgCIAEoDjIQLmNhc2htZXJlLkdlbmRlclIGZ2VuZGVyEhoKCGJpcnRoZGF5GAMg'
    'ASgEUghiaXJ0aGRheRIaCghwb3J0cmFpdBgEIAEoDFIIcG9ydHJhaXQSIAoLZGVzY3JpcHRpb2'
    '4YBSABKAlSC2Rlc2NyaXB0aW9uEhgKB2FkZHJlc3MYBiABKAlSB2FkZHJlc3M=');

@$core.Deprecated('Use newPersonResponseDescriptor instead')
const NewPersonResponse$json = {
  '1': 'NewPersonResponse',
  '2': [
    {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `NewPersonResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List newPersonResponseDescriptor = $convert.base64Decode(
    'ChFOZXdQZXJzb25SZXNwb25zZRIWCgZyZXN1bHQYASABKAlSBnJlc3VsdA==');

@$core.Deprecated('Use getPersonRequestDescriptor instead')
const GetPersonRequest$json = {
  '1': 'GetPersonRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 9, '10': 'id'},
  ],
};

/// Descriptor for `GetPersonRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getPersonRequestDescriptor = $convert.base64Decode(
    'ChBHZXRQZXJzb25SZXF1ZXN0Eg4KAmlkGAEgASgJUgJpZA==');

@$core.Deprecated('Use getGroupPersonsRequestDescriptor instead')
const GetGroupPersonsRequest$json = {
  '1': 'GetGroupPersonsRequest',
  '2': [
    {'1': 'group_id', '3': 1, '4': 1, '5': 9, '10': 'groupId'},
  ],
};

/// Descriptor for `GetGroupPersonsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getGroupPersonsRequestDescriptor = $convert.base64Decode(
    'ChZHZXRHcm91cFBlcnNvbnNSZXF1ZXN0EhkKCGdyb3VwX2lkGAEgASgJUgdncm91cElk');

@$core.Deprecated('Use getPersonsPageRequestDescriptor instead')
const GetPersonsPageRequest$json = {
  '1': 'GetPersonsPageRequest',
  '2': [
    {'1': 'start', '3': 1, '4': 1, '5': 5, '10': 'start'},
    {'1': 'end', '3': 2, '4': 1, '5': 5, '10': 'end'},
  ],
};

/// Descriptor for `GetPersonsPageRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getPersonsPageRequestDescriptor = $convert.base64Decode(
    'ChVHZXRQZXJzb25zUGFnZVJlcXVlc3QSFAoFc3RhcnQYASABKAVSBXN0YXJ0EhAKA2VuZBgCIA'
    'EoBVIDZW5k');

