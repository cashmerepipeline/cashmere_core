///
//  Generated code. Do not modify.
//  source: season.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

// ignore_for_file: UNDEFINED_SHOWN_NAME
import 'dart:core' as $core;
import 'package:protobuf/protobuf.dart' as $pb;

class Season extends $pb.ProtobufEnum {
  static const Season Spring = Season._(0, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'Spring');
  static const Season Summer = Season._(1, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'Summer');
  static const Season Autumn = Season._(2, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'Autumn');
  static const Season Winter = Season._(3, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'Winter');

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

