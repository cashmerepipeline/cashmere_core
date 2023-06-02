///
//  Generated code. Do not modify.
//  source: area.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

// ignore_for_file: UNDEFINED_SHOWN_NAME
import 'dart:core' as $core;
import 'package:protobuf/protobuf.dart' as $pb;

class AreaLevel extends $pb.ProtobufEnum {
  static const AreaLevel Country = AreaLevel._(0, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'Country');
  static const AreaLevel Province = AreaLevel._(1, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'Province');
  static const AreaLevel City = AreaLevel._(2, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'City');
  static const AreaLevel Area = AreaLevel._(3, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'Area');

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

