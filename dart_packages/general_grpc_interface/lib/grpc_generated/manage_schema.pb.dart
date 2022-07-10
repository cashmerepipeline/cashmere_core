///
//  Generated code. Do not modify.
//  source: manage_schema.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

class SchemaField extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'SchemaField', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..a<$core.int>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'id', $pb.PbFieldType.O3)
    ..a<$core.List<$core.int>>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'nameMap', $pb.PbFieldType.OY)
    ..aOS(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'dataType')
    ..aOB(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'removed')
    ..hasRequiredFields = false
  ;

  SchemaField._() : super();
  factory SchemaField({
    $core.int? id,
    $core.List<$core.int>? nameMap,
    $core.String? dataType,
    $core.bool? removed,
  }) {
    final _result = create();
    if (id != null) {
      _result.id = id;
    }
    if (nameMap != null) {
      _result.nameMap = nameMap;
    }
    if (dataType != null) {
      _result.dataType = dataType;
    }
    if (removed != null) {
      _result.removed = removed;
    }
    return _result;
  }
  factory SchemaField.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory SchemaField.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  SchemaField clone() => SchemaField()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  SchemaField copyWith(void Function(SchemaField) updates) => super.copyWith((message) => updates(message as SchemaField)) as SchemaField; // ignore: deprecated_member_use
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

class GetManageSchemaRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GetManageSchemaRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'manageId')
    ..hasRequiredFields = false
  ;

  GetManageSchemaRequest._() : super();
  factory GetManageSchemaRequest({
    $core.String? manageId,
  }) {
    final _result = create();
    if (manageId != null) {
      _result.manageId = manageId;
    }
    return _result;
  }
  factory GetManageSchemaRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetManageSchemaRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetManageSchemaRequest clone() => GetManageSchemaRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetManageSchemaRequest copyWith(void Function(GetManageSchemaRequest) updates) => super.copyWith((message) => updates(message as GetManageSchemaRequest)) as GetManageSchemaRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GetManageSchemaRequest create() => GetManageSchemaRequest._();
  GetManageSchemaRequest createEmptyInstance() => create();
  static $pb.PbList<GetManageSchemaRequest> createRepeated() => $pb.PbList<GetManageSchemaRequest>();
  @$core.pragma('dart2js:noInline')
  static GetManageSchemaRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetManageSchemaRequest>(create);
  static GetManageSchemaRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get manageId => $_getSZ(0);
  @$pb.TagNumber(1)
  set manageId($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasManageId() => $_has(0);
  @$pb.TagNumber(1)
  void clearManageId() => clearField(1);
}

class GetManageSchemaResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GetManageSchemaResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..p<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'schema', $pb.PbFieldType.PY)
    ..hasRequiredFields = false
  ;

  GetManageSchemaResponse._() : super();
  factory GetManageSchemaResponse({
    $core.Iterable<$core.List<$core.int>>? schema,
  }) {
    final _result = create();
    if (schema != null) {
      _result.schema.addAll(schema);
    }
    return _result;
  }
  factory GetManageSchemaResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetManageSchemaResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetManageSchemaResponse clone() => GetManageSchemaResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetManageSchemaResponse copyWith(void Function(GetManageSchemaResponse) updates) => super.copyWith((message) => updates(message as GetManageSchemaResponse)) as GetManageSchemaResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GetManageSchemaResponse create() => GetManageSchemaResponse._();
  GetManageSchemaResponse createEmptyInstance() => create();
  static $pb.PbList<GetManageSchemaResponse> createRepeated() => $pb.PbList<GetManageSchemaResponse>();
  @$core.pragma('dart2js:noInline')
  static GetManageSchemaResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetManageSchemaResponse>(create);
  static GetManageSchemaResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.List<$core.int>> get schema => $_getList(0);
}

class NewSchemaFieldRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'NewSchemaFieldRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'manageId')
    ..aOM<SchemaField>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'field', subBuilder: SchemaField.create)
    ..hasRequiredFields = false
  ;

  NewSchemaFieldRequest._() : super();
  factory NewSchemaFieldRequest({
    $core.String? manageId,
    SchemaField? field_2,
  }) {
    final _result = create();
    if (manageId != null) {
      _result.manageId = manageId;
    }
    if (field_2 != null) {
      _result.field_2 = field_2;
    }
    return _result;
  }
  factory NewSchemaFieldRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory NewSchemaFieldRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  NewSchemaFieldRequest clone() => NewSchemaFieldRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  NewSchemaFieldRequest copyWith(void Function(NewSchemaFieldRequest) updates) => super.copyWith((message) => updates(message as NewSchemaFieldRequest)) as NewSchemaFieldRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static NewSchemaFieldRequest create() => NewSchemaFieldRequest._();
  NewSchemaFieldRequest createEmptyInstance() => create();
  static $pb.PbList<NewSchemaFieldRequest> createRepeated() => $pb.PbList<NewSchemaFieldRequest>();
  @$core.pragma('dart2js:noInline')
  static NewSchemaFieldRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<NewSchemaFieldRequest>(create);
  static NewSchemaFieldRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get manageId => $_getSZ(0);
  @$pb.TagNumber(1)
  set manageId($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasManageId() => $_has(0);
  @$pb.TagNumber(1)
  void clearManageId() => clearField(1);

  @$pb.TagNumber(2)
  SchemaField get field_2 => $_getN(1);
  @$pb.TagNumber(2)
  set field_2(SchemaField v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasField_2() => $_has(1);
  @$pb.TagNumber(2)
  void clearField_2() => clearField(2);
  @$pb.TagNumber(2)
  SchemaField ensureField_2() => $_ensure(1);
}

class NewSchemaFieldResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'NewSchemaFieldResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'result')
    ..hasRequiredFields = false
  ;

  NewSchemaFieldResponse._() : super();
  factory NewSchemaFieldResponse({
    $core.String? result,
  }) {
    final _result = create();
    if (result != null) {
      _result.result = result;
    }
    return _result;
  }
  factory NewSchemaFieldResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory NewSchemaFieldResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  NewSchemaFieldResponse clone() => NewSchemaFieldResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  NewSchemaFieldResponse copyWith(void Function(NewSchemaFieldResponse) updates) => super.copyWith((message) => updates(message as NewSchemaFieldResponse)) as NewSchemaFieldResponse; // ignore: deprecated_member_use
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

class MarkSchemaFieldRemovedRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'MarkSchemaFieldRemovedRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'manageId')
    ..a<$core.int>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'fieldId', $pb.PbFieldType.O3)
    ..hasRequiredFields = false
  ;

  MarkSchemaFieldRemovedRequest._() : super();
  factory MarkSchemaFieldRemovedRequest({
    $core.String? manageId,
    $core.int? fieldId,
  }) {
    final _result = create();
    if (manageId != null) {
      _result.manageId = manageId;
    }
    if (fieldId != null) {
      _result.fieldId = fieldId;
    }
    return _result;
  }
  factory MarkSchemaFieldRemovedRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory MarkSchemaFieldRemovedRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  MarkSchemaFieldRemovedRequest clone() => MarkSchemaFieldRemovedRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  MarkSchemaFieldRemovedRequest copyWith(void Function(MarkSchemaFieldRemovedRequest) updates) => super.copyWith((message) => updates(message as MarkSchemaFieldRemovedRequest)) as MarkSchemaFieldRemovedRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static MarkSchemaFieldRemovedRequest create() => MarkSchemaFieldRemovedRequest._();
  MarkSchemaFieldRemovedRequest createEmptyInstance() => create();
  static $pb.PbList<MarkSchemaFieldRemovedRequest> createRepeated() => $pb.PbList<MarkSchemaFieldRemovedRequest>();
  @$core.pragma('dart2js:noInline')
  static MarkSchemaFieldRemovedRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<MarkSchemaFieldRemovedRequest>(create);
  static MarkSchemaFieldRemovedRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get manageId => $_getSZ(0);
  @$pb.TagNumber(1)
  set manageId($core.String v) { $_setString(0, v); }
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
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'MarkSchemaFieldRemovedResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'result')
    ..hasRequiredFields = false
  ;

  MarkSchemaFieldRemovedResponse._() : super();
  factory MarkSchemaFieldRemovedResponse({
    $core.String? result,
  }) {
    final _result = create();
    if (result != null) {
      _result.result = result;
    }
    return _result;
  }
  factory MarkSchemaFieldRemovedResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory MarkSchemaFieldRemovedResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  MarkSchemaFieldRemovedResponse clone() => MarkSchemaFieldRemovedResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  MarkSchemaFieldRemovedResponse copyWith(void Function(MarkSchemaFieldRemovedResponse) updates) => super.copyWith((message) => updates(message as MarkSchemaFieldRemovedResponse)) as MarkSchemaFieldRemovedResponse; // ignore: deprecated_member_use
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

class EditSchemaFieldNameRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'EditSchemaFieldNameRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'manageId')
    ..a<$core.int>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'fieldId', $pb.PbFieldType.O3)
    ..aOS(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'language')
    ..aOS(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'newName')
    ..hasRequiredFields = false
  ;

  EditSchemaFieldNameRequest._() : super();
  factory EditSchemaFieldNameRequest({
    $core.String? manageId,
    $core.int? fieldId,
    $core.String? language,
    $core.String? newName,
  }) {
    final _result = create();
    if (manageId != null) {
      _result.manageId = manageId;
    }
    if (fieldId != null) {
      _result.fieldId = fieldId;
    }
    if (language != null) {
      _result.language = language;
    }
    if (newName != null) {
      _result.newName = newName;
    }
    return _result;
  }
  factory EditSchemaFieldNameRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory EditSchemaFieldNameRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  EditSchemaFieldNameRequest clone() => EditSchemaFieldNameRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  EditSchemaFieldNameRequest copyWith(void Function(EditSchemaFieldNameRequest) updates) => super.copyWith((message) => updates(message as EditSchemaFieldNameRequest)) as EditSchemaFieldNameRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static EditSchemaFieldNameRequest create() => EditSchemaFieldNameRequest._();
  EditSchemaFieldNameRequest createEmptyInstance() => create();
  static $pb.PbList<EditSchemaFieldNameRequest> createRepeated() => $pb.PbList<EditSchemaFieldNameRequest>();
  @$core.pragma('dart2js:noInline')
  static EditSchemaFieldNameRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<EditSchemaFieldNameRequest>(create);
  static EditSchemaFieldNameRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get manageId => $_getSZ(0);
  @$pb.TagNumber(1)
  set manageId($core.String v) { $_setString(0, v); }
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
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'EditSchemaFieldNameResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'result')
    ..hasRequiredFields = false
  ;

  EditSchemaFieldNameResponse._() : super();
  factory EditSchemaFieldNameResponse({
    $core.String? result,
  }) {
    final _result = create();
    if (result != null) {
      _result.result = result;
    }
    return _result;
  }
  factory EditSchemaFieldNameResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory EditSchemaFieldNameResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  EditSchemaFieldNameResponse clone() => EditSchemaFieldNameResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  EditSchemaFieldNameResponse copyWith(void Function(EditSchemaFieldNameResponse) updates) => super.copyWith((message) => updates(message as EditSchemaFieldNameResponse)) as EditSchemaFieldNameResponse; // ignore: deprecated_member_use
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

