///
//  Generated code. Do not modify.
//  source: phone_area_code.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,deprecated_member_use_from_same_package,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

import 'dart:core' as $core;
import 'dart:convert' as $convert;
import 'dart:typed_data' as $typed_data;
@$core.Deprecated('Use phoneAreaCodeDescriptor instead')
const PhoneAreaCode$json = const {
  '1': 'PhoneAreaCode',
  '2': const [
    const {'1': 'code', '3': 1, '4': 1, '5': 9, '10': 'code'},
    const {'1': 'using_areas', '3': 2, '4': 3, '5': 9, '10': 'usingAreas'},
  ],
};

/// Descriptor for `PhoneAreaCode`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List phoneAreaCodeDescriptor = $convert.base64Decode('Cg1QaG9uZUFyZWFDb2RlEhIKBGNvZGUYASABKAlSBGNvZGUSHwoLdXNpbmdfYXJlYXMYAiADKAlSCnVzaW5nQXJlYXM=');
@$core.Deprecated('Use newPhoneAreaCodeRequestDescriptor instead')
const NewPhoneAreaCodeRequest$json = const {
  '1': 'NewPhoneAreaCodeRequest',
  '2': const [
    const {'1': 'name', '3': 1, '4': 1, '5': 11, '6': '.cashmere.Name', '10': 'name'},
    const {'1': 'code', '3': 2, '4': 1, '5': 9, '10': 'code'},
    const {'1': 'areas', '3': 3, '4': 3, '5': 9, '10': 'areas'},
  ],
};

/// Descriptor for `NewPhoneAreaCodeRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List newPhoneAreaCodeRequestDescriptor = $convert.base64Decode('ChdOZXdQaG9uZUFyZWFDb2RlUmVxdWVzdBIiCgRuYW1lGAEgASgLMg4uY2FzaG1lcmUuTmFtZVIEbmFtZRISCgRjb2RlGAIgASgJUgRjb2RlEhQKBWFyZWFzGAMgAygJUgVhcmVhcw==');
@$core.Deprecated('Use newPhoneAreaCodeResponseDescriptor instead')
const NewPhoneAreaCodeResponse$json = const {
  '1': 'NewPhoneAreaCodeResponse',
  '2': const [
    const {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `NewPhoneAreaCodeResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List newPhoneAreaCodeResponseDescriptor = $convert.base64Decode('ChhOZXdQaG9uZUFyZWFDb2RlUmVzcG9uc2USFgoGcmVzdWx0GAEgASgJUgZyZXN1bHQ=');
