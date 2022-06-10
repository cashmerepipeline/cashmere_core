///
//  Generated code. Do not modify.
//  source: manage.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

class Field extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'Field', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..a<$core.int>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'id', $pb.PbFieldType.O3)
    ..a<$core.List<$core.int>>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'name', $pb.PbFieldType.OY)
    ..aOS(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'dataType')
    ..aOB(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'removed')
    ..hasRequiredFields = false
  ;

  Field._() : super();
  factory Field({
    $core.int? id,
    $core.List<$core.int>? name,
    $core.String? dataType,
    $core.bool? removed,
  }) {
    final _result = create();
    if (id != null) {
      _result.id = id;
    }
    if (name != null) {
      _result.name = name;
    }
    if (dataType != null) {
      _result.dataType = dataType;
    }
    if (removed != null) {
      _result.removed = removed;
    }
    return _result;
  }
  factory Field.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Field.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Field clone() => Field()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Field copyWith(void Function(Field) updates) => super.copyWith((message) => updates(message as Field)) as Field; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static Field create() => Field._();
  Field createEmptyInstance() => create();
  static $pb.PbList<Field> createRepeated() => $pb.PbList<Field>();
  @$core.pragma('dart2js:noInline')
  static Field getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Field>(create);
  static Field? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get id => $_getIZ(0);
  @$pb.TagNumber(1)
  set id($core.int v) { $_setSignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => clearField(1);

  @$pb.TagNumber(2)
  $core.List<$core.int> get name => $_getN(1);
  @$pb.TagNumber(2)
  set name($core.List<$core.int> v) { $_setBytes(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasName() => $_has(1);
  @$pb.TagNumber(2)
  void clearName() => clearField(2);

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

class GetManagesRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GetManagesRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  GetManagesRequest._() : super();
  factory GetManagesRequest() => create();
  factory GetManagesRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetManagesRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetManagesRequest clone() => GetManagesRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetManagesRequest copyWith(void Function(GetManagesRequest) updates) => super.copyWith((message) => updates(message as GetManagesRequest)) as GetManagesRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GetManagesRequest create() => GetManagesRequest._();
  GetManagesRequest createEmptyInstance() => create();
  static $pb.PbList<GetManagesRequest> createRepeated() => $pb.PbList<GetManagesRequest>();
  @$core.pragma('dart2js:noInline')
  static GetManagesRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetManagesRequest>(create);
  static GetManagesRequest? _defaultInstance;
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
    ..aOM<Field>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'field', subBuilder: Field.create)
    ..hasRequiredFields = false
  ;

  NewSchemaFieldRequest._() : super();
  factory NewSchemaFieldRequest({
    $core.String? manageId,
    Field? field_2,
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
  Field get field_2 => $_getN(1);
  @$pb.TagNumber(2)
  set field_2(Field v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasField_2() => $_has(1);
  @$pb.TagNumber(2)
  void clearField_2() => clearField(2);
  @$pb.TagNumber(2)
  Field ensureField_2() => $_ensure(1);
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

class RemoveSchemaFieldRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'RemoveSchemaFieldRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'manageId')
    ..a<$core.int>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'fieldId', $pb.PbFieldType.O3)
    ..hasRequiredFields = false
  ;

  RemoveSchemaFieldRequest._() : super();
  factory RemoveSchemaFieldRequest({
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
  factory RemoveSchemaFieldRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory RemoveSchemaFieldRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  RemoveSchemaFieldRequest clone() => RemoveSchemaFieldRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  RemoveSchemaFieldRequest copyWith(void Function(RemoveSchemaFieldRequest) updates) => super.copyWith((message) => updates(message as RemoveSchemaFieldRequest)) as RemoveSchemaFieldRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static RemoveSchemaFieldRequest create() => RemoveSchemaFieldRequest._();
  RemoveSchemaFieldRequest createEmptyInstance() => create();
  static $pb.PbList<RemoveSchemaFieldRequest> createRepeated() => $pb.PbList<RemoveSchemaFieldRequest>();
  @$core.pragma('dart2js:noInline')
  static RemoveSchemaFieldRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<RemoveSchemaFieldRequest>(create);
  static RemoveSchemaFieldRequest? _defaultInstance;

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

class RemoveSchemaFieldResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'RemoveSchemaFieldResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'result')
    ..hasRequiredFields = false
  ;

  RemoveSchemaFieldResponse._() : super();
  factory RemoveSchemaFieldResponse({
    $core.String? result,
  }) {
    final _result = create();
    if (result != null) {
      _result.result = result;
    }
    return _result;
  }
  factory RemoveSchemaFieldResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory RemoveSchemaFieldResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  RemoveSchemaFieldResponse clone() => RemoveSchemaFieldResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  RemoveSchemaFieldResponse copyWith(void Function(RemoveSchemaFieldResponse) updates) => super.copyWith((message) => updates(message as RemoveSchemaFieldResponse)) as RemoveSchemaFieldResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static RemoveSchemaFieldResponse create() => RemoveSchemaFieldResponse._();
  RemoveSchemaFieldResponse createEmptyInstance() => create();
  static $pb.PbList<RemoveSchemaFieldResponse> createRepeated() => $pb.PbList<RemoveSchemaFieldResponse>();
  @$core.pragma('dart2js:noInline')
  static RemoveSchemaFieldResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<RemoveSchemaFieldResponse>(create);
  static RemoveSchemaFieldResponse? _defaultInstance;

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

class GetManageEntryCountRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GetManageEntryCountRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'manageId')
    ..hasRequiredFields = false
  ;

  GetManageEntryCountRequest._() : super();
  factory GetManageEntryCountRequest({
    $core.String? manageId,
  }) {
    final _result = create();
    if (manageId != null) {
      _result.manageId = manageId;
    }
    return _result;
  }
  factory GetManageEntryCountRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetManageEntryCountRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetManageEntryCountRequest clone() => GetManageEntryCountRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetManageEntryCountRequest copyWith(void Function(GetManageEntryCountRequest) updates) => super.copyWith((message) => updates(message as GetManageEntryCountRequest)) as GetManageEntryCountRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GetManageEntryCountRequest create() => GetManageEntryCountRequest._();
  GetManageEntryCountRequest createEmptyInstance() => create();
  static $pb.PbList<GetManageEntryCountRequest> createRepeated() => $pb.PbList<GetManageEntryCountRequest>();
  @$core.pragma('dart2js:noInline')
  static GetManageEntryCountRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetManageEntryCountRequest>(create);
  static GetManageEntryCountRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get manageId => $_getSZ(0);
  @$pb.TagNumber(1)
  set manageId($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasManageId() => $_has(0);
  @$pb.TagNumber(1)
  void clearManageId() => clearField(1);
}

class GetManageEntryCountResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GetManageEntryCountResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..a<$fixnum.Int64>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'count', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false
  ;

  GetManageEntryCountResponse._() : super();
  factory GetManageEntryCountResponse({
    $fixnum.Int64? count,
  }) {
    final _result = create();
    if (count != null) {
      _result.count = count;
    }
    return _result;
  }
  factory GetManageEntryCountResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetManageEntryCountResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetManageEntryCountResponse clone() => GetManageEntryCountResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetManageEntryCountResponse copyWith(void Function(GetManageEntryCountResponse) updates) => super.copyWith((message) => updates(message as GetManageEntryCountResponse)) as GetManageEntryCountResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GetManageEntryCountResponse create() => GetManageEntryCountResponse._();
  GetManageEntryCountResponse createEmptyInstance() => create();
  static $pb.PbList<GetManageEntryCountResponse> createRepeated() => $pb.PbList<GetManageEntryCountResponse>();
  @$core.pragma('dart2js:noInline')
  static GetManageEntryCountResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetManageEntryCountResponse>(create);
  static GetManageEntryCountResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get count => $_getI64(0);
  @$pb.TagNumber(1)
  set count($fixnum.Int64 v) { $_setInt64(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasCount() => $_has(0);
  @$pb.TagNumber(1)
  void clearCount() => clearField(1);
}

