//
//  Generated code. Do not modify.
//  source: name.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

/// 名
class Name extends $pb.GeneratedMessage {
  factory Name({
    $core.String? language,
    $core.String? name,
  }) {
    final $result = create();
    if (language != null) {
      $result.language = language;
    }
    if (name != null) {
      $result.name = name;
    }
    return $result;
  }
  Name._() : super();
  factory Name.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Name.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'Name', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'language')
    ..aOS(2, _omitFieldNames ? '' : 'name')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Name clone() => Name()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Name copyWith(void Function(Name) updates) => super.copyWith((message) => updates(message as Name)) as Name;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Name create() => Name._();
  Name createEmptyInstance() => create();
  static $pb.PbList<Name> createRepeated() => $pb.PbList<Name>();
  @$core.pragma('dart2js:noInline')
  static Name getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Name>(create);
  static Name? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get language => $_getSZ(0);
  @$pb.TagNumber(1)
  set language($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasLanguage() => $_has(0);
  @$pb.TagNumber(1)
  void clearLanguage() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get name => $_getSZ(1);
  @$pb.TagNumber(2)
  set name($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasName() => $_has(1);
  @$pb.TagNumber(2)
  void clearName() => clearField(2);
}

/// 名属性，语言：语言名
class NameField extends $pb.GeneratedMessage {
  factory NameField({
    $core.Map<$core.String, $core.String>? nameField,
  }) {
    final $result = create();
    if (nameField != null) {
      $result.nameField.addAll(nameField);
    }
    return $result;
  }
  NameField._() : super();
  factory NameField.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory NameField.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'NameField', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..m<$core.String, $core.String>(1, _omitFieldNames ? '' : 'nameField', entryClassName: 'NameField.NameFieldEntry', keyFieldType: $pb.PbFieldType.OS, valueFieldType: $pb.PbFieldType.OS, packageName: const $pb.PackageName('cashmere'))
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  NameField clone() => NameField()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  NameField copyWith(void Function(NameField) updates) => super.copyWith((message) => updates(message as NameField)) as NameField;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static NameField create() => NameField._();
  NameField createEmptyInstance() => create();
  static $pb.PbList<NameField> createRepeated() => $pb.PbList<NameField>();
  @$core.pragma('dart2js:noInline')
  static NameField getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<NameField>(create);
  static NameField? _defaultInstance;

  @$pb.TagNumber(1)
  $core.Map<$core.String, $core.String> get nameField => $_getMap(0);
}

/// 重命名
class RenameRequest extends $pb.GeneratedMessage {
  factory RenameRequest({
    $core.int? manageId,
    $core.String? entityId,
    Name? newName,
  }) {
    final $result = create();
    if (manageId != null) {
      $result.manageId = manageId;
    }
    if (entityId != null) {
      $result.entityId = entityId;
    }
    if (newName != null) {
      $result.newName = newName;
    }
    return $result;
  }
  RenameRequest._() : super();
  factory RenameRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory RenameRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'RenameRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..a<$core.int>(1, _omitFieldNames ? '' : 'manageId', $pb.PbFieldType.O3)
    ..aOS(2, _omitFieldNames ? '' : 'entityId')
    ..aOM<Name>(3, _omitFieldNames ? '' : 'newName', subBuilder: Name.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  RenameRequest clone() => RenameRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  RenameRequest copyWith(void Function(RenameRequest) updates) => super.copyWith((message) => updates(message as RenameRequest)) as RenameRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static RenameRequest create() => RenameRequest._();
  RenameRequest createEmptyInstance() => create();
  static $pb.PbList<RenameRequest> createRepeated() => $pb.PbList<RenameRequest>();
  @$core.pragma('dart2js:noInline')
  static RenameRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<RenameRequest>(create);
  static RenameRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get manageId => $_getIZ(0);
  @$pb.TagNumber(1)
  set manageId($core.int v) { $_setSignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasManageId() => $_has(0);
  @$pb.TagNumber(1)
  void clearManageId() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get entityId => $_getSZ(1);
  @$pb.TagNumber(2)
  set entityId($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasEntityId() => $_has(1);
  @$pb.TagNumber(2)
  void clearEntityId() => clearField(2);

  @$pb.TagNumber(3)
  Name get newName => $_getN(2);
  @$pb.TagNumber(3)
  set newName(Name v) { setField(3, v); }
  @$pb.TagNumber(3)
  $core.bool hasNewName() => $_has(2);
  @$pb.TagNumber(3)
  void clearNewName() => clearField(3);
  @$pb.TagNumber(3)
  Name ensureNewName() => $_ensure(2);
}

class RenameResponse extends $pb.GeneratedMessage {
  factory RenameResponse({
    $core.String? result,
  }) {
    final $result = create();
    if (result != null) {
      $result.result = result;
    }
    return $result;
  }
  RenameResponse._() : super();
  factory RenameResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory RenameResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'RenameResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'result')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  RenameResponse clone() => RenameResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  RenameResponse copyWith(void Function(RenameResponse) updates) => super.copyWith((message) => updates(message as RenameResponse)) as RenameResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static RenameResponse create() => RenameResponse._();
  RenameResponse createEmptyInstance() => create();
  static $pb.PbList<RenameResponse> createRepeated() => $pb.PbList<RenameResponse>();
  @$core.pragma('dart2js:noInline')
  static RenameResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<RenameResponse>(create);
  static RenameResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get result => $_getSZ(0);
  @$pb.TagNumber(1)
  set result($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasResult() => $_has(0);
  @$pb.TagNumber(1)
  void clearResult() => clearField(1);
}

/// 新语言名
class NewLanguageNameRequest extends $pb.GeneratedMessage {
  factory NewLanguageNameRequest({
    $core.int? manageId,
    $core.String? entityId,
    Name? newName,
  }) {
    final $result = create();
    if (manageId != null) {
      $result.manageId = manageId;
    }
    if (entityId != null) {
      $result.entityId = entityId;
    }
    if (newName != null) {
      $result.newName = newName;
    }
    return $result;
  }
  NewLanguageNameRequest._() : super();
  factory NewLanguageNameRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory NewLanguageNameRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'NewLanguageNameRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..a<$core.int>(1, _omitFieldNames ? '' : 'manageId', $pb.PbFieldType.O3)
    ..aOS(2, _omitFieldNames ? '' : 'entityId')
    ..aOM<Name>(3, _omitFieldNames ? '' : 'newName', subBuilder: Name.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  NewLanguageNameRequest clone() => NewLanguageNameRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  NewLanguageNameRequest copyWith(void Function(NewLanguageNameRequest) updates) => super.copyWith((message) => updates(message as NewLanguageNameRequest)) as NewLanguageNameRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static NewLanguageNameRequest create() => NewLanguageNameRequest._();
  NewLanguageNameRequest createEmptyInstance() => create();
  static $pb.PbList<NewLanguageNameRequest> createRepeated() => $pb.PbList<NewLanguageNameRequest>();
  @$core.pragma('dart2js:noInline')
  static NewLanguageNameRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<NewLanguageNameRequest>(create);
  static NewLanguageNameRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get manageId => $_getIZ(0);
  @$pb.TagNumber(1)
  set manageId($core.int v) { $_setSignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasManageId() => $_has(0);
  @$pb.TagNumber(1)
  void clearManageId() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get entityId => $_getSZ(1);
  @$pb.TagNumber(2)
  set entityId($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasEntityId() => $_has(1);
  @$pb.TagNumber(2)
  void clearEntityId() => clearField(2);

  @$pb.TagNumber(3)
  Name get newName => $_getN(2);
  @$pb.TagNumber(3)
  set newName(Name v) { setField(3, v); }
  @$pb.TagNumber(3)
  $core.bool hasNewName() => $_has(2);
  @$pb.TagNumber(3)
  void clearNewName() => clearField(3);
  @$pb.TagNumber(3)
  Name ensureNewName() => $_ensure(2);
}

class NewLanguageNameResponse extends $pb.GeneratedMessage {
  factory NewLanguageNameResponse({
    $core.String? result,
  }) {
    final $result = create();
    if (result != null) {
      $result.result = result;
    }
    return $result;
  }
  NewLanguageNameResponse._() : super();
  factory NewLanguageNameResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory NewLanguageNameResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'NewLanguageNameResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'result')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  NewLanguageNameResponse clone() => NewLanguageNameResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  NewLanguageNameResponse copyWith(void Function(NewLanguageNameResponse) updates) => super.copyWith((message) => updates(message as NewLanguageNameResponse)) as NewLanguageNameResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static NewLanguageNameResponse create() => NewLanguageNameResponse._();
  NewLanguageNameResponse createEmptyInstance() => create();
  static $pb.PbList<NewLanguageNameResponse> createRepeated() => $pb.PbList<NewLanguageNameResponse>();
  @$core.pragma('dart2js:noInline')
  static NewLanguageNameResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<NewLanguageNameResponse>(create);
  static NewLanguageNameResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get result => $_getSZ(0);
  @$pb.TagNumber(1)
  set result($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasResult() => $_has(0);
  @$pb.TagNumber(1)
  void clearResult() => clearField(1);
}

/// 移除语言名
class RemoveLanguageNameRequest extends $pb.GeneratedMessage {
  factory RemoveLanguageNameRequest({
    $core.int? manageId,
    $core.String? entityId,
    $core.String? language,
  }) {
    final $result = create();
    if (manageId != null) {
      $result.manageId = manageId;
    }
    if (entityId != null) {
      $result.entityId = entityId;
    }
    if (language != null) {
      $result.language = language;
    }
    return $result;
  }
  RemoveLanguageNameRequest._() : super();
  factory RemoveLanguageNameRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory RemoveLanguageNameRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'RemoveLanguageNameRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..a<$core.int>(1, _omitFieldNames ? '' : 'manageId', $pb.PbFieldType.O3)
    ..aOS(2, _omitFieldNames ? '' : 'entityId')
    ..aOS(3, _omitFieldNames ? '' : 'language')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  RemoveLanguageNameRequest clone() => RemoveLanguageNameRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  RemoveLanguageNameRequest copyWith(void Function(RemoveLanguageNameRequest) updates) => super.copyWith((message) => updates(message as RemoveLanguageNameRequest)) as RemoveLanguageNameRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static RemoveLanguageNameRequest create() => RemoveLanguageNameRequest._();
  RemoveLanguageNameRequest createEmptyInstance() => create();
  static $pb.PbList<RemoveLanguageNameRequest> createRepeated() => $pb.PbList<RemoveLanguageNameRequest>();
  @$core.pragma('dart2js:noInline')
  static RemoveLanguageNameRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<RemoveLanguageNameRequest>(create);
  static RemoveLanguageNameRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get manageId => $_getIZ(0);
  @$pb.TagNumber(1)
  set manageId($core.int v) { $_setSignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasManageId() => $_has(0);
  @$pb.TagNumber(1)
  void clearManageId() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get entityId => $_getSZ(1);
  @$pb.TagNumber(2)
  set entityId($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasEntityId() => $_has(1);
  @$pb.TagNumber(2)
  void clearEntityId() => clearField(2);

  @$pb.TagNumber(3)
  $core.String get language => $_getSZ(2);
  @$pb.TagNumber(3)
  set language($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasLanguage() => $_has(2);
  @$pb.TagNumber(3)
  void clearLanguage() => clearField(3);
}

class RemoveLanguageNameResponse extends $pb.GeneratedMessage {
  factory RemoveLanguageNameResponse({
    $core.String? result,
  }) {
    final $result = create();
    if (result != null) {
      $result.result = result;
    }
    return $result;
  }
  RemoveLanguageNameResponse._() : super();
  factory RemoveLanguageNameResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory RemoveLanguageNameResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'RemoveLanguageNameResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'result')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  RemoveLanguageNameResponse clone() => RemoveLanguageNameResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  RemoveLanguageNameResponse copyWith(void Function(RemoveLanguageNameResponse) updates) => super.copyWith((message) => updates(message as RemoveLanguageNameResponse)) as RemoveLanguageNameResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static RemoveLanguageNameResponse create() => RemoveLanguageNameResponse._();
  RemoveLanguageNameResponse createEmptyInstance() => create();
  static $pb.PbList<RemoveLanguageNameResponse> createRepeated() => $pb.PbList<RemoveLanguageNameResponse>();
  @$core.pragma('dart2js:noInline')
  static RemoveLanguageNameResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<RemoveLanguageNameResponse>(create);
  static RemoveLanguageNameResponse? _defaultInstance;

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
