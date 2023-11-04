//
//  Generated code. Do not modify.
//  source: entity.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

/// 编辑操作类型
class EditOperationTypeEnum extends $pb.ProtobufEnum {
  static const EditOperationTypeEnum EDIT_PRIMARY_FIELD = EditOperationTypeEnum._(0, _omitEnumNames ? '' : 'EDIT_PRIMARY_FIELD');
  static const EditOperationTypeEnum EIDT_MAP_FIELD = EditOperationTypeEnum._(1, _omitEnumNames ? '' : 'EIDT_MAP_FIELD');
  static const EditOperationTypeEnum EDIT_MAP_FIELD_REMOVE_KEY = EditOperationTypeEnum._(2, _omitEnumNames ? '' : 'EDIT_MAP_FIELD_REMOVE_KEY');
  static const EditOperationTypeEnum EDIT_ADD_TO_ARRAY_FIELD = EditOperationTypeEnum._(3, _omitEnumNames ? '' : 'EDIT_ADD_TO_ARRAY_FIELD');
  static const EditOperationTypeEnum EDIT_REMOVE_FROM_ARRAY_FIELD = EditOperationTypeEnum._(4, _omitEnumNames ? '' : 'EDIT_REMOVE_FROM_ARRAY_FIELD');

  static const $core.List<EditOperationTypeEnum> values = <EditOperationTypeEnum> [
    EDIT_PRIMARY_FIELD,
    EIDT_MAP_FIELD,
    EDIT_MAP_FIELD_REMOVE_KEY,
    EDIT_ADD_TO_ARRAY_FIELD,
    EDIT_REMOVE_FROM_ARRAY_FIELD,
  ];

  static final $core.Map<$core.int, EditOperationTypeEnum> _byValue = $pb.ProtobufEnum.initByValue(values);
  static EditOperationTypeEnum? valueOf($core.int value) => _byValue[value];

  const EditOperationTypeEnum._($core.int v, $core.String n) : super(v, n);
}


const _omitEnumNames = $core.bool.fromEnvironment('protobuf.omit_enum_names');
