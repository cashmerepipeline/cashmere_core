///
//  Generated code. Do not modify.
//  source: data_type.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields

// ignore_for_file: UNDEFINED_SHOWN_NAME
import 'dart:core' as $core;
import 'package:protobuf/protobuf.dart' as $pb;

class DataType extends $pb.ProtobufEnum {
  static const DataType File = DataType._(0, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'File');
  static const DataType Sequence = DataType._(1, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'Sequence');
  static const DataType Set = DataType._(2, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'Set');
  static const DataType Message = DataType._(3, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'Message');
  static const DataType Document = DataType._(4, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'Document');

  static const $core.List<DataType> values = <DataType> [
    File,
    Sequence,
    Set,
    Message,
    Document,
  ];

  static final $core.Map<$core.int, DataType> _byValue = $pb.ProtobufEnum.initByValue(values);
  static DataType? valueOf($core.int value) => _byValue[value];

  const DataType._($core.int v, $core.String n) : super(v, n);
}

