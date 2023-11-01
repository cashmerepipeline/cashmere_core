//
//  Generated code. Do not modify.
//  source: calendar.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

class CalendarType extends $pb.ProtobufEnum {
  static const CalendarType Specified = CalendarType._(0, _omitEnumNames ? '' : 'Specified');
  static const CalendarType Scripted = CalendarType._(1, _omitEnumNames ? '' : 'Scripted');

  static const $core.List<CalendarType> values = <CalendarType> [
    Specified,
    Scripted,
  ];

  static final $core.Map<$core.int, CalendarType> _byValue = $pb.ProtobufEnum.initByValue(values);
  static CalendarType? valueOf($core.int value) => _byValue[value];

  const CalendarType._($core.int v, $core.String n) : super(v, n);
}


const _omitEnumNames = $core.bool.fromEnvironment('protobuf.omit_enum_names');
