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

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

import 'entity.pbenum.dart';
import 'name.pb.dart' as $0;

export 'entity.pbenum.dart';

class Entity extends $pb.GeneratedMessage {
  factory Entity({
    $core.String? id,
    $0.Name? name,
    $core.String? creatorId,
    $core.String? createTimestamp,
    $core.String? modifierId,
    $core.String? modifyTimestamp,
    $core.String? ownerId,
    $core.Iterable<$core.String>? groups,
    $core.Iterable<$core.String>? specsIds,
    $core.Iterable<$core.String>? commentIds,
    $core.bool? removed,
    $core.Iterable<$core.String>? removedDataIds,
  }) {
    final $result = create();
    if (id != null) {
      $result.id = id;
    }
    if (name != null) {
      $result.name = name;
    }
    if (creatorId != null) {
      $result.creatorId = creatorId;
    }
    if (createTimestamp != null) {
      $result.createTimestamp = createTimestamp;
    }
    if (modifierId != null) {
      $result.modifierId = modifierId;
    }
    if (modifyTimestamp != null) {
      $result.modifyTimestamp = modifyTimestamp;
    }
    if (ownerId != null) {
      $result.ownerId = ownerId;
    }
    if (groups != null) {
      $result.groups.addAll(groups);
    }
    if (specsIds != null) {
      $result.specsIds.addAll(specsIds);
    }
    if (commentIds != null) {
      $result.commentIds.addAll(commentIds);
    }
    if (removed != null) {
      $result.removed = removed;
    }
    if (removedDataIds != null) {
      $result.removedDataIds.addAll(removedDataIds);
    }
    return $result;
  }
  Entity._() : super();
  factory Entity.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Entity.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'Entity', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'id')
    ..aOM<$0.Name>(2, _omitFieldNames ? '' : 'name', subBuilder: $0.Name.create)
    ..aOS(3, _omitFieldNames ? '' : 'creatorId')
    ..aOS(4, _omitFieldNames ? '' : 'createTimestamp')
    ..aOS(5, _omitFieldNames ? '' : 'modifierId')
    ..aOS(6, _omitFieldNames ? '' : 'modifyTimestamp')
    ..aOS(7, _omitFieldNames ? '' : 'ownerId')
    ..pPS(8, _omitFieldNames ? '' : 'groups')
    ..pPS(9, _omitFieldNames ? '' : 'specsIds')
    ..pPS(10, _omitFieldNames ? '' : 'commentIds')
    ..aOB(11, _omitFieldNames ? '' : 'removed')
    ..pPS(12, _omitFieldNames ? '' : 'removedDataIds')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Entity clone() => Entity()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Entity copyWith(void Function(Entity) updates) => super.copyWith((message) => updates(message as Entity)) as Entity;

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
  $core.List<$core.String> get specsIds => $_getList(8);

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

