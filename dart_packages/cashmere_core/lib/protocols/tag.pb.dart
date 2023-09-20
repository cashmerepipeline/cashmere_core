//
//  Generated code. Do not modify.
//  source: tag.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

import 'name.pb.dart' as $0;

class NewTagRequest extends $pb.GeneratedMessage {
  factory NewTagRequest({
    $core.int? targetManageId,
    $0.Name? name,
    $core.String? description,
  }) {
    final $result = create();
    if (targetManageId != null) {
      $result.targetManageId = targetManageId;
    }
    if (name != null) {
      $result.name = name;
    }
    if (description != null) {
      $result.description = description;
    }
    return $result;
  }
  NewTagRequest._() : super();
  factory NewTagRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory NewTagRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'NewTagRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..a<$core.int>(1, _omitFieldNames ? '' : 'targetManageId', $pb.PbFieldType.O3)
    ..aOM<$0.Name>(2, _omitFieldNames ? '' : 'name', subBuilder: $0.Name.create)
    ..aOS(3, _omitFieldNames ? '' : 'description')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  NewTagRequest clone() => NewTagRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  NewTagRequest copyWith(void Function(NewTagRequest) updates) => super.copyWith((message) => updates(message as NewTagRequest)) as NewTagRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static NewTagRequest create() => NewTagRequest._();
  NewTagRequest createEmptyInstance() => create();
  static $pb.PbList<NewTagRequest> createRepeated() => $pb.PbList<NewTagRequest>();
  @$core.pragma('dart2js:noInline')
  static NewTagRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<NewTagRequest>(create);
  static NewTagRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get targetManageId => $_getIZ(0);
  @$pb.TagNumber(1)
  set targetManageId($core.int v) { $_setSignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasTargetManageId() => $_has(0);
  @$pb.TagNumber(1)
  void clearTargetManageId() => clearField(1);

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
  $core.String get description => $_getSZ(2);
  @$pb.TagNumber(3)
  set description($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasDescription() => $_has(2);
  @$pb.TagNumber(3)
  void clearDescription() => clearField(3);
}

class NewTagResponse extends $pb.GeneratedMessage {
  factory NewTagResponse({
    $core.String? result,
  }) {
    final $result = create();
    if (result != null) {
      $result.result = result;
    }
    return $result;
  }
  NewTagResponse._() : super();
  factory NewTagResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory NewTagResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'NewTagResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'result')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  NewTagResponse clone() => NewTagResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  NewTagResponse copyWith(void Function(NewTagResponse) updates) => super.copyWith((message) => updates(message as NewTagResponse)) as NewTagResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static NewTagResponse create() => NewTagResponse._();
  NewTagResponse createEmptyInstance() => create();
  static $pb.PbList<NewTagResponse> createRepeated() => $pb.PbList<NewTagResponse>();
  @$core.pragma('dart2js:noInline')
  static NewTagResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<NewTagResponse>(create);
  static NewTagResponse? _defaultInstance;

  /// 成功返回id, 失败返回错误信息
  @$pb.TagNumber(1)
  $core.String get result => $_getSZ(0);
  @$pb.TagNumber(1)
  set result($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasResult() => $_has(0);
  @$pb.TagNumber(1)
  void clearResult() => clearField(1);
}

/// 添加标签到某个实体
class AddTagsToEntityRequest extends $pb.GeneratedMessage {
  factory AddTagsToEntityRequest({
    $core.Iterable<$core.String>? tagIds,
    $core.int? targetManageId,
    $core.String? targetEntityId,
  }) {
    final $result = create();
    if (tagIds != null) {
      $result.tagIds.addAll(tagIds);
    }
    if (targetManageId != null) {
      $result.targetManageId = targetManageId;
    }
    if (targetEntityId != null) {
      $result.targetEntityId = targetEntityId;
    }
    return $result;
  }
  AddTagsToEntityRequest._() : super();
  factory AddTagsToEntityRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory AddTagsToEntityRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'AddTagsToEntityRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..pPS(1, _omitFieldNames ? '' : 'tagIds')
    ..a<$core.int>(2, _omitFieldNames ? '' : 'targetManageId', $pb.PbFieldType.O3)
    ..aOS(3, _omitFieldNames ? '' : 'targetEntityId')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  AddTagsToEntityRequest clone() => AddTagsToEntityRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  AddTagsToEntityRequest copyWith(void Function(AddTagsToEntityRequest) updates) => super.copyWith((message) => updates(message as AddTagsToEntityRequest)) as AddTagsToEntityRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static AddTagsToEntityRequest create() => AddTagsToEntityRequest._();
  AddTagsToEntityRequest createEmptyInstance() => create();
  static $pb.PbList<AddTagsToEntityRequest> createRepeated() => $pb.PbList<AddTagsToEntityRequest>();
  @$core.pragma('dart2js:noInline')
  static AddTagsToEntityRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<AddTagsToEntityRequest>(create);
  static AddTagsToEntityRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.String> get tagIds => $_getList(0);

  @$pb.TagNumber(2)
  $core.int get targetManageId => $_getIZ(1);
  @$pb.TagNumber(2)
  set targetManageId($core.int v) { $_setSignedInt32(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasTargetManageId() => $_has(1);
  @$pb.TagNumber(2)
  void clearTargetManageId() => clearField(2);

  @$pb.TagNumber(3)
  $core.String get targetEntityId => $_getSZ(2);
  @$pb.TagNumber(3)
  set targetEntityId($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasTargetEntityId() => $_has(2);
  @$pb.TagNumber(3)
  void clearTargetEntityId() => clearField(3);
}

class AddTagsToEntityResponse extends $pb.GeneratedMessage {
  factory AddTagsToEntityResponse({
    $core.String? result,
  }) {
    final $result = create();
    if (result != null) {
      $result.result = result;
    }
    return $result;
  }
  AddTagsToEntityResponse._() : super();
  factory AddTagsToEntityResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory AddTagsToEntityResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'AddTagsToEntityResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'result')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  AddTagsToEntityResponse clone() => AddTagsToEntityResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  AddTagsToEntityResponse copyWith(void Function(AddTagsToEntityResponse) updates) => super.copyWith((message) => updates(message as AddTagsToEntityResponse)) as AddTagsToEntityResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static AddTagsToEntityResponse create() => AddTagsToEntityResponse._();
  AddTagsToEntityResponse createEmptyInstance() => create();
  static $pb.PbList<AddTagsToEntityResponse> createRepeated() => $pb.PbList<AddTagsToEntityResponse>();
  @$core.pragma('dart2js:noInline')
  static AddTagsToEntityResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<AddTagsToEntityResponse>(create);
  static AddTagsToEntityResponse? _defaultInstance;

  /// 成功返回“ok”, 失败返回错误信息
  @$pb.TagNumber(1)
  $core.String get result => $_getSZ(0);
  @$pb.TagNumber(1)
  set result($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasResult() => $_has(0);
  @$pb.TagNumber(1)
  void clearResult() => clearField(1);
}

class GetTagsRequest extends $pb.GeneratedMessage {
  factory GetTagsRequest({
    $core.int? targetManageId,
  }) {
    final $result = create();
    if (targetManageId != null) {
      $result.targetManageId = targetManageId;
    }
    return $result;
  }
  GetTagsRequest._() : super();
  factory GetTagsRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetTagsRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'GetTagsRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..a<$core.int>(1, _omitFieldNames ? '' : 'targetManageId', $pb.PbFieldType.O3)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetTagsRequest clone() => GetTagsRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetTagsRequest copyWith(void Function(GetTagsRequest) updates) => super.copyWith((message) => updates(message as GetTagsRequest)) as GetTagsRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GetTagsRequest create() => GetTagsRequest._();
  GetTagsRequest createEmptyInstance() => create();
  static $pb.PbList<GetTagsRequest> createRepeated() => $pb.PbList<GetTagsRequest>();
  @$core.pragma('dart2js:noInline')
  static GetTagsRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetTagsRequest>(create);
  static GetTagsRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get targetManageId => $_getIZ(0);
  @$pb.TagNumber(1)
  set targetManageId($core.int v) { $_setSignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasTargetManageId() => $_has(0);
  @$pb.TagNumber(1)
  void clearTargetManageId() => clearField(1);
}

class GetTagsResponse extends $pb.GeneratedMessage {
  factory GetTagsResponse({
    $core.Iterable<$core.List<$core.int>>? tags,
  }) {
    final $result = create();
    if (tags != null) {
      $result.tags.addAll(tags);
    }
    return $result;
  }
  GetTagsResponse._() : super();
  factory GetTagsResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetTagsResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'GetTagsResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..p<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'tags', $pb.PbFieldType.PY)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetTagsResponse clone() => GetTagsResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetTagsResponse copyWith(void Function(GetTagsResponse) updates) => super.copyWith((message) => updates(message as GetTagsResponse)) as GetTagsResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GetTagsResponse create() => GetTagsResponse._();
  GetTagsResponse createEmptyInstance() => create();
  static $pb.PbList<GetTagsResponse> createRepeated() => $pb.PbList<GetTagsResponse>();
  @$core.pragma('dart2js:noInline')
  static GetTagsResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetTagsResponse>(create);
  static GetTagsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.List<$core.int>> get tags => $_getList(0);
}

/// 移除标签
class RemoveTagsFromEntityRequest extends $pb.GeneratedMessage {
  factory RemoveTagsFromEntityRequest({
    $core.int? targetManageId,
    $core.String? targetEntityId,
    $core.Iterable<$core.String>? tagIds,
  }) {
    final $result = create();
    if (targetManageId != null) {
      $result.targetManageId = targetManageId;
    }
    if (targetEntityId != null) {
      $result.targetEntityId = targetEntityId;
    }
    if (tagIds != null) {
      $result.tagIds.addAll(tagIds);
    }
    return $result;
  }
  RemoveTagsFromEntityRequest._() : super();
  factory RemoveTagsFromEntityRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory RemoveTagsFromEntityRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'RemoveTagsFromEntityRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..a<$core.int>(1, _omitFieldNames ? '' : 'targetManageId', $pb.PbFieldType.O3)
    ..aOS(2, _omitFieldNames ? '' : 'targetEntityId')
    ..pPS(3, _omitFieldNames ? '' : 'tagIds')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  RemoveTagsFromEntityRequest clone() => RemoveTagsFromEntityRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  RemoveTagsFromEntityRequest copyWith(void Function(RemoveTagsFromEntityRequest) updates) => super.copyWith((message) => updates(message as RemoveTagsFromEntityRequest)) as RemoveTagsFromEntityRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static RemoveTagsFromEntityRequest create() => RemoveTagsFromEntityRequest._();
  RemoveTagsFromEntityRequest createEmptyInstance() => create();
  static $pb.PbList<RemoveTagsFromEntityRequest> createRepeated() => $pb.PbList<RemoveTagsFromEntityRequest>();
  @$core.pragma('dart2js:noInline')
  static RemoveTagsFromEntityRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<RemoveTagsFromEntityRequest>(create);
  static RemoveTagsFromEntityRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get targetManageId => $_getIZ(0);
  @$pb.TagNumber(1)
  set targetManageId($core.int v) { $_setSignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasTargetManageId() => $_has(0);
  @$pb.TagNumber(1)
  void clearTargetManageId() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get targetEntityId => $_getSZ(1);
  @$pb.TagNumber(2)
  set targetEntityId($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasTargetEntityId() => $_has(1);
  @$pb.TagNumber(2)
  void clearTargetEntityId() => clearField(2);

  @$pb.TagNumber(3)
  $core.List<$core.String> get tagIds => $_getList(2);
}

class RemoveTagsFromEntityResponse extends $pb.GeneratedMessage {
  factory RemoveTagsFromEntityResponse({
    $core.String? result,
  }) {
    final $result = create();
    if (result != null) {
      $result.result = result;
    }
    return $result;
  }
  RemoveTagsFromEntityResponse._() : super();
  factory RemoveTagsFromEntityResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory RemoveTagsFromEntityResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'RemoveTagsFromEntityResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'result')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  RemoveTagsFromEntityResponse clone() => RemoveTagsFromEntityResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  RemoveTagsFromEntityResponse copyWith(void Function(RemoveTagsFromEntityResponse) updates) => super.copyWith((message) => updates(message as RemoveTagsFromEntityResponse)) as RemoveTagsFromEntityResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static RemoveTagsFromEntityResponse create() => RemoveTagsFromEntityResponse._();
  RemoveTagsFromEntityResponse createEmptyInstance() => create();
  static $pb.PbList<RemoveTagsFromEntityResponse> createRepeated() => $pb.PbList<RemoveTagsFromEntityResponse>();
  @$core.pragma('dart2js:noInline')
  static RemoveTagsFromEntityResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<RemoveTagsFromEntityResponse>(create);
  static RemoveTagsFromEntityResponse? _defaultInstance;

  /// 成功返回“ok”, 失败返回错误信息
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
