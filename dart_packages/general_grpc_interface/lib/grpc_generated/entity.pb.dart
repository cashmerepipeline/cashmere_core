///
//  Generated code. Do not modify.
//  source: entity.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

import 'name.pb.dart' as $0;

class Entity extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'Entity', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'id')
    ..aOM<$0.Name>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'name', subBuilder: $0.Name.create)
    ..aOS(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'creatorId')
    ..aOS(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'createTimestamp')
    ..aOS(5, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'modifierId')
    ..aOS(6, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'modifyTimestamp')
    ..aOS(7, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'ownerId')
    ..pPS(8, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'groups')
    ..pPS(9, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'dataIds')
    ..pPS(10, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'commentIds')
    ..aOB(11, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'removed')
    ..pPS(12, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'removedDataIds')
    ..hasRequiredFields = false
  ;

  Entity._() : super();
  factory Entity({
    $core.String? id,
    $0.Name? name,
    $core.String? creatorId,
    $core.String? createTimestamp,
    $core.String? modifierId,
    $core.String? modifyTimestamp,
    $core.String? ownerId,
    $core.Iterable<$core.String>? groups,
    $core.Iterable<$core.String>? dataIds,
    $core.Iterable<$core.String>? commentIds,
    $core.bool? removed,
    $core.Iterable<$core.String>? removedDataIds,
  }) {
    final _result = create();
    if (id != null) {
      _result.id = id;
    }
    if (name != null) {
      _result.name = name;
    }
    if (creatorId != null) {
      _result.creatorId = creatorId;
    }
    if (createTimestamp != null) {
      _result.createTimestamp = createTimestamp;
    }
    if (modifierId != null) {
      _result.modifierId = modifierId;
    }
    if (modifyTimestamp != null) {
      _result.modifyTimestamp = modifyTimestamp;
    }
    if (ownerId != null) {
      _result.ownerId = ownerId;
    }
    if (groups != null) {
      _result.groups.addAll(groups);
    }
    if (dataIds != null) {
      _result.dataIds.addAll(dataIds);
    }
    if (commentIds != null) {
      _result.commentIds.addAll(commentIds);
    }
    if (removed != null) {
      _result.removed = removed;
    }
    if (removedDataIds != null) {
      _result.removedDataIds.addAll(removedDataIds);
    }
    return _result;
  }
  factory Entity.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Entity.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Entity clone() => Entity()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Entity copyWith(void Function(Entity) updates) => super.copyWith((message) => updates(message as Entity)) as Entity; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static Entity create() => Entity._();
  Entity createEmptyInstance() => create();
  static $pb.PbList<Entity> createRepeated() => $pb.PbList<Entity>();
  @$core.pragma('dart2js:noInline')
  static Entity getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Entity>(create);
  static Entity? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get id => $_getSZ(0);
  @$pb.TagNumber(1)
  set id($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => clearField(1);

  @$pb.TagNumber(2)
  $0.Name get name => $_getN(1);
  @$pb.TagNumber(2)
  set name($0.Name v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasName() => $_has(1);
  @$pb.TagNumber(2)
  void clearName() => clearField(2);
  @$pb.TagNumber(2)
  $0.Name ensureName() => $_ensure(1);

  @$pb.TagNumber(3)
  $core.String get creatorId => $_getSZ(2);
  @$pb.TagNumber(3)
  set creatorId($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasCreatorId() => $_has(2);
  @$pb.TagNumber(3)
  void clearCreatorId() => clearField(3);

  @$pb.TagNumber(4)
  $core.String get createTimestamp => $_getSZ(3);
  @$pb.TagNumber(4)
  set createTimestamp($core.String v) { $_setString(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasCreateTimestamp() => $_has(3);
  @$pb.TagNumber(4)
  void clearCreateTimestamp() => clearField(4);

  @$pb.TagNumber(5)
  $core.String get modifierId => $_getSZ(4);
  @$pb.TagNumber(5)
  set modifierId($core.String v) { $_setString(4, v); }
  @$pb.TagNumber(5)
  $core.bool hasModifierId() => $_has(4);
  @$pb.TagNumber(5)
  void clearModifierId() => clearField(5);

  @$pb.TagNumber(6)
  $core.String get modifyTimestamp => $_getSZ(5);
  @$pb.TagNumber(6)
  set modifyTimestamp($core.String v) { $_setString(5, v); }
  @$pb.TagNumber(6)
  $core.bool hasModifyTimestamp() => $_has(5);
  @$pb.TagNumber(6)
  void clearModifyTimestamp() => clearField(6);

  @$pb.TagNumber(7)
  $core.String get ownerId => $_getSZ(6);
  @$pb.TagNumber(7)
  set ownerId($core.String v) { $_setString(6, v); }
  @$pb.TagNumber(7)
  $core.bool hasOwnerId() => $_has(6);
  @$pb.TagNumber(7)
  void clearOwnerId() => clearField(7);

  @$pb.TagNumber(8)
  $core.List<$core.String> get groups => $_getList(7);

  @$pb.TagNumber(9)
  $core.List<$core.String> get dataIds => $_getList(8);

  @$pb.TagNumber(10)
  $core.List<$core.String> get commentIds => $_getList(9);

  @$pb.TagNumber(11)
  $core.bool get removed => $_getBF(10);
  @$pb.TagNumber(11)
  set removed($core.bool v) { $_setBool(10, v); }
  @$pb.TagNumber(11)
  $core.bool hasRemoved() => $_has(10);
  @$pb.TagNumber(11)
  void clearRemoved() => clearField(11);

  @$pb.TagNumber(12)
  $core.List<$core.String> get removedDataIds => $_getList(11);
}

class ChangeOwnerRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'ChangeOwnerRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..a<$core.int>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'manageId', $pb.PbFieldType.O3)
    ..aOS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'entityId')
    ..aOS(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'oldOwnerId')
    ..aOS(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'newOwnerId')
    ..hasRequiredFields = false
  ;

  ChangeOwnerRequest._() : super();
  factory ChangeOwnerRequest({
    $core.int? manageId,
    $core.String? entityId,
    $core.String? oldOwnerId,
    $core.String? newOwnerId,
  }) {
    final _result = create();
    if (manageId != null) {
      _result.manageId = manageId;
    }
    if (entityId != null) {
      _result.entityId = entityId;
    }
    if (oldOwnerId != null) {
      _result.oldOwnerId = oldOwnerId;
    }
    if (newOwnerId != null) {
      _result.newOwnerId = newOwnerId;
    }
    return _result;
  }
  factory ChangeOwnerRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChangeOwnerRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ChangeOwnerRequest clone() => ChangeOwnerRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ChangeOwnerRequest copyWith(void Function(ChangeOwnerRequest) updates) => super.copyWith((message) => updates(message as ChangeOwnerRequest)) as ChangeOwnerRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChangeOwnerRequest create() => ChangeOwnerRequest._();
  ChangeOwnerRequest createEmptyInstance() => create();
  static $pb.PbList<ChangeOwnerRequest> createRepeated() => $pb.PbList<ChangeOwnerRequest>();
  @$core.pragma('dart2js:noInline')
  static ChangeOwnerRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChangeOwnerRequest>(create);
  static ChangeOwnerRequest? _defaultInstance;

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
  $core.String get oldOwnerId => $_getSZ(2);
  @$pb.TagNumber(3)
  set oldOwnerId($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasOldOwnerId() => $_has(2);
  @$pb.TagNumber(3)
  void clearOldOwnerId() => clearField(3);

  @$pb.TagNumber(4)
  $core.String get newOwnerId => $_getSZ(3);
  @$pb.TagNumber(4)
  set newOwnerId($core.String v) { $_setString(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasNewOwnerId() => $_has(3);
  @$pb.TagNumber(4)
  void clearNewOwnerId() => clearField(4);
}

class ChangeOwnerResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'ChangeOwnerResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'result')
    ..hasRequiredFields = false
  ;

  ChangeOwnerResponse._() : super();
  factory ChangeOwnerResponse({
    $core.String? result,
  }) {
    final _result = create();
    if (result != null) {
      _result.result = result;
    }
    return _result;
  }
  factory ChangeOwnerResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChangeOwnerResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ChangeOwnerResponse clone() => ChangeOwnerResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ChangeOwnerResponse copyWith(void Function(ChangeOwnerResponse) updates) => super.copyWith((message) => updates(message as ChangeOwnerResponse)) as ChangeOwnerResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChangeOwnerResponse create() => ChangeOwnerResponse._();
  ChangeOwnerResponse createEmptyInstance() => create();
  static $pb.PbList<ChangeOwnerResponse> createRepeated() => $pb.PbList<ChangeOwnerResponse>();
  @$core.pragma('dart2js:noInline')
  static ChangeOwnerResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChangeOwnerResponse>(create);
  static ChangeOwnerResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get result => $_getSZ(0);
  @$pb.TagNumber(1)
  set result($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasResult() => $_has(0);
  @$pb.TagNumber(1)
  void clearResult() => clearField(1);
}

class NewEntityRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'NewEntityRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..a<$core.int>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'manageId', $pb.PbFieldType.O3)
    ..a<$core.List<$core.int>>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'data', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  NewEntityRequest._() : super();
  factory NewEntityRequest({
    $core.int? manageId,
    $core.List<$core.int>? data,
  }) {
    final _result = create();
    if (manageId != null) {
      _result.manageId = manageId;
    }
    if (data != null) {
      _result.data = data;
    }
    return _result;
  }
  factory NewEntityRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory NewEntityRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  NewEntityRequest clone() => NewEntityRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  NewEntityRequest copyWith(void Function(NewEntityRequest) updates) => super.copyWith((message) => updates(message as NewEntityRequest)) as NewEntityRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static NewEntityRequest create() => NewEntityRequest._();
  NewEntityRequest createEmptyInstance() => create();
  static $pb.PbList<NewEntityRequest> createRepeated() => $pb.PbList<NewEntityRequest>();
  @$core.pragma('dart2js:noInline')
  static NewEntityRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<NewEntityRequest>(create);
  static NewEntityRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get manageId => $_getIZ(0);
  @$pb.TagNumber(1)
  set manageId($core.int v) { $_setSignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasManageId() => $_has(0);
  @$pb.TagNumber(1)
  void clearManageId() => clearField(1);

  @$pb.TagNumber(2)
  $core.List<$core.int> get data => $_getN(1);
  @$pb.TagNumber(2)
  set data($core.List<$core.int> v) { $_setBytes(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasData() => $_has(1);
  @$pb.TagNumber(2)
  void clearData() => clearField(2);
}

class NewEntityResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'NewEntityResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'result')
    ..hasRequiredFields = false
  ;

  NewEntityResponse._() : super();
  factory NewEntityResponse({
    $core.String? result,
  }) {
    final _result = create();
    if (result != null) {
      _result.result = result;
    }
    return _result;
  }
  factory NewEntityResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory NewEntityResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  NewEntityResponse clone() => NewEntityResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  NewEntityResponse copyWith(void Function(NewEntityResponse) updates) => super.copyWith((message) => updates(message as NewEntityResponse)) as NewEntityResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static NewEntityResponse create() => NewEntityResponse._();
  NewEntityResponse createEmptyInstance() => create();
  static $pb.PbList<NewEntityResponse> createRepeated() => $pb.PbList<NewEntityResponse>();
  @$core.pragma('dart2js:noInline')
  static NewEntityResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<NewEntityResponse>(create);
  static NewEntityResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get result => $_getSZ(0);
  @$pb.TagNumber(1)
  set result($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasResult() => $_has(0);
  @$pb.TagNumber(1)
  void clearResult() => clearField(1);
}

class EditEntityRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'EditEntityRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..a<$core.int>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'manageId', $pb.PbFieldType.O3)
    ..aOS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'entityId')
    ..a<$core.List<$core.int>>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'data', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  EditEntityRequest._() : super();
  factory EditEntityRequest({
    $core.int? manageId,
    $core.String? entityId,
    $core.List<$core.int>? data,
  }) {
    final _result = create();
    if (manageId != null) {
      _result.manageId = manageId;
    }
    if (entityId != null) {
      _result.entityId = entityId;
    }
    if (data != null) {
      _result.data = data;
    }
    return _result;
  }
  factory EditEntityRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory EditEntityRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  EditEntityRequest clone() => EditEntityRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  EditEntityRequest copyWith(void Function(EditEntityRequest) updates) => super.copyWith((message) => updates(message as EditEntityRequest)) as EditEntityRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static EditEntityRequest create() => EditEntityRequest._();
  EditEntityRequest createEmptyInstance() => create();
  static $pb.PbList<EditEntityRequest> createRepeated() => $pb.PbList<EditEntityRequest>();
  @$core.pragma('dart2js:noInline')
  static EditEntityRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<EditEntityRequest>(create);
  static EditEntityRequest? _defaultInstance;

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
  $core.List<$core.int> get data => $_getN(2);
  @$pb.TagNumber(3)
  set data($core.List<$core.int> v) { $_setBytes(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasData() => $_has(2);
  @$pb.TagNumber(3)
  void clearData() => clearField(3);
}

class EditEntityResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'EditEntityResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'result')
    ..hasRequiredFields = false
  ;

  EditEntityResponse._() : super();
  factory EditEntityResponse({
    $core.String? result,
  }) {
    final _result = create();
    if (result != null) {
      _result.result = result;
    }
    return _result;
  }
  factory EditEntityResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory EditEntityResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  EditEntityResponse clone() => EditEntityResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  EditEntityResponse copyWith(void Function(EditEntityResponse) updates) => super.copyWith((message) => updates(message as EditEntityResponse)) as EditEntityResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static EditEntityResponse create() => EditEntityResponse._();
  EditEntityResponse createEmptyInstance() => create();
  static $pb.PbList<EditEntityResponse> createRepeated() => $pb.PbList<EditEntityResponse>();
  @$core.pragma('dart2js:noInline')
  static EditEntityResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<EditEntityResponse>(create);
  static EditEntityResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get result => $_getSZ(0);
  @$pb.TagNumber(1)
  set result($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasResult() => $_has(0);
  @$pb.TagNumber(1)
  void clearResult() => clearField(1);
}

class GetEntityRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GetEntityRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..a<$core.int>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'manageId', $pb.PbFieldType.O3)
    ..aOS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'entityId')
    ..hasRequiredFields = false
  ;

  GetEntityRequest._() : super();
  factory GetEntityRequest({
    $core.int? manageId,
    $core.String? entityId,
  }) {
    final _result = create();
    if (manageId != null) {
      _result.manageId = manageId;
    }
    if (entityId != null) {
      _result.entityId = entityId;
    }
    return _result;
  }
  factory GetEntityRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetEntityRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetEntityRequest clone() => GetEntityRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetEntityRequest copyWith(void Function(GetEntityRequest) updates) => super.copyWith((message) => updates(message as GetEntityRequest)) as GetEntityRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GetEntityRequest create() => GetEntityRequest._();
  GetEntityRequest createEmptyInstance() => create();
  static $pb.PbList<GetEntityRequest> createRepeated() => $pb.PbList<GetEntityRequest>();
  @$core.pragma('dart2js:noInline')
  static GetEntityRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetEntityRequest>(create);
  static GetEntityRequest? _defaultInstance;

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
}

class GetEntityResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GetEntityResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'entity', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  GetEntityResponse._() : super();
  factory GetEntityResponse({
    $core.List<$core.int>? entity,
  }) {
    final _result = create();
    if (entity != null) {
      _result.entity = entity;
    }
    return _result;
  }
  factory GetEntityResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetEntityResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetEntityResponse clone() => GetEntityResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetEntityResponse copyWith(void Function(GetEntityResponse) updates) => super.copyWith((message) => updates(message as GetEntityResponse)) as GetEntityResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GetEntityResponse create() => GetEntityResponse._();
  GetEntityResponse createEmptyInstance() => create();
  static $pb.PbList<GetEntityResponse> createRepeated() => $pb.PbList<GetEntityResponse>();
  @$core.pragma('dart2js:noInline')
  static GetEntityResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetEntityResponse>(create);
  static GetEntityResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get entity => $_getN(0);
  @$pb.TagNumber(1)
  set entity($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasEntity() => $_has(0);
  @$pb.TagNumber(1)
  void clearEntity() => clearField(1);
}

class GetEntitiesRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GetEntitiesRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..a<$core.int>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'manageId', $pb.PbFieldType.O3)
    ..pPS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'entityIds')
    ..hasRequiredFields = false
  ;

  GetEntitiesRequest._() : super();
  factory GetEntitiesRequest({
    $core.int? manageId,
    $core.Iterable<$core.String>? entityIds,
  }) {
    final _result = create();
    if (manageId != null) {
      _result.manageId = manageId;
    }
    if (entityIds != null) {
      _result.entityIds.addAll(entityIds);
    }
    return _result;
  }
  factory GetEntitiesRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetEntitiesRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetEntitiesRequest clone() => GetEntitiesRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetEntitiesRequest copyWith(void Function(GetEntitiesRequest) updates) => super.copyWith((message) => updates(message as GetEntitiesRequest)) as GetEntitiesRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GetEntitiesRequest create() => GetEntitiesRequest._();
  GetEntitiesRequest createEmptyInstance() => create();
  static $pb.PbList<GetEntitiesRequest> createRepeated() => $pb.PbList<GetEntitiesRequest>();
  @$core.pragma('dart2js:noInline')
  static GetEntitiesRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetEntitiesRequest>(create);
  static GetEntitiesRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get manageId => $_getIZ(0);
  @$pb.TagNumber(1)
  set manageId($core.int v) { $_setSignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasManageId() => $_has(0);
  @$pb.TagNumber(1)
  void clearManageId() => clearField(1);

  @$pb.TagNumber(2)
  $core.List<$core.String> get entityIds => $_getList(1);
}

class GetEntitiesResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GetEntitiesResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..p<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'entities', $pb.PbFieldType.PY)
    ..hasRequiredFields = false
  ;

  GetEntitiesResponse._() : super();
  factory GetEntitiesResponse({
    $core.Iterable<$core.List<$core.int>>? entities,
  }) {
    final _result = create();
    if (entities != null) {
      _result.entities.addAll(entities);
    }
    return _result;
  }
  factory GetEntitiesResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetEntitiesResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetEntitiesResponse clone() => GetEntitiesResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetEntitiesResponse copyWith(void Function(GetEntitiesResponse) updates) => super.copyWith((message) => updates(message as GetEntitiesResponse)) as GetEntitiesResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GetEntitiesResponse create() => GetEntitiesResponse._();
  GetEntitiesResponse createEmptyInstance() => create();
  static $pb.PbList<GetEntitiesResponse> createRepeated() => $pb.PbList<GetEntitiesResponse>();
  @$core.pragma('dart2js:noInline')
  static GetEntitiesResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetEntitiesResponse>(create);
  static GetEntitiesResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.List<$core.int>> get entities => $_getList(0);
}

class GetEntitiesPageRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GetEntitiesPageRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..a<$core.int>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'manageId', $pb.PbFieldType.O3)
    ..a<$core.int>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'pageIndex', $pb.PbFieldType.OU3)
    ..a<$core.List<$core.int>>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'conditions', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  GetEntitiesPageRequest._() : super();
  factory GetEntitiesPageRequest({
    $core.int? manageId,
    $core.int? pageIndex,
    $core.List<$core.int>? conditions,
  }) {
    final _result = create();
    if (manageId != null) {
      _result.manageId = manageId;
    }
    if (pageIndex != null) {
      _result.pageIndex = pageIndex;
    }
    if (conditions != null) {
      _result.conditions = conditions;
    }
    return _result;
  }
  factory GetEntitiesPageRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetEntitiesPageRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetEntitiesPageRequest clone() => GetEntitiesPageRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetEntitiesPageRequest copyWith(void Function(GetEntitiesPageRequest) updates) => super.copyWith((message) => updates(message as GetEntitiesPageRequest)) as GetEntitiesPageRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GetEntitiesPageRequest create() => GetEntitiesPageRequest._();
  GetEntitiesPageRequest createEmptyInstance() => create();
  static $pb.PbList<GetEntitiesPageRequest> createRepeated() => $pb.PbList<GetEntitiesPageRequest>();
  @$core.pragma('dart2js:noInline')
  static GetEntitiesPageRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetEntitiesPageRequest>(create);
  static GetEntitiesPageRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get manageId => $_getIZ(0);
  @$pb.TagNumber(1)
  set manageId($core.int v) { $_setSignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasManageId() => $_has(0);
  @$pb.TagNumber(1)
  void clearManageId() => clearField(1);

  @$pb.TagNumber(2)
  $core.int get pageIndex => $_getIZ(1);
  @$pb.TagNumber(2)
  set pageIndex($core.int v) { $_setUnsignedInt32(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasPageIndex() => $_has(1);
  @$pb.TagNumber(2)
  void clearPageIndex() => clearField(2);

  @$pb.TagNumber(3)
  $core.List<$core.int> get conditions => $_getN(2);
  @$pb.TagNumber(3)
  set conditions($core.List<$core.int> v) { $_setBytes(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasConditions() => $_has(2);
  @$pb.TagNumber(3)
  void clearConditions() => clearField(3);
}

class GetEntitiesPageResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GetEntitiesPageResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..p<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'entities', $pb.PbFieldType.PY)
    ..hasRequiredFields = false
  ;

  GetEntitiesPageResponse._() : super();
  factory GetEntitiesPageResponse({
    $core.Iterable<$core.List<$core.int>>? entities,
  }) {
    final _result = create();
    if (entities != null) {
      _result.entities.addAll(entities);
    }
    return _result;
  }
  factory GetEntitiesPageResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetEntitiesPageResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetEntitiesPageResponse clone() => GetEntitiesPageResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetEntitiesPageResponse copyWith(void Function(GetEntitiesPageResponse) updates) => super.copyWith((message) => updates(message as GetEntitiesPageResponse)) as GetEntitiesPageResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GetEntitiesPageResponse create() => GetEntitiesPageResponse._();
  GetEntitiesPageResponse createEmptyInstance() => create();
  static $pb.PbList<GetEntitiesPageResponse> createRepeated() => $pb.PbList<GetEntitiesPageResponse>();
  @$core.pragma('dart2js:noInline')
  static GetEntitiesPageResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetEntitiesPageResponse>(create);
  static GetEntitiesPageResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.List<$core.int>> get entities => $_getList(0);
}

class MarkEntityRemovedRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'MarkEntityRemovedRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'manageId')
    ..aOS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'entityId')
    ..hasRequiredFields = false
  ;

  MarkEntityRemovedRequest._() : super();
  factory MarkEntityRemovedRequest({
    $core.String? manageId,
    $core.String? entityId,
  }) {
    final _result = create();
    if (manageId != null) {
      _result.manageId = manageId;
    }
    if (entityId != null) {
      _result.entityId = entityId;
    }
    return _result;
  }
  factory MarkEntityRemovedRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory MarkEntityRemovedRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  MarkEntityRemovedRequest clone() => MarkEntityRemovedRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  MarkEntityRemovedRequest copyWith(void Function(MarkEntityRemovedRequest) updates) => super.copyWith((message) => updates(message as MarkEntityRemovedRequest)) as MarkEntityRemovedRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static MarkEntityRemovedRequest create() => MarkEntityRemovedRequest._();
  MarkEntityRemovedRequest createEmptyInstance() => create();
  static $pb.PbList<MarkEntityRemovedRequest> createRepeated() => $pb.PbList<MarkEntityRemovedRequest>();
  @$core.pragma('dart2js:noInline')
  static MarkEntityRemovedRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<MarkEntityRemovedRequest>(create);
  static MarkEntityRemovedRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get manageId => $_getSZ(0);
  @$pb.TagNumber(1)
  set manageId($core.String v) { $_setString(0, v); }
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
}

class MarkEntityRemovedResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'MarkEntityRemovedResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'result')
    ..hasRequiredFields = false
  ;

  MarkEntityRemovedResponse._() : super();
  factory MarkEntityRemovedResponse({
    $core.String? result,
  }) {
    final _result = create();
    if (result != null) {
      _result.result = result;
    }
    return _result;
  }
  factory MarkEntityRemovedResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory MarkEntityRemovedResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  MarkEntityRemovedResponse clone() => MarkEntityRemovedResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  MarkEntityRemovedResponse copyWith(void Function(MarkEntityRemovedResponse) updates) => super.copyWith((message) => updates(message as MarkEntityRemovedResponse)) as MarkEntityRemovedResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static MarkEntityRemovedResponse create() => MarkEntityRemovedResponse._();
  MarkEntityRemovedResponse createEmptyInstance() => create();
  static $pb.PbList<MarkEntityRemovedResponse> createRepeated() => $pb.PbList<MarkEntityRemovedResponse>();
  @$core.pragma('dart2js:noInline')
  static MarkEntityRemovedResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<MarkEntityRemovedResponse>(create);
  static MarkEntityRemovedResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get result => $_getSZ(0);
  @$pb.TagNumber(1)
  set result($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasResult() => $_has(0);
  @$pb.TagNumber(1)
  void clearResult() => clearField(1);
}

class GetDataListRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GetDataListRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..a<$core.int>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'manageId', $pb.PbFieldType.O3)
    ..aOS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'entityId')
    ..hasRequiredFields = false
  ;

  GetDataListRequest._() : super();
  factory GetDataListRequest({
    $core.int? manageId,
    $core.String? entityId,
  }) {
    final _result = create();
    if (manageId != null) {
      _result.manageId = manageId;
    }
    if (entityId != null) {
      _result.entityId = entityId;
    }
    return _result;
  }
  factory GetDataListRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetDataListRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetDataListRequest clone() => GetDataListRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetDataListRequest copyWith(void Function(GetDataListRequest) updates) => super.copyWith((message) => updates(message as GetDataListRequest)) as GetDataListRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GetDataListRequest create() => GetDataListRequest._();
  GetDataListRequest createEmptyInstance() => create();
  static $pb.PbList<GetDataListRequest> createRepeated() => $pb.PbList<GetDataListRequest>();
  @$core.pragma('dart2js:noInline')
  static GetDataListRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetDataListRequest>(create);
  static GetDataListRequest? _defaultInstance;

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
}

class GetDataListResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GetDataListResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..pPS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'dataIds')
    ..hasRequiredFields = false
  ;

  GetDataListResponse._() : super();
  factory GetDataListResponse({
    $core.Iterable<$core.String>? dataIds,
  }) {
    final _result = create();
    if (dataIds != null) {
      _result.dataIds.addAll(dataIds);
    }
    return _result;
  }
  factory GetDataListResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetDataListResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetDataListResponse clone() => GetDataListResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetDataListResponse copyWith(void Function(GetDataListResponse) updates) => super.copyWith((message) => updates(message as GetDataListResponse)) as GetDataListResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GetDataListResponse create() => GetDataListResponse._();
  GetDataListResponse createEmptyInstance() => create();
  static $pb.PbList<GetDataListResponse> createRepeated() => $pb.PbList<GetDataListResponse>();
  @$core.pragma('dart2js:noInline')
  static GetDataListResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetDataListResponse>(create);
  static GetDataListResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.String> get dataIds => $_getList(0);
}

class GetRemovedDataListRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GetRemovedDataListRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'manageId')
    ..aOS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'entityId')
    ..aOS(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'dataId')
    ..hasRequiredFields = false
  ;

  GetRemovedDataListRequest._() : super();
  factory GetRemovedDataListRequest({
    $core.String? manageId,
    $core.String? entityId,
    $core.String? dataId,
  }) {
    final _result = create();
    if (manageId != null) {
      _result.manageId = manageId;
    }
    if (entityId != null) {
      _result.entityId = entityId;
    }
    if (dataId != null) {
      _result.dataId = dataId;
    }
    return _result;
  }
  factory GetRemovedDataListRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetRemovedDataListRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetRemovedDataListRequest clone() => GetRemovedDataListRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetRemovedDataListRequest copyWith(void Function(GetRemovedDataListRequest) updates) => super.copyWith((message) => updates(message as GetRemovedDataListRequest)) as GetRemovedDataListRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GetRemovedDataListRequest create() => GetRemovedDataListRequest._();
  GetRemovedDataListRequest createEmptyInstance() => create();
  static $pb.PbList<GetRemovedDataListRequest> createRepeated() => $pb.PbList<GetRemovedDataListRequest>();
  @$core.pragma('dart2js:noInline')
  static GetRemovedDataListRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetRemovedDataListRequest>(create);
  static GetRemovedDataListRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get manageId => $_getSZ(0);
  @$pb.TagNumber(1)
  set manageId($core.String v) { $_setString(0, v); }
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
  $core.String get dataId => $_getSZ(2);
  @$pb.TagNumber(3)
  set dataId($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasDataId() => $_has(2);
  @$pb.TagNumber(3)
  void clearDataId() => clearField(3);
}

class GetRemovedDataListResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GetRemovedDataListResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..pPS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'dataIds')
    ..hasRequiredFields = false
  ;

  GetRemovedDataListResponse._() : super();
  factory GetRemovedDataListResponse({
    $core.Iterable<$core.String>? dataIds,
  }) {
    final _result = create();
    if (dataIds != null) {
      _result.dataIds.addAll(dataIds);
    }
    return _result;
  }
  factory GetRemovedDataListResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetRemovedDataListResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetRemovedDataListResponse clone() => GetRemovedDataListResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetRemovedDataListResponse copyWith(void Function(GetRemovedDataListResponse) updates) => super.copyWith((message) => updates(message as GetRemovedDataListResponse)) as GetRemovedDataListResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GetRemovedDataListResponse create() => GetRemovedDataListResponse._();
  GetRemovedDataListResponse createEmptyInstance() => create();
  static $pb.PbList<GetRemovedDataListResponse> createRepeated() => $pb.PbList<GetRemovedDataListResponse>();
  @$core.pragma('dart2js:noInline')
  static GetRemovedDataListResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetRemovedDataListResponse>(create);
  static GetRemovedDataListResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.String> get dataIds => $_getList(0);
}

