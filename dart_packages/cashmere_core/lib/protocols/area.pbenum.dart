//
//  Generated code. Do not modify.
//  source: area.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

class AreaLevel extends $pb.ProtobufEnum {
  static const AreaLevel Country = AreaLevel._(0, _omitEnumNames ? '' : 'Country');
  static const AreaLevel Province = AreaLevel._(1, _omitEnumNames ? '' : 'Province');
  static const AreaLevel City = AreaLevel._(2, _omitEnumNames ? '' : 'City');
  static const AreaLevel Area = AreaLevel._(3, _omitEnumNames ? '' : 'Area');

  static const $core.List<AreaLevel> values = <AreaLevel> [
    Country,
    Province,
    City,
    Area,
  ];

  static final $core.Map<$core.int, AreaLevel> _byValue = $pb.ProtobufEnum.initByValue(values);
  static AreaLevel? valueOf($core.int value) => _byValue[value];

  const AreaLevel._($core.int v, $core.String n) : super(v, n);
}


const _omitEnumNames = $core.bool.fromEnvironment('protobuf.omit_enum_names');