/// 变更物主
class ChangeEntityOwnerRequest extends $pb.GeneratedMessage {
  factory ChangeEntityOwnerRequest({
    $core.String? manageId,
    $core.String? entityId,
    $core.String? oldOwnerId,
    $core.String? newOwnerId,
  }) {
    final $result = create();
    if (manageId != null) {
      $result.manageId = manageId;
    }
    if (entityId != null) {
      $result.entityId = entityId;
    }
    if (oldOwnerId != null) {
      $result.oldOwnerId = oldOwnerId;
    }
    if (newOwnerId != null) {
      $result.newOwnerId = newOwnerId;
    }
    return $result;
  }
  ChangeEntityOwnerRequest._() : super();
  factory ChangeEntityOwnerRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChangeEntityOwnerRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'ChangeEntityOwnerRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'manageId')
    ..aOS(2, _omitFieldNames ? '' : 'entityId')
    ..aOS(3, _omitFieldNames ? '' : 'oldOwnerId')
    ..aOS(4, _omitFieldNames ? '' : 'newOwnerId')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ChangeEntityOwnerRequest clone() => ChangeEntityOwnerRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ChangeEntityOwnerRequest copyWith(void Function(ChangeEntityOwnerRequest) updates) => super.copyWith((message) => updates(message as ChangeEntityOwnerRequest)) as ChangeEntityOwnerRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ChangeEntityOwnerRequest create() => ChangeEntityOwnerRequest._();
  ChangeEntityOwnerRequest createEmptyInstance() => create();
  static $pb.PbList<ChangeEntityOwnerRequest> createRepeated() => $pb.PbList<ChangeEntityOwnerRequest>();
  @$core.pragma('dart2js:noInline')
  static ChangeEntityOwnerRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChangeEntityOwnerRequest>(create);
  static ChangeEntityOwnerRequest? _defaultInstance;

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

class ChangeEntityOwnerResponse extends $pb.GeneratedMessage {
  factory ChangeEntityOwnerResponse({
    $core.String? result,
  }) {
    final $result = create();
    if (result != null) {
      $result.result = result;
    }
    return $result;
  }
  ChangeEntityOwnerResponse._() : super();
  factory ChangeEntityOwnerResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChangeEntityOwnerResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'ChangeEntityOwnerResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'result')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ChangeEntityOwnerResponse clone() => ChangeEntityOwnerResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ChangeEntityOwnerResponse copyWith(void Function(ChangeEntityOwnerResponse) updates) => super.copyWith((message) => updates(message as ChangeEntityOwnerResponse)) as ChangeEntityOwnerResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ChangeEntityOwnerResponse create() => ChangeEntityOwnerResponse._();
  ChangeEntityOwnerResponse createEmptyInstance() => create();
  static $pb.PbList<ChangeEntityOwnerResponse> createRepeated() => $pb.PbList<ChangeEntityOwnerResponse>();
  @$core.pragma('dart2js:noInline')
  static ChangeEntityOwnerResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChangeEntityOwnerResponse>(create);
  static ChangeEntityOwnerResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get result => $_getSZ(0);
  @$pb.TagNumber(1)
  set result($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasResult() => $_has(0);
  @$pb.TagNumber(1)
  void clearResult() => clearField(1);
}

/// 太通用，不建议开放
class NewEntityRequest extends $pb.GeneratedMessage {
  factory NewEntityRequest({
    $core.String? manageId,
    $core.List<$core.int>? data,
  }) {
    final $result = create();
    if (manageId != null) {
      $result.manageId = manageId;
    }
    if (data != null) {
      $result.data = data;
    }
    return $result;
  }
  NewEntityRequest._() : super();
  factory NewEntityRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory NewEntityRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'NewEntityRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'manageId')
    ..a<$core.List<$core.int>>(2, _omitFieldNames ? '' : 'data', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  NewEntityRequest clone() => NewEntityRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  NewEntityRequest copyWith(void Function(NewEntityRequest) updates) => super.copyWith((message) => updates(message as NewEntityRequest)) as NewEntityRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static NewEntityRequest create() => NewEntityRequest._();
  NewEntityRequest createEmptyInstance() => create();
  static $pb.PbList<NewEntityRequest> createRepeated() => $pb.PbList<NewEntityRequest>();
  @$core.pragma('dart2js:noInline')
  static NewEntityRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<NewEntityRequest>(create);
  static NewEntityRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get manageId => $_getSZ(0);
  @$pb.TagNumber(1)
  set manageId($core.String v) { $_setString(0, v); }
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
  factory NewEntityResponse({
    $core.String? result,
  }) {
    final $result = create();
    if (result != null) {
      $result.result = result;
    }
    return $result;
  }
  NewEntityResponse._() : super();
  factory NewEntityResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory NewEntityResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'NewEntityResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'result')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  NewEntityResponse clone() => NewEntityResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  NewEntityResponse copyWith(void Function(NewEntityResponse) updates) => super.copyWith((message) => updates(message as NewEntityResponse)) as NewEntityResponse;

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

/// 不建议开放
class EditEntityRequest extends $pb.GeneratedMessage {
  factory EditEntityRequest({
    $core.String? manageId,
    $core.String? entityId,
    $core.List<$core.int>? data,
  }) {
    final $result = create();
    if (manageId != null) {
      $result.manageId = manageId;
    }
    if (entityId != null) {
      $result.entityId = entityId;
    }
    if (data != null) {
      $result.data = data;
    }
    return $result;
  }
  EditEntityRequest._() : super();
  factory EditEntityRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory EditEntityRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'EditEntityRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'manageId')
    ..aOS(2, _omitFieldNames ? '' : 'entityId')
    ..a<$core.List<$core.int>>(3, _omitFieldNames ? '' : 'data', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  EditEntityRequest clone() => EditEntityRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  EditEntityRequest copyWith(void Function(EditEntityRequest) updates) => super.copyWith((message) => updates(message as EditEntityRequest)) as EditEntityRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static EditEntityRequest create() => EditEntityRequest._();
  EditEntityRequest createEmptyInstance() => create();
  static $pb.PbList<EditEntityRequest> createRepeated() => $pb.PbList<EditEntityRequest>();
  @$core.pragma('dart2js:noInline')
  static EditEntityRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<EditEntityRequest>(create);
  static EditEntityRequest? _defaultInstance;

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

  /// {field_id:value, ...}
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
  factory EditEntityResponse({
    $core.String? result,
  }) {
    final $result = create();
    if (result != null) {
      $result.result = result;
    }
    return $result;
  }
  EditEntityResponse._() : super();
  factory EditEntityResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory EditEntityResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'EditEntityResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'result')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  EditEntityResponse clone() => EditEntityResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  EditEntityResponse copyWith(void Function(EditEntityResponse) updates) => super.copyWith((message) => updates(message as EditEntityResponse)) as EditEntityResponse;

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

class EntityFieldEdit extends $pb.GeneratedMessage {
  factory EntityFieldEdit({
    $core.String? manageId,
    $core.String? entityId,
    $core.String? fieldId,
    EditOperationTypeEnum? operationType,
    $core.List<$core.int>? edit,
  }) {
    final $result = create();
    if (manageId != null) {
      $result.manageId = manageId;
    }
    if (entityId != null) {
      $result.entityId = entityId;
    }
    if (fieldId != null) {
      $result.fieldId = fieldId;
    }
    if (operationType != null) {
      $result.operationType = operationType;
    }
    if (edit != null) {
      $result.edit = edit;
    }
    return $result;
  }
  EntityFieldEdit._() : super();
  factory EntityFieldEdit.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory EntityFieldEdit.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'EntityFieldEdit', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'manageId')
    ..aOS(2, _omitFieldNames ? '' : 'entityId')
    ..aOS(3, _omitFieldNames ? '' : 'fieldId')
    ..e<EditOperationTypeEnum>(4, _omitFieldNames ? '' : 'operationType', $pb.PbFieldType.OE, defaultOrMaker: EditOperationTypeEnum.EDIT_PRIMARY_FIELD, valueOf: EditOperationTypeEnum.valueOf, enumValues: EditOperationTypeEnum.values)
    ..a<$core.List<$core.int>>(5, _omitFieldNames ? '' : 'edit', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  EntityFieldEdit clone() => EntityFieldEdit()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  EntityFieldEdit copyWith(void Function(EntityFieldEdit) updates) => super.copyWith((message) => updates(message as EntityFieldEdit)) as EntityFieldEdit;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static EntityFieldEdit create() => EntityFieldEdit._();
  EntityFieldEdit createEmptyInstance() => create();
  static $pb.PbList<EntityFieldEdit> createRepeated() => $pb.PbList<EntityFieldEdit>();
  @$core.pragma('dart2js:noInline')
  static EntityFieldEdit getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<EntityFieldEdit>(create);
  static EntityFieldEdit? _defaultInstance;

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
  $core.String get fieldId => $_getSZ(2);
  @$pb.TagNumber(3)
  set fieldId($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasFieldId() => $_has(2);
  @$pb.TagNumber(3)
  void clearFieldId() => clearField(3);

  @$pb.TagNumber(4)
  EditOperationTypeEnum get operationType => $_getN(3);
  @$pb.TagNumber(4)
  set operationType(EditOperationTypeEnum v) { setField(4, v); }
  @$pb.TagNumber(4)
  $core.bool hasOperationType() => $_has(3);
  @$pb.TagNumber(4)
  void clearOperationType() => clearField(4);

  /// 修改, 使用bson Document格式表示，如：{field_id:value}
  @$pb.TagNumber(5)
  $core.List<$core.int> get edit => $_getN(4);
  @$pb.TagNumber(5)
  set edit($core.List<$core.int> v) { $_setBytes(4, v); }
  @$pb.TagNumber(5)
  $core.bool hasEdit() => $_has(4);
  @$pb.TagNumber(5)
  void clearEdit() => clearField(5);
}

/// 支持多实体多属性一次提交，如果是单实体单属性编辑提交，也可以使用下面单属性编辑接口
class EditMultiEntityFieldsRequest extends $pb.GeneratedMessage {
  factory EditMultiEntityFieldsRequest({
    $core.Iterable<EntityFieldEdit>? edits,
  }) {
    final $result = create();
    if (edits != null) {
      $result.edits.addAll(edits);
    }
    return $result;
  }
  EditMultiEntityFieldsRequest._() : super();
  factory EditMultiEntityFieldsRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory EditMultiEntityFieldsRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'EditMultiEntityFieldsRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..pc<EntityFieldEdit>(1, _omitFieldNames ? '' : 'edits', $pb.PbFieldType.PM, subBuilder: EntityFieldEdit.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  EditMultiEntityFieldsRequest clone() => EditMultiEntityFieldsRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  EditMultiEntityFieldsRequest copyWith(void Function(EditMultiEntityFieldsRequest) updates) => super.copyWith((message) => updates(message as EditMultiEntityFieldsRequest)) as EditMultiEntityFieldsRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static EditMultiEntityFieldsRequest create() => EditMultiEntityFieldsRequest._();
  EditMultiEntityFieldsRequest createEmptyInstance() => create();
  static $pb.PbList<EditMultiEntityFieldsRequest> createRepeated() => $pb.PbList<EditMultiEntityFieldsRequest>();
  @$core.pragma('dart2js:noInline')
  static EditMultiEntityFieldsRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<EditMultiEntityFieldsRequest>(create);
  static EditMultiEntityFieldsRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<EntityFieldEdit> get edits => $_getList(0);
}

class EditMultiEntityFieldsResponse extends $pb.GeneratedMessage {
  factory EditMultiEntityFieldsResponse({
    $core.String? result,
  }) {
    final $result = create();
    if (result != null) {
      $result.result = result;
    }
    return $result;
  }
  EditMultiEntityFieldsResponse._() : super();
  factory EditMultiEntityFieldsResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory EditMultiEntityFieldsResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'EditMultiEntityFieldsResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'result')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  EditMultiEntityFieldsResponse clone() => EditMultiEntityFieldsResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  EditMultiEntityFieldsResponse copyWith(void Function(EditMultiEntityFieldsResponse) updates) => super.copyWith((message) => updates(message as EditMultiEntityFieldsResponse)) as EditMultiEntityFieldsResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static EditMultiEntityFieldsResponse create() => EditMultiEntityFieldsResponse._();
  EditMultiEntityFieldsResponse createEmptyInstance() => create();
  static $pb.PbList<EditMultiEntityFieldsResponse> createRepeated() => $pb.PbList<EditMultiEntityFieldsResponse>();
  @$core.pragma('dart2js:noInline')
  static EditMultiEntityFieldsResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<EditMultiEntityFieldsResponse>(create);
  static EditMultiEntityFieldsResponse? _defaultInstance;

  /// 成功返回"ok"
  @$pb.TagNumber(1)
  $core.String get result => $_getSZ(0);
  @$pb.TagNumber(1)
  set result($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasResult() => $_has(0);
  @$pb.TagNumber(1)
  void clearResult() => clearField(1);
}

/// 编辑单个实体单个字段，基础类型字段
class EditEntityFieldRequest extends $pb.GeneratedMessage {
  factory EditEntityFieldRequest({
    $core.String? manageId,
    $core.String? entityId,
    $core.String? fieldId,
    $core.List<$core.int>? newValue,
  }) {
    final $result = create();
    if (manageId != null) {
      $result.manageId = manageId;
    }
    if (entityId != null) {
      $result.entityId = entityId;
    }
    if (fieldId != null) {
      $result.fieldId = fieldId;
    }
    if (newValue != null) {
      $result.newValue = newValue;
    }
    return $result;
  }
  EditEntityFieldRequest._() : super();
  factory EditEntityFieldRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory EditEntityFieldRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'EditEntityFieldRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'manageId')
    ..aOS(2, _omitFieldNames ? '' : 'entityId')
    ..aOS(3, _omitFieldNames ? '' : 'fieldId')
    ..a<$core.List<$core.int>>(4, _omitFieldNames ? '' : 'newValue', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  EditEntityFieldRequest clone() => EditEntityFieldRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  EditEntityFieldRequest copyWith(void Function(EditEntityFieldRequest) updates) => super.copyWith((message) => updates(message as EditEntityFieldRequest)) as EditEntityFieldRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static EditEntityFieldRequest create() => EditEntityFieldRequest._();
  EditEntityFieldRequest createEmptyInstance() => create();
  static $pb.PbList<EditEntityFieldRequest> createRepeated() => $pb.PbList<EditEntityFieldRequest>();
  @$core.pragma('dart2js:noInline')
  static EditEntityFieldRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<EditEntityFieldRequest>(create);
  static EditEntityFieldRequest? _defaultInstance;

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
  $core.String get fieldId => $_getSZ(2);
  @$pb.TagNumber(3)
  set fieldId($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasFieldId() => $_has(2);
  @$pb.TagNumber(3)
  void clearFieldId() => clearField(3);

  /// {field_id:value}
  @$pb.TagNumber(4)
  $core.List<$core.int> get newValue => $_getN(3);
  @$pb.TagNumber(4)
  set newValue($core.List<$core.int> v) { $_setBytes(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasNewValue() => $_has(3);
  @$pb.TagNumber(4)
  void clearNewValue() => clearField(4);
}

class EditEntityFieldResponse extends $pb.GeneratedMessage {
  factory EditEntityFieldResponse({
    $core.String? result,
  }) {
    final $result = create();
    if (result != null) {
      $result.result = result;
    }
    return $result;
  }
  EditEntityFieldResponse._() : super();
  factory EditEntityFieldResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory EditEntityFieldResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'EditEntityFieldResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'result')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  EditEntityFieldResponse clone() => EditEntityFieldResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  EditEntityFieldResponse copyWith(void Function(EditEntityFieldResponse) updates) => super.copyWith((message) => updates(message as EditEntityFieldResponse)) as EditEntityFieldResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static EditEntityFieldResponse create() => EditEntityFieldResponse._();
  EditEntityFieldResponse createEmptyInstance() => create();
  static $pb.PbList<EditEntityFieldResponse> createRepeated() => $pb.PbList<EditEntityFieldResponse>();
  @$core.pragma('dart2js:noInline')
  static EditEntityFieldResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<EditEntityFieldResponse>(create);
  static EditEntityFieldResponse? _defaultInstance;

  /// 成功返回新值
  @$pb.TagNumber(1)
  $core.String get result => $_getSZ(0);
  @$pb.TagNumber(1)
  set result($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasResult() => $_has(0);
  @$pb.TagNumber(1)
  void clearResult() => clearField(1);
}

/// 编辑单个实体MAP字段中的某个属性
class EditEntityMapFieldRequest extends $pb.GeneratedMessage {
  factory EditEntityMapFieldRequest({
    $core.String? manageId,
    $core.String? entityId,
    $core.String? fieldId,
    $core.String? key,
    $core.List<$core.int>? newValue,
  }) {
    final $result = create();
    if (manageId != null) {
      $result.manageId = manageId;
    }
    if (entityId != null) {
      $result.entityId = entityId;
    }
    if (fieldId != null) {
      $result.fieldId = fieldId;
    }
    if (key != null) {
      $result.key = key;
    }
    if (newValue != null) {
      $result.newValue = newValue;
    }
    return $result;
  }
  EditEntityMapFieldRequest._() : super();
  factory EditEntityMapFieldRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory EditEntityMapFieldRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'EditEntityMapFieldRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'manageId')
    ..aOS(2, _omitFieldNames ? '' : 'entityId')
    ..aOS(3, _omitFieldNames ? '' : 'fieldId')
    ..aOS(4, _omitFieldNames ? '' : 'key')
    ..a<$core.List<$core.int>>(5, _omitFieldNames ? '' : 'newValue', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  EditEntityMapFieldRequest clone() => EditEntityMapFieldRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  EditEntityMapFieldRequest copyWith(void Function(EditEntityMapFieldRequest) updates) => super.copyWith((message) => updates(message as EditEntityMapFieldRequest)) as EditEntityMapFieldRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static EditEntityMapFieldRequest create() => EditEntityMapFieldRequest._();
  EditEntityMapFieldRequest createEmptyInstance() => create();
  static $pb.PbList<EditEntityMapFieldRequest> createRepeated() => $pb.PbList<EditEntityMapFieldRequest>();
  @$core.pragma('dart2js:noInline')
  static EditEntityMapFieldRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<EditEntityMapFieldRequest>(create);
  static EditEntityMapFieldRequest? _defaultInstance;

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
  $core.String get fieldId => $_getSZ(2);
  @$pb.TagNumber(3)
  set fieldId($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasFieldId() => $_has(2);
  @$pb.TagNumber(3)
  void clearFieldId() => clearField(3);

  @$pb.TagNumber(4)
  $core.String get key => $_getSZ(3);
  @$pb.TagNumber(4)
  set key($core.String v) { $_setString(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasKey() => $_has(3);
  @$pb.TagNumber(4)
  void clearKey() => clearField(4);

  /// {key:value}
  @$pb.TagNumber(5)
  $core.List<$core.int> get newValue => $_getN(4);
  @$pb.TagNumber(5)
  set newValue($core.List<$core.int> v) { $_setBytes(4, v); }
  @$pb.TagNumber(5)
  $core.bool hasNewValue() => $_has(4);
  @$pb.TagNumber(5)
  void clearNewValue() => clearField(5);
}

class EditEntityMapFieldResponse extends $pb.GeneratedMessage {
  factory EditEntityMapFieldResponse({
    $core.String? result,
  }) {
    final $result = create();
    if (result != null) {
      $result.result = result;
    }
    return $result;
  }
  EditEntityMapFieldResponse._() : super();
  factory EditEntityMapFieldResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory EditEntityMapFieldResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'EditEntityMapFieldResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'result')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  EditEntityMapFieldResponse clone() => EditEntityMapFieldResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  EditEntityMapFieldResponse copyWith(void Function(EditEntityMapFieldResponse) updates) => super.copyWith((message) => updates(message as EditEntityMapFieldResponse)) as EditEntityMapFieldResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static EditEntityMapFieldResponse create() => EditEntityMapFieldResponse._();
  EditEntityMapFieldResponse createEmptyInstance() => create();
  static $pb.PbList<EditEntityMapFieldResponse> createRepeated() => $pb.PbList<EditEntityMapFieldResponse>();
  @$core.pragma('dart2js:noInline')
  static EditEntityMapFieldResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<EditEntityMapFieldResponse>(create);
  static EditEntityMapFieldResponse? _defaultInstance;

  /// 成功返回新值
  @$pb.TagNumber(1)
  $core.String get result => $_getSZ(0);
  @$pb.TagNumber(1)
  set result($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasResult() => $_has(0);
  @$pb.TagNumber(1)
  void clearResult() => clearField(1);
}

/// 修改单个实体MAP移除某个key
class EditEntityMapFieldRemoveKeyRequest extends $pb.GeneratedMessage {
  factory EditEntityMapFieldRemoveKeyRequest({
    $core.String? manageId,
    $core.String? entityId,
    $core.String? fieldId,
    $core.String? key,
  }) {
    final $result = create();
    if (manageId != null) {
      $result.manageId = manageId;
    }
    if (entityId != null) {
      $result.entityId = entityId;
    }
    if (fieldId != null) {
      $result.fieldId = fieldId;
    }
    if (key != null) {
      $result.key = key;
    }
    return $result;
  }
  EditEntityMapFieldRemoveKeyRequest._() : super();
  factory EditEntityMapFieldRemoveKeyRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory EditEntityMapFieldRemoveKeyRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'EditEntityMapFieldRemoveKeyRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'manageId')
    ..aOS(2, _omitFieldNames ? '' : 'entityId')
    ..aOS(3, _omitFieldNames ? '' : 'fieldId')
    ..aOS(4, _omitFieldNames ? '' : 'key')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  EditEntityMapFieldRemoveKeyRequest clone() => EditEntityMapFieldRemoveKeyRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  EditEntityMapFieldRemoveKeyRequest copyWith(void Function(EditEntityMapFieldRemoveKeyRequest) updates) => super.copyWith((message) => updates(message as EditEntityMapFieldRemoveKeyRequest)) as EditEntityMapFieldRemoveKeyRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static EditEntityMapFieldRemoveKeyRequest create() => EditEntityMapFieldRemoveKeyRequest._();
  EditEntityMapFieldRemoveKeyRequest createEmptyInstance() => create();
  static $pb.PbList<EditEntityMapFieldRemoveKeyRequest> createRepeated() => $pb.PbList<EditEntityMapFieldRemoveKeyRequest>();
  @$core.pragma('dart2js:noInline')
  static EditEntityMapFieldRemoveKeyRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<EditEntityMapFieldRemoveKeyRequest>(create);
  static EditEntityMapFieldRemoveKeyRequest? _defaultInstance;

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
  $core.String get fieldId => $_getSZ(2);
  @$pb.TagNumber(3)
  set fieldId($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasFieldId() => $_has(2);
  @$pb.TagNumber(3)
  void clearFieldId() => clearField(3);

  @$pb.TagNumber(4)
  $core.String get key => $_getSZ(3);
  @$pb.TagNumber(4)
  set key($core.String v) { $_setString(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasKey() => $_has(3);
  @$pb.TagNumber(4)
  void clearKey() => clearField(4);
}

class EditEntityMapFieldRemoveKeyResponse extends $pb.GeneratedMessage {
  factory EditEntityMapFieldRemoveKeyResponse({
    $core.String? result,
  }) {
    final $result = create();
    if (result != null) {
      $result.result = result;
    }
    return $result;
  }
  EditEntityMapFieldRemoveKeyResponse._() : super();
  factory EditEntityMapFieldRemoveKeyResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory EditEntityMapFieldRemoveKeyResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'EditEntityMapFieldRemoveKeyResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'result')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  EditEntityMapFieldRemoveKeyResponse clone() => EditEntityMapFieldRemoveKeyResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  EditEntityMapFieldRemoveKeyResponse copyWith(void Function(EditEntityMapFieldRemoveKeyResponse) updates) => super.copyWith((message) => updates(message as EditEntityMapFieldRemoveKeyResponse)) as EditEntityMapFieldRemoveKeyResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static EditEntityMapFieldRemoveKeyResponse create() => EditEntityMapFieldRemoveKeyResponse._();
  EditEntityMapFieldRemoveKeyResponse createEmptyInstance() => create();
  static $pb.PbList<EditEntityMapFieldRemoveKeyResponse> createRepeated() => $pb.PbList<EditEntityMapFieldRemoveKeyResponse>();
  @$core.pragma('dart2js:noInline')
  static EditEntityMapFieldRemoveKeyResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<EditEntityMapFieldRemoveKeyResponse>(create);
  static EditEntityMapFieldRemoveKeyResponse? _defaultInstance;

  /// 成功返回key
  @$pb.TagNumber(1)
  $core.String get result => $_getSZ(0);
  @$pb.TagNumber(1)
  set result($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasResult() => $_has(0);
  @$pb.TagNumber(1)
  void clearResult() => clearField(1);
}

/// 修改单个实体List实体属性, 添加成员
class EditEntityArrayFieldAddItemsRequest extends $pb.GeneratedMessage {
  factory EditEntityArrayFieldAddItemsRequest({
    $core.String? manageId,
    $core.String? entityId,
    $core.String? fieldId,
    $core.List<$core.int>? items,
  }) {
    final $result = create();
    if (manageId != null) {
      $result.manageId = manageId;
    }
    if (entityId != null) {
      $result.entityId = entityId;
    }
    if (fieldId != null) {
      $result.fieldId = fieldId;
    }
    if (items != null) {
      $result.items = items;
    }
    return $result;
  }
  EditEntityArrayFieldAddItemsRequest._() : super();
  factory EditEntityArrayFieldAddItemsRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory EditEntityArrayFieldAddItemsRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'EditEntityArrayFieldAddItemsRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'manageId')
    ..aOS(2, _omitFieldNames ? '' : 'entityId')
    ..aOS(3, _omitFieldNames ? '' : 'fieldId')
    ..a<$core.List<$core.int>>(4, _omitFieldNames ? '' : 'items', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  EditEntityArrayFieldAddItemsRequest clone() => EditEntityArrayFieldAddItemsRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  EditEntityArrayFieldAddItemsRequest copyWith(void Function(EditEntityArrayFieldAddItemsRequest) updates) => super.copyWith((message) => updates(message as EditEntityArrayFieldAddItemsRequest)) as EditEntityArrayFieldAddItemsRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static EditEntityArrayFieldAddItemsRequest create() => EditEntityArrayFieldAddItemsRequest._();
  EditEntityArrayFieldAddItemsRequest createEmptyInstance() => create();
  static $pb.PbList<EditEntityArrayFieldAddItemsRequest> createRepeated() => $pb.PbList<EditEntityArrayFieldAddItemsRequest>();
  @$core.pragma('dart2js:noInline')
  static EditEntityArrayFieldAddItemsRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<EditEntityArrayFieldAddItemsRequest>(create);
  static EditEntityArrayFieldAddItemsRequest? _defaultInstance;

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
  $core.String get fieldId => $_getSZ(2);
  @$pb.TagNumber(3)
  set fieldId($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasFieldId() => $_has(2);
  @$pb.TagNumber(3)
  void clearFieldId() => clearField(3);

  /// {field_id:[items]}
  @$pb.TagNumber(4)
  $core.List<$core.int> get items => $_getN(3);
  @$pb.TagNumber(4)
  set items($core.List<$core.int> v) { $_setBytes(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasItems() => $_has(3);
  @$pb.TagNumber(4)
  void clearItems() => clearField(4);
}

class EditEntityArrayFieldAddItemsResponse extends $pb.GeneratedMessage {
  factory EditEntityArrayFieldAddItemsResponse({
    $core.String? result,
  }) {
    final $result = create();
    if (result != null) {
      $result.result = result;
    }
    return $result;
  }
  EditEntityArrayFieldAddItemsResponse._() : super();
  factory EditEntityArrayFieldAddItemsResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory EditEntityArrayFieldAddItemsResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'EditEntityArrayFieldAddItemsResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'result')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  EditEntityArrayFieldAddItemsResponse clone() => EditEntityArrayFieldAddItemsResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  EditEntityArrayFieldAddItemsResponse copyWith(void Function(EditEntityArrayFieldAddItemsResponse) updates) => super.copyWith((message) => updates(message as EditEntityArrayFieldAddItemsResponse)) as EditEntityArrayFieldAddItemsResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static EditEntityArrayFieldAddItemsResponse create() => EditEntityArrayFieldAddItemsResponse._();
  EditEntityArrayFieldAddItemsResponse createEmptyInstance() => create();
  static $pb.PbList<EditEntityArrayFieldAddItemsResponse> createRepeated() => $pb.PbList<EditEntityArrayFieldAddItemsResponse>();
  @$core.pragma('dart2js:noInline')
  static EditEntityArrayFieldAddItemsResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<EditEntityArrayFieldAddItemsResponse>(create);
  static EditEntityArrayFieldAddItemsResponse? _defaultInstance;

  /// 成功返回"ok"
  @$pb.TagNumber(1)
  $core.String get result => $_getSZ(0);
  @$pb.TagNumber(1)
  set result($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasResult() => $_has(0);
  @$pb.TagNumber(1)
  void clearResult() => clearField(1);
}

/// 修改单个实体List实体属性, 移除物体
class EditEntityArrayFieldRemoveItemsRequest extends $pb.GeneratedMessage {
  factory EditEntityArrayFieldRemoveItemsRequest({
    $core.String? manageId,
    $core.String? entityId,
    $core.String? fieldId,
    $core.List<$core.int>? items,
  }) {
    final $result = create();
    if (manageId != null) {
      $result.manageId = manageId;
    }
    if (entityId != null) {
      $result.entityId = entityId;
    }
    if (fieldId != null) {
      $result.fieldId = fieldId;
    }
    if (items != null) {
      $result.items = items;
    }
    return $result;
  }
  EditEntityArrayFieldRemoveItemsRequest._() : super();
  factory EditEntityArrayFieldRemoveItemsRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory EditEntityArrayFieldRemoveItemsRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'EditEntityArrayFieldRemoveItemsRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'manageId')
    ..aOS(2, _omitFieldNames ? '' : 'entityId')
    ..aOS(3, _omitFieldNames ? '' : 'fieldId')
    ..a<$core.List<$core.int>>(4, _omitFieldNames ? '' : 'items', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  EditEntityArrayFieldRemoveItemsRequest clone() => EditEntityArrayFieldRemoveItemsRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  EditEntityArrayFieldRemoveItemsRequest copyWith(void Function(EditEntityArrayFieldRemoveItemsRequest) updates) => super.copyWith((message) => updates(message as EditEntityArrayFieldRemoveItemsRequest)) as EditEntityArrayFieldRemoveItemsRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static EditEntityArrayFieldRemoveItemsRequest create() => EditEntityArrayFieldRemoveItemsRequest._();
  EditEntityArrayFieldRemoveItemsRequest createEmptyInstance() => create();
  static $pb.PbList<EditEntityArrayFieldRemoveItemsRequest> createRepeated() => $pb.PbList<EditEntityArrayFieldRemoveItemsRequest>();
  @$core.pragma('dart2js:noInline')
  static EditEntityArrayFieldRemoveItemsRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<EditEntityArrayFieldRemoveItemsRequest>(create);
  static EditEntityArrayFieldRemoveItemsRequest? _defaultInstance;

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
  $core.String get fieldId => $_getSZ(2);
  @$pb.TagNumber(3)
  set fieldId($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasFieldId() => $_has(2);
  @$pb.TagNumber(3)
  void clearFieldId() => clearField(3);

  /// {field_id:[items]}
  @$pb.TagNumber(4)
  $core.List<$core.int> get items => $_getN(3);
  @$pb.TagNumber(4)
  set items($core.List<$core.int> v) { $_setBytes(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasItems() => $_has(3);
  @$pb.TagNumber(4)
  void clearItems() => clearField(4);
}

class EditEntityArrayFieldRemoveItemsResponse extends $pb.GeneratedMessage {
  factory EditEntityArrayFieldRemoveItemsResponse({
    $core.String? result,
  }) {
    final $result = create();
    if (result != null) {
      $result.result = result;
    }
    return $result;
  }
  EditEntityArrayFieldRemoveItemsResponse._() : super();
  factory EditEntityArrayFieldRemoveItemsResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory EditEntityArrayFieldRemoveItemsResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'EditEntityArrayFieldRemoveItemsResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'result')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  EditEntityArrayFieldRemoveItemsResponse clone() => EditEntityArrayFieldRemoveItemsResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  EditEntityArrayFieldRemoveItemsResponse copyWith(void Function(EditEntityArrayFieldRemoveItemsResponse) updates) => super.copyWith((message) => updates(message as EditEntityArrayFieldRemoveItemsResponse)) as EditEntityArrayFieldRemoveItemsResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static EditEntityArrayFieldRemoveItemsResponse create() => EditEntityArrayFieldRemoveItemsResponse._();
  EditEntityArrayFieldRemoveItemsResponse createEmptyInstance() => create();
  static $pb.PbList<EditEntityArrayFieldRemoveItemsResponse> createRepeated() => $pb.PbList<EditEntityArrayFieldRemoveItemsResponse>();
  @$core.pragma('dart2js:noInline')
  static EditEntityArrayFieldRemoveItemsResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<EditEntityArrayFieldRemoveItemsResponse>(create);
  static EditEntityArrayFieldRemoveItemsResponse? _defaultInstance;

  /// 成功返回"ok"
  @$pb.TagNumber(1)
  $core.String get result => $_getSZ(0);
  @$pb.TagNumber(1)
  set result($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasResult() => $_has(0);
  @$pb.TagNumber(1)
  void clearResult() => clearField(1);
}

/// 依据id取得单个实体
class GetEntityRequest extends $pb.GeneratedMessage {
  factory GetEntityRequest({
    $core.String? manageId,
    $core.String? entityId,
    $core.Iterable<$core.String>? noPresentFields,
    $core.Iterable<$core.String>? presentFields,
  }) {
    final $result = create();
    if (manageId != null) {
      $result.manageId = manageId;
    }
    if (entityId != null) {
      $result.entityId = entityId;
    }
    if (noPresentFields != null) {
      $result.noPresentFields.addAll(noPresentFields);
    }
    if (presentFields != null) {
      $result.presentFields.addAll(presentFields);
    }
    return $result;
  }
  GetEntityRequest._() : super();
  factory GetEntityRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetEntityRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'GetEntityRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'manageId')
    ..aOS(2, _omitFieldNames ? '' : 'entityId')
    ..pPS(3, _omitFieldNames ? '' : 'noPresentFields')
    ..pPS(4, _omitFieldNames ? '' : 'presentFields')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetEntityRequest clone() => GetEntityRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetEntityRequest copyWith(void Function(GetEntityRequest) updates) => super.copyWith((message) => updates(message as GetEntityRequest)) as GetEntityRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GetEntityRequest create() => GetEntityRequest._();
  GetEntityRequest createEmptyInstance() => create();
  static $pb.PbList<GetEntityRequest> createRepeated() => $pb.PbList<GetEntityRequest>();
  @$core.pragma('dart2js:noInline')
  static GetEntityRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetEntityRequest>(create);
  static GetEntityRequest? _defaultInstance;

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

  /// 不出现的字段, 主要用于分步加载数据
  @$pb.TagNumber(3)
  $core.List<$core.String> get noPresentFields => $_getList(2);

  @$pb.TagNumber(4)
  $core.List<$core.String> get presentFields => $_getList(3);
}

class GetEntityResponse extends $pb.GeneratedMessage {
  factory GetEntityResponse({
    $core.List<$core.int>? entity,
  }) {
    final $result = create();
    if (entity != null) {
      $result.entity = entity;
    }
    return $result;
  }
  GetEntityResponse._() : super();
  factory GetEntityResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetEntityResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'GetEntityResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'entity', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetEntityResponse clone() => GetEntityResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetEntityResponse copyWith(void Function(GetEntityResponse) updates) => super.copyWith((message) => updates(message as GetEntityResponse)) as GetEntityResponse;

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

/// 依据id列表取得多个实体
class GetEntitiesRequest extends $pb.GeneratedMessage {
  factory GetEntitiesRequest({
    $core.String? manageId,
    $core.Iterable<$core.String>? entityIds,
    $core.Iterable<$core.String>? noPresentFields,
    $core.Iterable<$core.String>? presentFields,
  }) {
    final $result = create();
    if (manageId != null) {
      $result.manageId = manageId;
    }
    if (entityIds != null) {
      $result.entityIds.addAll(entityIds);
    }
    if (noPresentFields != null) {
      $result.noPresentFields.addAll(noPresentFields);
    }
    if (presentFields != null) {
      $result.presentFields.addAll(presentFields);
    }
    return $result;
  }
  GetEntitiesRequest._() : super();
  factory GetEntitiesRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetEntitiesRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'GetEntitiesRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'manageId')
    ..pPS(2, _omitFieldNames ? '' : 'entityIds')
    ..pPS(3, _omitFieldNames ? '' : 'noPresentFields')
    ..pPS(4, _omitFieldNames ? '' : 'presentFields')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetEntitiesRequest clone() => GetEntitiesRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetEntitiesRequest copyWith(void Function(GetEntitiesRequest) updates) => super.copyWith((message) => updates(message as GetEntitiesRequest)) as GetEntitiesRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GetEntitiesRequest create() => GetEntitiesRequest._();
  GetEntitiesRequest createEmptyInstance() => create();
  static $pb.PbList<GetEntitiesRequest> createRepeated() => $pb.PbList<GetEntitiesRequest>();
  @$core.pragma('dart2js:noInline')
  static GetEntitiesRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetEntitiesRequest>(create);
  static GetEntitiesRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get manageId => $_getSZ(0);
  @$pb.TagNumber(1)
  set manageId($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasManageId() => $_has(0);
  @$pb.TagNumber(1)
  void clearManageId() => clearField(1);

  /// 列表最长100, 根据需要指定长度
  @$pb.TagNumber(2)
  $core.List<$core.String> get entityIds => $_getList(1);

  /// 不出现的字段, 主要用于分步加载数据
  @$pb.TagNumber(3)
  $core.List<$core.String> get noPresentFields => $_getList(2);

  @$pb.TagNumber(4)
  $core.List<$core.String> get presentFields => $_getList(3);
}

class GetEntitiesResponse extends $pb.GeneratedMessage {
  factory GetEntitiesResponse({
    $core.List<$core.int>? entity,
  }) {
    final $result = create();
    if (entity != null) {
      $result.entity = entity;
    }
    return $result;
  }
  GetEntitiesResponse._() : super();
  factory GetEntitiesResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetEntitiesResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'GetEntitiesResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'entity', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetEntitiesResponse clone() => GetEntitiesResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetEntitiesResponse copyWith(void Function(GetEntitiesResponse) updates) => super.copyWith((message) => updates(message as GetEntitiesResponse)) as GetEntitiesResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GetEntitiesResponse create() => GetEntitiesResponse._();
  GetEntitiesResponse createEmptyInstance() => create();
  static $pb.PbList<GetEntitiesResponse> createRepeated() => $pb.PbList<GetEntitiesResponse>();
  @$core.pragma('dart2js:noInline')
  static GetEntitiesResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetEntitiesResponse>(create);
  static GetEntitiesResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get entity => $_getN(0);
  @$pb.TagNumber(1)
  set entity($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasEntity() => $_has(0);
  @$pb.TagNumber(1)
  void clearEntity() => clearField(1);
}

/// 依据页码取得实体页列表，页码从1开始
/// 需要先取得实体总数，然后计算页数
class GetEntitiesPageRequest extends $pb.GeneratedMessage {
  factory GetEntitiesPageRequest({
    $core.String? manageId,
    $core.int? pageIndex,
    $core.List<$core.int>? matchDoc,
    $core.List<$core.int>? sortDoc,
    $core.Iterable<$core.String>? noPresentFields,
    $core.String? startOid,
    $core.Iterable<$core.String>? presentFields,
  }) {
    final $result = create();
    if (manageId != null) {
      $result.manageId = manageId;
    }
    if (pageIndex != null) {
      $result.pageIndex = pageIndex;
    }
    if (matchDoc != null) {
      $result.matchDoc = matchDoc;
    }
    if (sortDoc != null) {
      $result.sortDoc = sortDoc;
    }
    if (noPresentFields != null) {
      $result.noPresentFields.addAll(noPresentFields);
    }
    if (startOid != null) {
      $result.startOid = startOid;
    }
    if (presentFields != null) {
      $result.presentFields.addAll(presentFields);
    }
    return $result;
  }
  GetEntitiesPageRequest._() : super();
  factory GetEntitiesPageRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetEntitiesPageRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'GetEntitiesPageRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'manageId')
    ..a<$core.int>(2, _omitFieldNames ? '' : 'pageIndex', $pb.PbFieldType.OU3)
    ..a<$core.List<$core.int>>(3, _omitFieldNames ? '' : 'matchDoc', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(4, _omitFieldNames ? '' : 'sortDoc', $pb.PbFieldType.OY)
    ..pPS(5, _omitFieldNames ? '' : 'noPresentFields')
    ..aOS(6, _omitFieldNames ? '' : 'startOid')
    ..pPS(7, _omitFieldNames ? '' : 'presentFields')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetEntitiesPageRequest clone() => GetEntitiesPageRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetEntitiesPageRequest copyWith(void Function(GetEntitiesPageRequest) updates) => super.copyWith((message) => updates(message as GetEntitiesPageRequest)) as GetEntitiesPageRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GetEntitiesPageRequest create() => GetEntitiesPageRequest._();
  GetEntitiesPageRequest createEmptyInstance() => create();
  static $pb.PbList<GetEntitiesPageRequest> createRepeated() => $pb.PbList<GetEntitiesPageRequest>();
  @$core.pragma('dart2js:noInline')
  static GetEntitiesPageRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetEntitiesPageRequest>(create);
  static GetEntitiesPageRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get manageId => $_getSZ(0);
  @$pb.TagNumber(1)
  set manageId($core.String v) { $_setString(0, v); }
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

  /// 过滤条件 bson document
  @$pb.TagNumber(3)
  $core.List<$core.int> get matchDoc => $_getN(2);
  @$pb.TagNumber(3)
  set matchDoc($core.List<$core.int> v) { $_setBytes(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasMatchDoc() => $_has(2);
  @$pb.TagNumber(3)
  void clearMatchDoc() => clearField(3);

  /// 排序条件 bson document
  @$pb.TagNumber(4)
  $core.List<$core.int> get sortDoc => $_getN(3);
  @$pb.TagNumber(4)
  set sortDoc($core.List<$core.int> v) { $_setBytes(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasSortDoc() => $_has(3);
  @$pb.TagNumber(4)
  void clearSortDoc() => clearField(4);

  /// 不出现的字段, 主要用于分步加载数据
  @$pb.TagNumber(5)
  $core.List<$core.String> get noPresentFields => $_getList(4);

  /// 起始oid，用于分页，第一页不需要指定
  @$pb.TagNumber(6)
  $core.String get startOid => $_getSZ(5);
  @$pb.TagNumber(6)
  set startOid($core.String v) { $_setString(5, v); }
  @$pb.TagNumber(6)
  $core.bool hasStartOid() => $_has(5);
  @$pb.TagNumber(6)
  void clearStartOid() => clearField(6);

  @$pb.TagNumber(7)
  $core.List<$core.String> get presentFields => $_getList(6);
}

/// 返回为流
class GetEntitiesPageResponse extends $pb.GeneratedMessage {
  factory GetEntitiesPageResponse({
    $core.List<$core.int>? entity,
  }) {
    final $result = create();
    if (entity != null) {
      $result.entity = entity;
    }
    return $result;
  }
  GetEntitiesPageResponse._() : super();
  factory GetEntitiesPageResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetEntitiesPageResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'GetEntitiesPageResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'entity', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetEntitiesPageResponse clone() => GetEntitiesPageResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetEntitiesPageResponse copyWith(void Function(GetEntitiesPageResponse) updates) => super.copyWith((message) => updates(message as GetEntitiesPageResponse)) as GetEntitiesPageResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GetEntitiesPageResponse create() => GetEntitiesPageResponse._();
  GetEntitiesPageResponse createEmptyInstance() => create();
  static $pb.PbList<GetEntitiesPageResponse> createRepeated() => $pb.PbList<GetEntitiesPageResponse>();
  @$core.pragma('dart2js:noInline')
  static GetEntitiesPageResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetEntitiesPageResponse>(create);
  static GetEntitiesPageResponse? _defaultInstance;

  /// bson docuemts
  @$pb.TagNumber(1)
  $core.List<$core.int> get entity => $_getN(0);
  @$pb.TagNumber(1)
  set entity($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasEntity() => $_has(0);
  @$pb.TagNumber(1)
  void clearEntity() => clearField(1);
}

/// 交互取得实体页
class InteractiveGetEntitiesRequest extends $pb.GeneratedMessage {
  factory InteractiveGetEntitiesRequest({
    $core.String? manageId,
    $core.int? pageIndex,
    $core.List<$core.int>? matchDoc,
    $core.List<$core.int>? sortDoc,
    $core.Iterable<$core.String>? noPresentFields,
    $core.Iterable<$core.String>? presentFields,
  }) {
    final $result = create();
    if (manageId != null) {
      $result.manageId = manageId;
    }
    if (pageIndex != null) {
      $result.pageIndex = pageIndex;
    }
    if (matchDoc != null) {
      $result.matchDoc = matchDoc;
    }
    if (sortDoc != null) {
      $result.sortDoc = sortDoc;
    }
    if (noPresentFields != null) {
      $result.noPresentFields.addAll(noPresentFields);
    }
    if (presentFields != null) {
      $result.presentFields.addAll(presentFields);
    }
    return $result;
  }
  InteractiveGetEntitiesRequest._() : super();
  factory InteractiveGetEntitiesRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory InteractiveGetEntitiesRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'InteractiveGetEntitiesRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'manageId')
    ..a<$core.int>(2, _omitFieldNames ? '' : 'pageIndex', $pb.PbFieldType.OU3)
    ..a<$core.List<$core.int>>(3, _omitFieldNames ? '' : 'matchDoc', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(4, _omitFieldNames ? '' : 'sortDoc', $pb.PbFieldType.OY)
    ..pPS(5, _omitFieldNames ? '' : 'noPresentFields')
    ..pPS(6, _omitFieldNames ? '' : 'presentFields')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  InteractiveGetEntitiesRequest clone() => InteractiveGetEntitiesRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  InteractiveGetEntitiesRequest copyWith(void Function(InteractiveGetEntitiesRequest) updates) => super.copyWith((message) => updates(message as InteractiveGetEntitiesRequest)) as InteractiveGetEntitiesRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static InteractiveGetEntitiesRequest create() => InteractiveGetEntitiesRequest._();
  InteractiveGetEntitiesRequest createEmptyInstance() => create();
  static $pb.PbList<InteractiveGetEntitiesRequest> createRepeated() => $pb.PbList<InteractiveGetEntitiesRequest>();
  @$core.pragma('dart2js:noInline')
  static InteractiveGetEntitiesRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<InteractiveGetEntitiesRequest>(create);
  static InteractiveGetEntitiesRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get manageId => $_getSZ(0);
  @$pb.TagNumber(1)
  set manageId($core.String v) { $_setString(0, v); }
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

  /// bson document
  @$pb.TagNumber(3)
  $core.List<$core.int> get matchDoc => $_getN(2);
  @$pb.TagNumber(3)
  set matchDoc($core.List<$core.int> v) { $_setBytes(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasMatchDoc() => $_has(2);
  @$pb.TagNumber(3)
  void clearMatchDoc() => clearField(3);

  @$pb.TagNumber(4)
  $core.List<$core.int> get sortDoc => $_getN(3);
  @$pb.TagNumber(4)
  set sortDoc($core.List<$core.int> v) { $_setBytes(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasSortDoc() => $_has(3);
  @$pb.TagNumber(4)
  void clearSortDoc() => clearField(4);

  @$pb.TagNumber(5)
  $core.List<$core.String> get noPresentFields => $_getList(4);

  @$pb.TagNumber(6)
  $core.List<$core.String> get presentFields => $_getList(5);
}

class InteractiveGetEntitiesResponse extends $pb.GeneratedMessage {
  factory InteractiveGetEntitiesResponse({
    $core.int? pageIndex,
    $core.Iterable<$core.List<$core.int>>? entities,
    $fixnum.Int64? totalCount,
  }) {
    final $result = create();
    if (pageIndex != null) {
      $result.pageIndex = pageIndex;
    }
    if (entities != null) {
      $result.entities.addAll(entities);
    }
    if (totalCount != null) {
      $result.totalCount = totalCount;
    }
    return $result;
  }
  InteractiveGetEntitiesResponse._() : super();
  factory InteractiveGetEntitiesResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory InteractiveGetEntitiesResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'InteractiveGetEntitiesResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..a<$core.int>(1, _omitFieldNames ? '' : 'pageIndex', $pb.PbFieldType.OU3)
    ..p<$core.List<$core.int>>(2, _omitFieldNames ? '' : 'entities', $pb.PbFieldType.PY)
    ..a<$fixnum.Int64>(3, _omitFieldNames ? '' : 'totalCount', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  InteractiveGetEntitiesResponse clone() => InteractiveGetEntitiesResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  InteractiveGetEntitiesResponse copyWith(void Function(InteractiveGetEntitiesResponse) updates) => super.copyWith((message) => updates(message as InteractiveGetEntitiesResponse)) as InteractiveGetEntitiesResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static InteractiveGetEntitiesResponse create() => InteractiveGetEntitiesResponse._();
  InteractiveGetEntitiesResponse createEmptyInstance() => create();
  static $pb.PbList<InteractiveGetEntitiesResponse> createRepeated() => $pb.PbList<InteractiveGetEntitiesResponse>();
  @$core.pragma('dart2js:noInline')
  static InteractiveGetEntitiesResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<InteractiveGetEntitiesResponse>(create);
  static InteractiveGetEntitiesResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get pageIndex => $_getIZ(0);
  @$pb.TagNumber(1)
  set pageIndex($core.int v) { $_setUnsignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasPageIndex() => $_has(0);
  @$pb.TagNumber(1)
  void clearPageIndex() => clearField(1);

  @$pb.TagNumber(2)
  $core.List<$core.List<$core.int>> get entities => $_getList(1);

  @$pb.TagNumber(3)
  $fixnum.Int64 get totalCount => $_getI64(2);
  @$pb.TagNumber(3)
  set totalCount($fixnum.Int64 v) { $_setInt64(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasTotalCount() => $_has(2);
  @$pb.TagNumber(3)
  void clearTotalCount() => clearField(3);
}

/// / 取得硬编码管理实体
class GetHardCodedEntitiesRequest extends $pb.GeneratedMessage {
  factory GetHardCodedEntitiesRequest({
    $core.String? manageId,
  }) {
    final $result = create();
    if (manageId != null) {
      $result.manageId = manageId;
    }
    return $result;
  }
  GetHardCodedEntitiesRequest._() : super();
  factory GetHardCodedEntitiesRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetHardCodedEntitiesRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'GetHardCodedEntitiesRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'manageId')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetHardCodedEntitiesRequest clone() => GetHardCodedEntitiesRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetHardCodedEntitiesRequest copyWith(void Function(GetHardCodedEntitiesRequest) updates) => super.copyWith((message) => updates(message as GetHardCodedEntitiesRequest)) as GetHardCodedEntitiesRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GetHardCodedEntitiesRequest create() => GetHardCodedEntitiesRequest._();
  GetHardCodedEntitiesRequest createEmptyInstance() => create();
  static $pb.PbList<GetHardCodedEntitiesRequest> createRepeated() => $pb.PbList<GetHardCodedEntitiesRequest>();
  @$core.pragma('dart2js:noInline')
  static GetHardCodedEntitiesRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetHardCodedEntitiesRequest>(create);
  static GetHardCodedEntitiesRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get manageId => $_getSZ(0);
  @$pb.TagNumber(1)
  set manageId($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasManageId() => $_has(0);
  @$pb.TagNumber(1)
  void clearManageId() => clearField(1);
}

class GetHardCodedEntitiesResponse extends $pb.GeneratedMessage {
  factory GetHardCodedEntitiesResponse({
    $core.Iterable<$core.List<$core.int>>? entities,
  }) {
    final $result = create();
    if (entities != null) {
      $result.entities.addAll(entities);
    }
    return $result;
  }
  GetHardCodedEntitiesResponse._() : super();
  factory GetHardCodedEntitiesResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetHardCodedEntitiesResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'GetHardCodedEntitiesResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..p<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'entities', $pb.PbFieldType.PY)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetHardCodedEntitiesResponse clone() => GetHardCodedEntitiesResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetHardCodedEntitiesResponse copyWith(void Function(GetHardCodedEntitiesResponse) updates) => super.copyWith((message) => updates(message as GetHardCodedEntitiesResponse)) as GetHardCodedEntitiesResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GetHardCodedEntitiesResponse create() => GetHardCodedEntitiesResponse._();
  GetHardCodedEntitiesResponse createEmptyInstance() => create();
  static $pb.PbList<GetHardCodedEntitiesResponse> createRepeated() => $pb.PbList<GetHardCodedEntitiesResponse>();
  @$core.pragma('dart2js:noInline')
  static GetHardCodedEntitiesResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetHardCodedEntitiesResponse>(create);
  static GetHardCodedEntitiesResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.List<$core.int>> get entities => $_getList(0);
}

/// 标记实体已移除
class MarkEntityRemovedRequest extends $pb.GeneratedMessage {
  factory MarkEntityRemovedRequest({
    $core.String? manageId,
    $core.String? entityId,
  }) {
    final $result = create();
    if (manageId != null) {
      $result.manageId = manageId;
    }
    if (entityId != null) {
      $result.entityId = entityId;
    }
    return $result;
  }
  MarkEntityRemovedRequest._() : super();
  factory MarkEntityRemovedRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory MarkEntityRemovedRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'MarkEntityRemovedRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'manageId')
    ..aOS(2, _omitFieldNames ? '' : 'entityId')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  MarkEntityRemovedRequest clone() => MarkEntityRemovedRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  MarkEntityRemovedRequest copyWith(void Function(MarkEntityRemovedRequest) updates) => super.copyWith((message) => updates(message as MarkEntityRemovedRequest)) as MarkEntityRemovedRequest;

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
  factory MarkEntityRemovedResponse({
    $core.String? result,
  }) {
    final $result = create();
    if (result != null) {
      $result.result = result;
    }
    return $result;
  }
  MarkEntityRemovedResponse._() : super();
  factory MarkEntityRemovedResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory MarkEntityRemovedResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'MarkEntityRemovedResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'result')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  MarkEntityRemovedResponse clone() => MarkEntityRemovedResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  MarkEntityRemovedResponse copyWith(void Function(MarkEntityRemovedResponse) updates) => super.copyWith((message) => updates(message as MarkEntityRemovedResponse)) as MarkEntityRemovedResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static MarkEntityRemovedResponse create() => MarkEntityRemovedResponse._();
  MarkEntityRemovedResponse createEmptyInstance() => create();
  static $pb.PbList<MarkEntityRemovedResponse> createRepeated() => $pb.PbList<MarkEntityRemovedResponse>();
  @$core.pragma('dart2js:noInline')
  static MarkEntityRemovedResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<MarkEntityRemovedResponse>(create);
  static MarkEntityRemovedResponse? _defaultInstance;

  /// 成功返回"ok"
  @$pb.TagNumber(1)
  $core.String get result => $_getSZ(0);
  @$pb.TagNumber(1)
  set result($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasResult() => $_has(0);
  @$pb.TagNumber(1)
  void clearResult() => clearField(1);
}

/// 恢复标记为已移除的实体
class RecoverRemovedEntityRequest extends $pb.GeneratedMessage {
  factory RecoverRemovedEntityRequest({
    $core.String? manageId,
    $core.String? entityId,
  }) {
    final $result = create();
    if (manageId != null) {
      $result.manageId = manageId;
    }
    if (entityId != null) {
      $result.entityId = entityId;
    }
    return $result;
  }
  RecoverRemovedEntityRequest._() : super();
  factory RecoverRemovedEntityRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory RecoverRemovedEntityRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'RecoverRemovedEntityRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'manageId')
    ..aOS(2, _omitFieldNames ? '' : 'entityId')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  RecoverRemovedEntityRequest clone() => RecoverRemovedEntityRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  RecoverRemovedEntityRequest copyWith(void Function(RecoverRemovedEntityRequest) updates) => super.copyWith((message) => updates(message as RecoverRemovedEntityRequest)) as RecoverRemovedEntityRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static RecoverRemovedEntityRequest create() => RecoverRemovedEntityRequest._();
  RecoverRemovedEntityRequest createEmptyInstance() => create();
  static $pb.PbList<RecoverRemovedEntityRequest> createRepeated() => $pb.PbList<RecoverRemovedEntityRequest>();
  @$core.pragma('dart2js:noInline')
  static RecoverRemovedEntityRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<RecoverRemovedEntityRequest>(create);
  static RecoverRemovedEntityRequest? _defaultInstance;

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

class RecoverRemovedEntityResponse extends $pb.GeneratedMessage {
  factory RecoverRemovedEntityResponse({
    $core.String? result,
  }) {
    final $result = create();
    if (result != null) {
      $result.result = result;
    }
    return $result;
  }
  RecoverRemovedEntityResponse._() : super();
  factory RecoverRemovedEntityResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory RecoverRemovedEntityResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'RecoverRemovedEntityResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'result')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  RecoverRemovedEntityResponse clone() => RecoverRemovedEntityResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  RecoverRemovedEntityResponse copyWith(void Function(RecoverRemovedEntityResponse) updates) => super.copyWith((message) => updates(message as RecoverRemovedEntityResponse)) as RecoverRemovedEntityResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static RecoverRemovedEntityResponse create() => RecoverRemovedEntityResponse._();
  RecoverRemovedEntityResponse createEmptyInstance() => create();
  static $pb.PbList<RecoverRemovedEntityResponse> createRepeated() => $pb.PbList<RecoverRemovedEntityResponse>();
  @$core.pragma('dart2js:noInline')
  static RecoverRemovedEntityResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<RecoverRemovedEntityResponse>(create);
  static RecoverRemovedEntityResponse? _defaultInstance;

  /// 成功返回"ok"
  @$pb.TagNumber(1)
  $core.String get result => $_getSZ(0);
  @$pb.TagNumber(1)
  set result($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasResult() => $_has(0);
  @$pb.TagNumber(1)
  void clearResult() => clearField(1);
}

/// 取得已删除实体页
class GetRemovedEntitiesPageRequest extends $pb.GeneratedMessage {
  factory GetRemovedEntitiesPageRequest({
    $core.String? manageId,
    $core.int? pageIndex,
    $core.List<$core.int>? conditions,
  }) {
    final $result = create();
    if (manageId != null) {
      $result.manageId = manageId;
    }
    if (pageIndex != null) {
      $result.pageIndex = pageIndex;
    }
    if (conditions != null) {
      $result.conditions = conditions;
    }
    return $result;
  }
  GetRemovedEntitiesPageRequest._() : super();
  factory GetRemovedEntitiesPageRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetRemovedEntitiesPageRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'GetRemovedEntitiesPageRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'manageId')
    ..a<$core.int>(2, _omitFieldNames ? '' : 'pageIndex', $pb.PbFieldType.OU3)
    ..a<$core.List<$core.int>>(3, _omitFieldNames ? '' : 'conditions', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetRemovedEntitiesPageRequest clone() => GetRemovedEntitiesPageRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetRemovedEntitiesPageRequest copyWith(void Function(GetRemovedEntitiesPageRequest) updates) => super.copyWith((message) => updates(message as GetRemovedEntitiesPageRequest)) as GetRemovedEntitiesPageRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GetRemovedEntitiesPageRequest create() => GetRemovedEntitiesPageRequest._();
  GetRemovedEntitiesPageRequest createEmptyInstance() => create();
  static $pb.PbList<GetRemovedEntitiesPageRequest> createRepeated() => $pb.PbList<GetRemovedEntitiesPageRequest>();
  @$core.pragma('dart2js:noInline')
  static GetRemovedEntitiesPageRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetRemovedEntitiesPageRequest>(create);
  static GetRemovedEntitiesPageRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get manageId => $_getSZ(0);
  @$pb.TagNumber(1)
  set manageId($core.String v) { $_setString(0, v); }
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

class GetRemovedEntitiesPageResponse extends $pb.GeneratedMessage {
  factory GetRemovedEntitiesPageResponse({
    $core.Iterable<$core.List<$core.int>>? entities,
  }) {
    final $result = create();
    if (entities != null) {
      $result.entities.addAll(entities);
    }
    return $result;
  }
  GetRemovedEntitiesPageResponse._() : super();
  factory GetRemovedEntitiesPageResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetRemovedEntitiesPageResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'GetRemovedEntitiesPageResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..p<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'entities', $pb.PbFieldType.PY)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetRemovedEntitiesPageResponse clone() => GetRemovedEntitiesPageResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetRemovedEntitiesPageResponse copyWith(void Function(GetRemovedEntitiesPageResponse) updates) => super.copyWith((message) => updates(message as GetRemovedEntitiesPageResponse)) as GetRemovedEntitiesPageResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GetRemovedEntitiesPageResponse create() => GetRemovedEntitiesPageResponse._();
  GetRemovedEntitiesPageResponse createEmptyInstance() => create();
  static $pb.PbList<GetRemovedEntitiesPageResponse> createRepeated() => $pb.PbList<GetRemovedEntitiesPageResponse>();
  @$core.pragma('dart2js:noInline')
  static GetRemovedEntitiesPageResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetRemovedEntitiesPageResponse>(create);
  static GetRemovedEntitiesPageResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.List<$core.int>> get entities => $_getList(0);
}

/// 取得实体已标记移除数据表
class GetRemovedDataListRequest extends $pb.GeneratedMessage {
  factory GetRemovedDataListRequest({
    $core.String? manageId,
    $core.String? entityId,
    $core.String? dataId,
  }) {
    final $result = create();
    if (manageId != null) {
      $result.manageId = manageId;
    }
    if (entityId != null) {
      $result.entityId = entityId;
    }
    if (dataId != null) {
      $result.dataId = dataId;
    }
    return $result;
  }
  GetRemovedDataListRequest._() : super();
  factory GetRemovedDataListRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetRemovedDataListRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'GetRemovedDataListRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'manageId')
    ..aOS(2, _omitFieldNames ? '' : 'entityId')
    ..aOS(3, _omitFieldNames ? '' : 'dataId')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetRemovedDataListRequest clone() => GetRemovedDataListRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetRemovedDataListRequest copyWith(void Function(GetRemovedDataListRequest) updates) => super.copyWith((message) => updates(message as GetRemovedDataListRequest)) as GetRemovedDataListRequest;

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
  factory GetRemovedDataListResponse({
    $core.Iterable<$core.String>? dataIds,
  }) {
    final $result = create();
    if (dataIds != null) {
      $result.dataIds.addAll(dataIds);
    }
    return $result;
  }
  GetRemovedDataListResponse._() : super();
  factory GetRemovedDataListResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetRemovedDataListResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'GetRemovedDataListResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..pPS(1, _omitFieldNames ? '' : 'dataIds')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetRemovedDataListResponse clone() => GetRemovedDataListResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetRemovedDataListResponse copyWith(void Function(GetRemovedDataListResponse) updates) => super.copyWith((message) => updates(message as GetRemovedDataListResponse)) as GetRemovedDataListResponse;

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

/// ---------
/// 更新检查
/// ---------
class EntityTimestamp extends $pb.GeneratedMessage {
  factory EntityTimestamp({
    $core.String? entityId,
    $core.List<$core.int>? timestamp,
  }) {
    final $result = create();
    if (entityId != null) {
      $result.entityId = entityId;
    }
    if (timestamp != null) {
      $result.timestamp = timestamp;
    }
    return $result;
  }
  EntityTimestamp._() : super();
  factory EntityTimestamp.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory EntityTimestamp.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'EntityTimestamp', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'entityId')
    ..a<$core.List<$core.int>>(2, _omitFieldNames ? '' : 'timestamp', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  EntityTimestamp clone() => EntityTimestamp()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  EntityTimestamp copyWith(void Function(EntityTimestamp) updates) => super.copyWith((message) => updates(message as EntityTimestamp)) as EntityTimestamp;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static EntityTimestamp create() => EntityTimestamp._();
  EntityTimestamp createEmptyInstance() => create();
  static $pb.PbList<EntityTimestamp> createRepeated() => $pb.PbList<EntityTimestamp>();
  @$core.pragma('dart2js:noInline')
  static EntityTimestamp getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<EntityTimestamp>(create);
  static EntityTimestamp? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get entityId => $_getSZ(0);
  @$pb.TagNumber(1)
  set entityId($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasEntityId() => $_has(0);
  @$pb.TagNumber(1)
  void clearEntityId() => clearField(1);

  /// 格式: 二进制 bson Document 形式: {"value": Timestamp()}
  @$pb.TagNumber(2)
  $core.List<$core.int> get timestamp => $_getN(1);
  @$pb.TagNumber(2)
  set timestamp($core.List<$core.int> v) { $_setBytes(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasTimestamp() => $_has(1);
  @$pb.TagNumber(2)
  void clearTimestamp() => clearField(2);
}

/// 检查实体是否有更新，返回有更新的实体
class CheckEntitiesUpdateRequest extends $pb.GeneratedMessage {
  factory CheckEntitiesUpdateRequest({
    $core.String? manageId,
    $core.Iterable<EntityTimestamp>? entities,
  }) {
    final $result = create();
    if (manageId != null) {
      $result.manageId = manageId;
    }
    if (entities != null) {
      $result.entities.addAll(entities);
    }
    return $result;
  }
  CheckEntitiesUpdateRequest._() : super();
  factory CheckEntitiesUpdateRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory CheckEntitiesUpdateRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'CheckEntitiesUpdateRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'manageId')
    ..pc<EntityTimestamp>(2, _omitFieldNames ? '' : 'entities', $pb.PbFieldType.PM, subBuilder: EntityTimestamp.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  CheckEntitiesUpdateRequest clone() => CheckEntitiesUpdateRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  CheckEntitiesUpdateRequest copyWith(void Function(CheckEntitiesUpdateRequest) updates) => super.copyWith((message) => updates(message as CheckEntitiesUpdateRequest)) as CheckEntitiesUpdateRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static CheckEntitiesUpdateRequest create() => CheckEntitiesUpdateRequest._();
  CheckEntitiesUpdateRequest createEmptyInstance() => create();
  static $pb.PbList<CheckEntitiesUpdateRequest> createRepeated() => $pb.PbList<CheckEntitiesUpdateRequest>();
  @$core.pragma('dart2js:noInline')
  static CheckEntitiesUpdateRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<CheckEntitiesUpdateRequest>(create);
  static CheckEntitiesUpdateRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get manageId => $_getSZ(0);
  @$pb.TagNumber(1)
  set manageId($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasManageId() => $_has(0);
  @$pb.TagNumber(1)
  void clearManageId() => clearField(1);

  /// 列表最长不能超过100
  @$pb.TagNumber(2)
  $core.List<EntityTimestamp> get entities => $_getList(1);
}

class CheckEntitiesUpdateResponse extends $pb.GeneratedMessage {
  factory CheckEntitiesUpdateResponse({
    $core.Iterable<$core.List<$core.int>>? entities,
  }) {
    final $result = create();
    if (entities != null) {
      $result.entities.addAll(entities);
    }
    return $result;
  }
  CheckEntitiesUpdateResponse._() : super();
  factory CheckEntitiesUpdateResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory CheckEntitiesUpdateResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'CheckEntitiesUpdateResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..p<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'entities', $pb.PbFieldType.PY)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  CheckEntitiesUpdateResponse clone() => CheckEntitiesUpdateResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  CheckEntitiesUpdateResponse copyWith(void Function(CheckEntitiesUpdateResponse) updates) => super.copyWith((message) => updates(message as CheckEntitiesUpdateResponse)) as CheckEntitiesUpdateResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static CheckEntitiesUpdateResponse create() => CheckEntitiesUpdateResponse._();
  CheckEntitiesUpdateResponse createEmptyInstance() => create();
  static $pb.PbList<CheckEntitiesUpdateResponse> createRepeated() => $pb.PbList<CheckEntitiesUpdateResponse>();
  @$core.pragma('dart2js:noInline')
  static CheckEntitiesUpdateResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<CheckEntitiesUpdateResponse>(create);
  static CheckEntitiesUpdateResponse? _defaultInstance;

  /// 如果有则返回bson新实体，否则返回空
  @$pb.TagNumber(1)
  $core.List<$core.List<$core.int>> get entities => $_getList(0);
}

/// 检查迟于指定时间是否有更新
/// 返回编号列表页流
class CheckUpdatesLaterThenTimeRequest extends $pb.GeneratedMessage {
  factory CheckUpdatesLaterThenTimeRequest({
    $core.String? manageId,
    $core.List<$core.int>? timestamp,
    $core.bool? ascendingOrder,
    $core.List<$core.int>? filter,
  }) {
    final $result = create();
    if (manageId != null) {
      $result.manageId = manageId;
    }
    if (timestamp != null) {
      $result.timestamp = timestamp;
    }
    if (ascendingOrder != null) {
      $result.ascendingOrder = ascendingOrder;
    }
    if (filter != null) {
      $result.filter = filter;
    }
    return $result;
  }
  CheckUpdatesLaterThenTimeRequest._() : super();
  factory CheckUpdatesLaterThenTimeRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory CheckUpdatesLaterThenTimeRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'CheckUpdatesLaterThenTimeRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'manageId')
    ..a<$core.List<$core.int>>(2, _omitFieldNames ? '' : 'timestamp', $pb.PbFieldType.OY)
    ..aOB(3, _omitFieldNames ? '' : 'ascendingOrder')
    ..a<$core.List<$core.int>>(4, _omitFieldNames ? '' : 'filter', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  CheckUpdatesLaterThenTimeRequest clone() => CheckUpdatesLaterThenTimeRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  CheckUpdatesLaterThenTimeRequest copyWith(void Function(CheckUpdatesLaterThenTimeRequest) updates) => super.copyWith((message) => updates(message as CheckUpdatesLaterThenTimeRequest)) as CheckUpdatesLaterThenTimeRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static CheckUpdatesLaterThenTimeRequest create() => CheckUpdatesLaterThenTimeRequest._();
  CheckUpdatesLaterThenTimeRequest createEmptyInstance() => create();
  static $pb.PbList<CheckUpdatesLaterThenTimeRequest> createRepeated() => $pb.PbList<CheckUpdatesLaterThenTimeRequest>();
  @$core.pragma('dart2js:noInline')
  static CheckUpdatesLaterThenTimeRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<CheckUpdatesLaterThenTimeRequest>(create);
  static CheckUpdatesLaterThenTimeRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get manageId => $_getSZ(0);
  @$pb.TagNumber(1)
  set manageId($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasManageId() => $_has(0);
  @$pb.TagNumber(1)
  void clearManageId() => clearField(1);

  /// 格式二进制 bson Document 形式{"value": Timestamp()}
  @$pb.TagNumber(2)
  $core.List<$core.int> get timestamp => $_getN(1);
  @$pb.TagNumber(2)
  set timestamp($core.List<$core.int> v) { $_setBytes(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasTimestamp() => $_has(1);
  @$pb.TagNumber(2)
  void clearTimestamp() => clearField(2);

  /// 是否按时间升序排列, 默认降序
  @$pb.TagNumber(3)
  $core.bool get ascendingOrder => $_getBF(2);
  @$pb.TagNumber(3)
  set ascendingOrder($core.bool v) { $_setBool(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasAscendingOrder() => $_has(2);
  @$pb.TagNumber(3)
  void clearAscendingOrder() => clearField(3);

  /// bson document 形式 {"key": value}, 只支持匹配形式
  @$pb.TagNumber(4)
  $core.List<$core.int> get filter => $_getN(3);
  @$pb.TagNumber(4)
  set filter($core.List<$core.int> v) { $_setBytes(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasFilter() => $_has(3);
  @$pb.TagNumber(4)
  void clearFilter() => clearField(4);
}

class CheckUpdatesLaterThenTimeResponse extends $pb.GeneratedMessage {
  factory CheckUpdatesLaterThenTimeResponse({
    $core.Iterable<$core.List<$core.int>>? results,
  }) {
    final $result = create();
    if (results != null) {
      $result.results.addAll(results);
    }
    return $result;
  }
  CheckUpdatesLaterThenTimeResponse._() : super();
  factory CheckUpdatesLaterThenTimeResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory CheckUpdatesLaterThenTimeResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'CheckUpdatesLaterThenTimeResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..p<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'results', $pb.PbFieldType.PY)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  CheckUpdatesLaterThenTimeResponse clone() => CheckUpdatesLaterThenTimeResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  CheckUpdatesLaterThenTimeResponse copyWith(void Function(CheckUpdatesLaterThenTimeResponse) updates) => super.copyWith((message) => updates(message as CheckUpdatesLaterThenTimeResponse)) as CheckUpdatesLaterThenTimeResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static CheckUpdatesLaterThenTimeResponse create() => CheckUpdatesLaterThenTimeResponse._();
  CheckUpdatesLaterThenTimeResponse createEmptyInstance() => create();
  static $pb.PbList<CheckUpdatesLaterThenTimeResponse> createRepeated() => $pb.PbList<CheckUpdatesLaterThenTimeResponse>();
  @$core.pragma('dart2js:noInline')
  static CheckUpdatesLaterThenTimeResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<CheckUpdatesLaterThenTimeResponse>(create);
  static CheckUpdatesLaterThenTimeResponse? _defaultInstance;

  /// {"_id", "id", modifiedtimestamp}, 分组返回，每组最多20条
  /// 最多返回1000条
  @$pb.TagNumber(1)
  $core.List<$core.List<$core.int>> get results => $_getList(0);
}


const _omitFieldNames = $core.bool.fromEnvironment('protobuf.omit_field_names');
const _omitMessageNames = $core.bool.fromEnvironment('protobuf.omit_message_names');
