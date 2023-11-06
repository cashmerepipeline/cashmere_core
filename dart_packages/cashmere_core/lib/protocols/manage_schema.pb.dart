//
//  Generated code. Do not modify.
//  source: manage_schema.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

class SchemaField extends $pb.GeneratedMessage {
  factory SchemaField({
    $core.int? id,
    $core.List<$core.int>? nameMap,
    $core.String? dataType,
    $core.bool? removed,
  }) {
    final $result = create();
    if (id != null) {
      $result.id = id;
    }
    if (nameMap != null) {
      $result.nameMap = nameMap;
    }
    if (dataType != null) {
      $result.dataType = dataType;
    }
    if (removed != null) {
      $result.removed = removed;
    }
    return $result;
  }
  SchemaField._() : super();
  factory SchemaField.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory SchemaField.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'SchemaField', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..a<$core.int>(1, _omitFieldNames ? '' : 'id', $pb.PbFieldType.O3)
    ..a<$core.List<$core.int>>(2, _omitFieldNames ? '' : 'nameMap', $pb.PbFieldType.OY)
    ..aOS(3, _omitFieldNames ? '' : 'dataType')
    ..aOB(4, _omitFieldNames ? '' : 'removed')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  SchemaField clone() => SchemaField()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  SchemaField copyWith(void Function(SchemaField) updates) => super.copyWith((message) => updates(message as SchemaField)) as SchemaField;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static SchemaField create() => SchemaField._();
  SchemaField createEmptyInstance() => create();
  static $pb.PbList<SchemaField> createRepeated() => $pb.PbList<SchemaField>();
  @$core.pragma('dart2js:noInline')
  static SchemaField getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<SchemaField>(create);
  static SchemaField? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get id => $_getIZ(0);
  @$pb.TagNumber(1)
  set id($core.int v) { $_setSignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => clearField(1);

  @$pb.TagNumber(2)
  $core.List<$core.int> get nameMap => $_getN(1);
  @$pb.TagNumber(2)
  set nameMap($core.List<$core.int> v) { $_setBytes(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasNameMap() => $_has(1);
  @$pb.TagNumber(2)
  void clearNameMap() => clearField(2);

  @$pb.TagNumber(3)
  $core.String get dataType => $_getSZ(2);
  @$pb.TagNumber(3)
  set dataType($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasDataType() => $_has(2);
  @$pb.TagNumber(3)
  void clearDataType() => clearField(3);

  @$pb.TagNumber(4)
  $core.bool get removed => $_getBF(3);
  @$pb.TagNumber(4)
  set removed($core.bool v) { $_setBool(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasRemoved() => $_has(3);
  @$pb.TagNumber(4)
  void clearRemoved() => clearField(4);
}

/// 取得管理描写
class GetManageSchemaRequest extends $pb.GeneratedMessage {
  factory GetManageSchemaRequest({
    $core.int? manageId,
  }) {
    final $result = create();
    if (manageId != null) {
      $result.manageId = manageId;
    }
    return $result;
  }
  GetManageSchemaRequest._() : super();
  factory GetManageSchemaRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetManageSchemaRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'GetManageSchemaRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..a<$core.int>(1, _omitFieldNames ? '' : 'manageId', $pb.PbFieldType.O3)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetManageSchemaRequest clone() => GetManageSchemaRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetManageSchemaRequest copyWith(void Function(GetManageSchemaRequest) updates) => super.copyWith((message) => updates(message as GetManageSchemaRequest)) as GetManageSchemaRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GetManageSchemaRequest create() => GetManageSchemaRequest._();
  GetManageSchemaRequest createEmptyInstance() => create();
  static $pb.PbList<GetManageSchemaRequest> createRepeated() => $pb.PbList<GetManageSchemaRequest>();
  @$core.pragma('dart2js:noInline')
  static GetManageSchemaRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetManageSchemaRequest>(create);
  static GetManageSchemaRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get manageId => $_getIZ(0);
  @$pb.TagNumber(1)
  set manageId($core.int v) { $_setSignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasManageId() => $_has(0);
  @$pb.TagNumber(1)
  void clearManageId() => clearField(1);
}

class GetManageSchemaResponse extends $pb.GeneratedMessage {
  factory GetManageSchemaResponse({
    $core.Iterable<SchemaField>? fields,
  }) {
    final $result = create();
    if (fields != null) {
      $result.fields.addAll(fields);
    }
    return $result;
  }
  GetManageSchemaResponse._() : super();
  factory GetManageSchemaResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetManageSchemaResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'GetManageSchemaResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..pc<SchemaField>(1, _omitFieldNames ? '' : 'fields', $pb.PbFieldType.PM, subBuilder: SchemaField.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetManageSchemaResponse clone() => GetManageSchemaResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetManageSchemaResponse copyWith(void Function(GetManageSchemaResponse) updates) => super.copyWith((message) => updates(message as GetManageSchemaResponse)) as GetManageSchemaResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GetManageSchemaResponse create() => GetManageSchemaResponse._();
  GetManageSchemaResponse createEmptyInstance() => create();
  static $pb.PbList<GetManageSchemaResponse> createRepeated() => $pb.PbList<GetManageSchemaResponse>();
  @$core.pragma('dart2js:noInline')
  static GetManageSchemaResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetManageSchemaResponse>(create);
  static GetManageSchemaResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<SchemaField> get fields => $_getList(0);
}

/// 添加管理属性
class NewSchemaFieldRequest extends $pb.GeneratedMessage {
  factory NewSchemaFieldRequest({
    $core.int? manageId,
    SchemaField? newField,
  }) {
    final $result = create();
    if (manageId != null) {
      $result.manageId = manageId;
    }
    if (newField != null) {
      $result.newField = newField;
    }
    return $result;
  }
  NewSchemaFieldRequest._() : super();
  factory NewSchemaFieldRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory NewSchemaFieldRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'NewSchemaFieldRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..a<$core.int>(1, _omitFieldNames ? '' : 'manageId', $pb.PbFieldType.O3)
    ..aOM<SchemaField>(2, _omitFieldNames ? '' : 'newField', subBuilder: SchemaField.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  NewSchemaFieldRequest clone() => NewSchemaFieldRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  NewSchemaFieldRequest copyWith(void Function(NewSchemaFieldRequest) updates) => super.copyWith((message) => updates(message as NewSchemaFieldRequest)) as NewSchemaFieldRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static NewSchemaFieldRequest create() => NewSchemaFieldRequest._();
  NewSchemaFieldRequest createEmptyInstance() => create();
  static $pb.PbList<NewSchemaFieldRequest> createRepeated() => $pb.PbList<NewSchemaFieldRequest>();
  @$core.pragma('dart2js:noInline')
  static NewSchemaFieldRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<NewSchemaFieldRequest>(create);
  static NewSchemaFieldRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get manageId => $_getIZ(0);
  @$pb.TagNumber(1)
  set manageId($core.int v) { $_setSignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasManageId() => $_has(0);
  @$pb.TagNumber(1)
  void clearManageId() => clearField(1);

  @$pb.TagNumber(2)
  SchemaField get newField => $_getN(1);
  @$pb.TagNumber(2)
  set newField(SchemaField v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasNewField() => $_has(1);
  @$pb.TagNumber(2)
  void clearNewField() => clearField(2);
  @$pb.TagNumber(2)
  SchemaField ensureNewField() => $_ensure(1);
}

class NewSchemaFieldResponse extends $pb.GeneratedMessage {
  factory NewSchemaFieldResponse({
    $core.String? result,
  }) {
    final $result = create();
    if (result != null) {
      $result.result = result;
    }
    return $result;
  }
  NewSchemaFieldResponse._() : super();
  factory NewSchemaFieldResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory NewSchemaFieldResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'NewSchemaFieldResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'result')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  NewSchemaFieldResponse clone() => NewSchemaFieldResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  NewSchemaFieldResponse copyWith(void Function(NewSchemaFieldResponse) updates) => super.copyWith((message) => updates(message as NewSchemaFieldResponse)) as NewSchemaFieldResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static NewSchemaFieldResponse create() => NewSchemaFieldResponse._();
  NewSchemaFieldResponse createEmptyInstance() => create();
  static $pb.PbList<NewSchemaFieldResponse> createRepeated() => $pb.PbList<NewSchemaFieldResponse>();
  @$core.pragma('dart2js:noInline')
  static NewSchemaFieldResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<NewSchemaFieldResponse>(create);
  static NewSchemaFieldResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get result => $_getSZ(0);
  @$pb.TagNumber(1)
  set result($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasResult() => $_has(0);
  @$pb.TagNumber(1)
  void clearResult() => clearField(1);
}

/// 标记属性移除
class MarkSchemaFieldRemovedRequest extends $pb.GeneratedMessage {
  factory MarkSchemaFieldRemovedRequest({
    $core.int? manageId,
    $core.int? fieldId,
  }) {
    final $result = create();
    if (manageId != null) {
      $result.manageId = manageId;
    }
    if (fieldId != null) {
      $result.fieldId = fieldId;
    }
    return $result;
  }
  MarkSchemaFieldRemovedRequest._() : super();
  factory MarkSchemaFieldRemovedRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory MarkSchemaFieldRemovedRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'MarkSchemaFieldRemovedRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..a<$core.int>(1, _omitFieldNames ? '' : 'manageId', $pb.PbFieldType.O3)
    ..a<$core.int>(2, _omitFieldNames ? '' : 'fieldId', $pb.PbFieldType.O3)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  MarkSchemaFieldRemovedRequest clone() => MarkSchemaFieldRemovedRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  MarkSchemaFieldRemovedRequest copyWith(void Function(MarkSchemaFieldRemovedRequest) updates) => super.copyWith((message) => updates(message as MarkSchemaFieldRemovedRequest)) as MarkSchemaFieldRemovedRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static MarkSchemaFieldRemovedRequest create() => MarkSchemaFieldRemovedRequest._();
  MarkSchemaFieldRemovedRequest createEmptyInstance() => create();
  static $pb.PbList<MarkSchemaFieldRemovedRequest> createRepeated() => $pb.PbList<MarkSchemaFieldRemovedRequest>();
  @$core.pragma('dart2js:noInline')
  static MarkSchemaFieldRemovedRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<MarkSchemaFieldRemovedRequest>(create);
  static MarkSchemaFieldRemovedRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get manageId => $_getIZ(0);
  @$pb.TagNumber(1)
  set manageId($core.int v) { $_setSignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasManageId() => $_has(0);
  @$pb.TagNumber(1)
  void clearManageId() => clearField(1);

  @$pb.TagNumber(2)
  $core.int get fieldId => $_getIZ(1);
  @$pb.TagNumber(2)
  set fieldId($core.int v) { $_setSignedInt32(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasFieldId() => $_has(1);
  @$pb.TagNumber(2)
  void clearFieldId() => clearField(2);
}

class MarkSchemaFieldRemovedResponse extends $pb.GeneratedMessage {
  factory MarkSchemaFieldRemovedResponse({
    $core.String? result,
  }) {
    final $result = create();
    if (result != null) {
      $result.result = result;
    }
    return $result;
  }
  MarkSchemaFieldRemovedResponse._() : super();
  factory MarkSchemaFieldRemovedResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory MarkSchemaFieldRemovedResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'MarkSchemaFieldRemovedResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'result')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  MarkSchemaFieldRemovedResponse clone() => MarkSchemaFieldRemovedResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  MarkSchemaFieldRemovedResponse copyWith(void Function(MarkSchemaFieldRemovedResponse) updates) => super.copyWith((message) => updates(message as MarkSchemaFieldRemovedResponse)) as MarkSchemaFieldRemovedResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static MarkSchemaFieldRemovedResponse create() => MarkSchemaFieldRemovedResponse._();
  MarkSchemaFieldRemovedResponse createEmptyInstance() => create();
  static $pb.PbList<MarkSchemaFieldRemovedResponse> createRepeated() => $pb.PbList<MarkSchemaFieldRemovedResponse>();
  @$core.pragma('dart2js:noInline')
  static MarkSchemaFieldRemovedResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<MarkSchemaFieldRemovedResponse>(create);
  static MarkSchemaFieldRemovedResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get result => $_getSZ(0);
  @$pb.TagNumber(1)
  set result($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasResult() => $_has(0);
  @$pb.TagNumber(1)
  void clearResult() => clearField(1);
}

/// 编辑属性名
class EditSchemaFieldNameRequest extends $pb.GeneratedMessage {
  factory EditSchemaFieldNameRequest({
    $core.int? manageId,
    $core.int? fieldId,
    $core.String? language,
    $core.String? newName,
  }) {
    final $result = create();
    if (manageId != null) {
      $result.manageId = manageId;
    }
    if (fieldId != null) {
      $result.fieldId = fieldId;
    }
    if (language != null) {
      $result.language = language;
    }
    if (newName != null) {
      $result.newName = newName;
    }
    return $result;
  }
  EditSchemaFieldNameRequest._() : super();
  factory EditSchemaFieldNameRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory EditSchemaFieldNameRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'EditSchemaFieldNameRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..a<$core.int>(1, _omitFieldNames ? '' : 'manageId', $pb.PbFieldType.O3)
    ..a<$core.int>(2, _omitFieldNames ? '' : 'fieldId', $pb.PbFieldType.O3)
    ..aOS(3, _omitFieldNames ? '' : 'language')
    ..aOS(4, _omitFieldNames ? '' : 'newName')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  EditSchemaFieldNameRequest clone() => EditSchemaFieldNameRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  EditSchemaFieldNameRequest copyWith(void Function(EditSchemaFieldNameRequest) updates) => super.copyWith((message) => updates(message as EditSchemaFieldNameRequest)) as EditSchemaFieldNameRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static EditSchemaFieldNameRequest create() => EditSchemaFieldNameRequest._();
  EditSchemaFieldNameRequest createEmptyInstance() => create();
  static $pb.PbList<EditSchemaFieldNameRequest> createRepeated() => $pb.PbList<EditSchemaFieldNameRequest>();
  @$core.pragma('dart2js:noInline')
  static EditSchemaFieldNameRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<EditSchemaFieldNameRequest>(create);
  static EditSchemaFieldNameRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get manageId => $_getIZ(0);
  @$pb.TagNumber(1)
  set manageId($core.int v) { $_setSignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasManageId() => $_has(0);
  @$pb.TagNumber(1)
  void clearManageId() => clearField(1);

  @$pb.TagNumber(2)
  $core.int get fieldId => $_getIZ(1);
  @$pb.TagNumber(2)
  set fieldId($core.int v) { $_setSignedInt32(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasFieldId() => $_has(1);
  @$pb.TagNumber(2)
  void clearFieldId() => clearField(2);

  @$pb.TagNumber(3)
  $core.String get language => $_getSZ(2);
  @$pb.TagNumber(3)
  set language($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasLanguage() => $_has(2);
  @$pb.TagNumber(3)
  void clearLanguage() => clearField(3);

  @$pb.TagNumber(4)
  $core.String get newName => $_getSZ(3);
  @$pb.TagNumber(4)
  set newName($core.String v) { $_setString(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasNewName() => $_has(3);
  @$pb.TagNumber(4)
  void clearNewName() => clearField(4);
}

class EditSchemaFieldNameResponse extends $pb.GeneratedMessage {
  factory EditSchemaFieldNameResponse({
    $core.String? result,
  }) {
    final $result = create();
    if (result != null) {
      $result.result = result;
    }
    return $result;
  }
  EditSchemaFieldNameResponse._() : super();
  factory EditSchemaFieldNameResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory EditSchemaFieldNameResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'EditSchemaFieldNameResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'result')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  EditSchemaFieldNameResponse clone() => EditSchemaFieldNameResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  EditSchemaFieldNameResponse copyWith(void Function(EditSchemaFieldNameResponse) updates) => super.copyWith((message) => updates(message as EditSchemaFieldNameResponse)) as EditSchemaFieldNameResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static EditSchemaFieldNameResponse create() => EditSchemaFieldNameResponse._();
  EditSchemaFieldNameResponse createEmptyInstance() => create();
  static $pb.PbList<EditSchemaFieldNameResponse> createRepeated() => $pb.PbList<EditSchemaFieldNameResponse>();
  @$core.pragma('dart2js:noInline')
  static EditSchemaFieldNameResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<EditSchemaFieldNameResponse>(create);
  static EditSchemaFieldNameResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get result => $_getSZ(0);
  @$pb.TagNumber(1)
  set result($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasResult() => $_has(0);
  @$pb.TagNumber(1)
  void clearResult() => clearField(1);
}


const _omitFieldNames = $core.bool.fromEnvironment('protobuf.omit_field_names');
const _omitMessageNames = $core.bool.fromEnvironment('protobuf.omit_message_names');
