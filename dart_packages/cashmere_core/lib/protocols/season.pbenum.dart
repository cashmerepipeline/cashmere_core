//
//  Generated code. Do not modify.
//  source: season.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

class Season extends $pb.ProtobufEnum {
  static const Season Spring = Season._(0, _omitEnumNames ? '' : 'Spring');
  static const Season Summer = Season._(1, _omitEnumNames ? '' : 'Summer');
  static const Season Autumn = Season._(2, _omitEnumNames ? '' : 'Autumn');
  static const Season Winter = Season._(3, _omitEnumNames ? '' : 'Winter');

  static const $core.List<Season> values = <Season> [
    Spring,
    Summer,
    Autumn,
    Winter,
  ];

  static final $core.Map<$core.int, Season> _byValue = $pb.ProtobufEnum.initByValue(values);
  static Season? valueOf($core.int value) => _byValue[value];

  const Season._($core.int v, $core.String n) : super(v, n);
}


const _omitEnumNames = $core.bool.fromEnvironment('protobuf.omit_enum_names');
