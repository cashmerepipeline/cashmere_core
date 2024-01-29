//
//  Generated code. Do not modify.
//  source: category.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

import 'name.pb.dart' as $0;

class NewCategoryRequest extends $pb.GeneratedMessage {
  factory NewCategoryRequest({
    $core.String? manageId,
    $0.Name? name,
    $core.String? description,
    $core.String? code,
  }) {
    final $result = create();
    if (manageId != null) {
      $result.manageId = manageId;
    }
    if (name != null) {
      $result.name = name;
    }
    if (description != null) {
      $result.description = description;
    }
    if (code != null) {
      $result.code = code;
    }
    return $result;
  }
  NewCategoryRequest._() : super();
  factory NewCategoryRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory NewCategoryRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'NewCategoryRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'manageId')
    ..aOM<$0.Name>(2, _omitFieldNames ? '' : 'name', subBuilder: $0.Name.create)
    ..aOS(3, _omitFieldNames ? '' : 'description')
    ..aOS(4, _omitFieldNames ? '' : 'code')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  NewCategoryRequest clone() => NewCategoryRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  NewCategoryRequest copyWith(void Function(NewCategoryRequest) updates) => super.copyWith((message) => updates(message as NewCategoryRequest)) as NewCategoryRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static NewCategoryRequest create() => NewCategoryRequest._();
  NewCategoryRequest createEmptyInstance() => create();
  static $pb.PbList<NewCategoryRequest> createRepeated() => $pb.PbList<NewCategoryRequest>();
  @$core.pragma('dart2js:noInline')
  static NewCategoryRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<NewCategoryRequest>(create);
  static NewCategoryRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get manageId => $_getSZ(0);
  @$pb.TagNumber(1)
  set manageId($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasManageId() => $_has(0);
  @$pb.TagNumber(1)
  void clearManageId() => clearField(1);

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

  @$pb.TagNumber(4)
  $core.String get code => $_getSZ(3);
  @$pb.TagNumber(4)
  set code($core.String v) { $_setString(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasCode() => $_has(3);
  @$pb.TagNumber(4)
  void clearCode() => clearField(4);
}

class NewCategoryResponse extends $pb.GeneratedMessage {
  factory NewCategoryResponse({
    $core.String? result,
  }) {
    final $result = create();
    if (result != null) {
      $result.result = result;
    }
    return $result;
  }
  NewCategoryResponse._() : super();
  factory NewCategoryResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory NewCategoryResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'NewCategoryResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'result')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  NewCategoryResponse clone() => NewCategoryResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  NewCategoryResponse copyWith(void Function(NewCategoryResponse) updates) => super.copyWith((message) => updates(message as NewCategoryResponse)) as NewCategoryResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static NewCategoryResponse create() => NewCategoryResponse._();
  NewCategoryResponse createEmptyInstance() => create();
  static $pb.PbList<NewCategoryResponse> createRepeated() => $pb.PbList<NewCategoryResponse>();
  @$core.pragma('dart2js:noInline')
  static NewCategoryResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<NewCategoryResponse>(create);
  static NewCategoryResponse? _defaultInstance;

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

class GetCategoriesRequest extends $pb.GeneratedMessage {
  factory GetCategoriesRequest({
    $core.String? manageId,
  }) {
    final $result = create();
    if (manageId != null) {
      $result.manageId = manageId;
    }
    return $result;
  }
  GetCategoriesRequest._() : super();
  factory GetCategoriesRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetCategoriesRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'GetCategoriesRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'manageId')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetCategoriesRequest clone() => GetCategoriesRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetCategoriesRequest copyWith(void Function(GetCategoriesRequest) updates) => super.copyWith((message) => updates(message as GetCategoriesRequest)) as GetCategoriesRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GetCategoriesRequest create() => GetCategoriesRequest._();
  GetCategoriesRequest createEmptyInstance() => create();
  static $pb.PbList<GetCategoriesRequest> createRepeated() => $pb.PbList<GetCategoriesRequest>();
  @$core.pragma('dart2js:noInline')
  static GetCategoriesRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetCategoriesRequest>(create);
  static GetCategoriesRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get manageId => $_getSZ(0);
  @$pb.TagNumber(1)
  set manageId($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasManageId() => $_has(0);
  @$pb.TagNumber(1)
  void clearManageId() => clearField(1);
}

class GetCategoriesResponse extends $pb.GeneratedMessage {
  factory GetCategoriesResponse({
    $core.Iterable<$core.List<$core.int>>? codes,
  }) {
    final $result = create();
    if (codes != null) {
      $result.codes.addAll(codes);
    }
    return $result;
  }
  GetCategoriesResponse._() : super();
  factory GetCategoriesResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetCategoriesResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'GetCategoriesResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..p<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'codes', $pb.PbFieldType.PY)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetCategoriesResponse clone() => GetCategoriesResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetCategoriesResponse copyWith(void Function(GetCategoriesResponse) updates) => super.copyWith((message) => updates(message as GetCategoriesResponse)) as GetCategoriesResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GetCategoriesResponse create() => GetCategoriesResponse._();
  GetCategoriesResponse createEmptyInstance() => create();
  static $pb.PbList<GetCategoriesResponse> createRepeated() => $pb.PbList<GetCategoriesResponse>();
  @$core.pragma('dart2js:noInline')
  static GetCategoriesResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetCategoriesResponse>(create);
  static GetCategoriesResponse? _defaultInstance;

  /// bson bytes
  @$pb.TagNumber(1)
  $core.List<$core.List<$core.int>> get codes => $_getList(0);
}

/// 标记实体到类, 将品类编号添加到实体的品类列表中
class MarkEntityCategoriesRequest extends $pb.GeneratedMessage {
  factory MarkEntityCategoriesRequest({
    $core.String? manageId,
    $core.String? entityId,
    $core.Iterable<$core.String>? categoryIds,
  }) {
    final $result = create();
    if (manageId != null) {
      $result.manageId = manageId;
    }
    if (entityId != null) {
      $result.entityId = entityId;
    }
    if (categoryIds != null) {
      $result.categoryIds.addAll(categoryIds);
    }
    return $result;
  }
  MarkEntityCategoriesRequest._() : super();
  factory MarkEntityCategoriesRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory MarkEntityCategoriesRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'MarkEntityCategoriesRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(2, _omitFieldNames ? '' : 'manageId')
    ..aOS(3, _omitFieldNames ? '' : 'entityId')
    ..pPS(4, _omitFieldNames ? '' : 'categoryIds')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  MarkEntityCategoriesRequest clone() => MarkEntityCategoriesRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  MarkEntityCategoriesRequest copyWith(void Function(MarkEntityCategoriesRequest) updates) => super.copyWith((message) => updates(message as MarkEntityCategoriesRequest)) as MarkEntityCategoriesRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static MarkEntityCategoriesRequest create() => MarkEntityCategoriesRequest._();
  MarkEntityCategoriesRequest createEmptyInstance() => create();
  static $pb.PbList<MarkEntityCategoriesRequest> createRepeated() => $pb.PbList<MarkEntityCategoriesRequest>();
  @$core.pragma('dart2js:noInline')
  static MarkEntityCategoriesRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<MarkEntityCategoriesRequest>(create);
  static MarkEntityCategoriesRequest? _defaultInstance;

  @$pb.TagNumber(2)
  $core.String get manageId => $_getSZ(0);
  @$pb.TagNumber(2)
  set manageId($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(2)
  $core.bool hasManageId() => $_has(0);
  @$pb.TagNumber(2)
  void clearManageId() => clearField(2);

  @$pb.TagNumber(3)
  $core.String get entityId => $_getSZ(1);
  @$pb.TagNumber(3)
  set entityId($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(3)
  $core.bool hasEntityId() => $_has(1);
  @$pb.TagNumber(3)
  void clearEntityId() => clearField(3);

  @$pb.TagNumber(4)
  $core.List<$core.String> get categoryIds => $_getList(2);
}

class MarkEntityCategoriesResponse extends $pb.GeneratedMessage {
  factory MarkEntityCategoriesResponse({
    $core.String? result,
  }) {
    final $result = create();
    if (result != null) {
      $result.result = result;
    }
    return $result;
  }
  MarkEntityCategoriesResponse._() : super();
  factory MarkEntityCategoriesResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory MarkEntityCategoriesResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'MarkEntityCategoriesResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'result')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  MarkEntityCategoriesResponse clone() => MarkEntityCategoriesResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  MarkEntityCategoriesResponse copyWith(void Function(MarkEntityCategoriesResponse) updates) => super.copyWith((message) => updates(message as MarkEntityCategoriesResponse)) as MarkEntityCategoriesResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static MarkEntityCategoriesResponse create() => MarkEntityCategoriesResponse._();
  MarkEntityCategoriesResponse createEmptyInstance() => create();
  static $pb.PbList<MarkEntityCategoriesResponse> createRepeated() => $pb.PbList<MarkEntityCategoriesResponse>();
  @$core.pragma('dart2js:noInline')
  static MarkEntityCategoriesResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<MarkEntityCategoriesResponse>(create);
  static MarkEntityCategoriesResponse? _defaultInstance;

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

/// 取消标记品类，将品类从实体品类列表中删除
class UnmarkEntityCategoriesRequest extends $pb.GeneratedMessage {
  factory UnmarkEntityCategoriesRequest({
    $core.String? manageId,
    $core.String? entityId,
    $core.Iterable<$core.String>? categoryIds,
  }) {
    final $result = create();
    if (manageId != null) {
      $result.manageId = manageId;
    }
    if (entityId != null) {
      $result.entityId = entityId;
    }
    if (categoryIds != null) {
      $result.categoryIds.addAll(categoryIds);
    }
    return $result;
  }
  UnmarkEntityCategoriesRequest._() : super();
  factory UnmarkEntityCategoriesRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory UnmarkEntityCategoriesRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'UnmarkEntityCategoriesRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(2, _omitFieldNames ? '' : 'manageId')
    ..aOS(3, _omitFieldNames ? '' : 'entityId')
    ..pPS(4, _omitFieldNames ? '' : 'categoryIds')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  UnmarkEntityCategoriesRequest clone() => UnmarkEntityCategoriesRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  UnmarkEntityCategoriesRequest copyWith(void Function(UnmarkEntityCategoriesRequest) updates) => super.copyWith((message) => updates(message as UnmarkEntityCategoriesRequest)) as UnmarkEntityCategoriesRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static UnmarkEntityCategoriesRequest create() => UnmarkEntityCategoriesRequest._();
  UnmarkEntityCategoriesRequest createEmptyInstance() => create();
  static $pb.PbList<UnmarkEntityCategoriesRequest> createRepeated() => $pb.PbList<UnmarkEntityCategoriesRequest>();
  @$core.pragma('dart2js:noInline')
  static UnmarkEntityCategoriesRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<UnmarkEntityCategoriesRequest>(create);
  static UnmarkEntityCategoriesRequest? _defaultInstance;

  @$pb.TagNumber(2)
  $core.String get manageId => $_getSZ(0);
  @$pb.TagNumber(2)
  set manageId($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(2)
  $core.bool hasManageId() => $_has(0);
  @$pb.TagNumber(2)
  void clearManageId() => clearField(2);

  @$pb.TagNumber(3)
  $core.String get entityId => $_getSZ(1);
  @$pb.TagNumber(3)
  set entityId($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(3)
  $core.bool hasEntityId() => $_has(1);
  @$pb.TagNumber(3)
  void clearEntityId() => clearField(3);

  @$pb.TagNumber(4)
  $core.List<$core.String> get categoryIds => $_getList(2);
}

class UnmarkEntityCategoriesResponse extends $pb.GeneratedMessage {
  factory UnmarkEntityCategoriesResponse({
    $core.String? result,
  }) {
    final $result = create();
    if (result != null) {
      $result.result = result;
    }
    return $result;
  }
  UnmarkEntityCategoriesResponse._() : super();
  factory UnmarkEntityCategoriesResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory UnmarkEntityCategoriesResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'UnmarkEntityCategoriesResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'result')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  UnmarkEntityCategoriesResponse clone() => UnmarkEntityCategoriesResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  UnmarkEntityCategoriesResponse copyWith(void Function(UnmarkEntityCategoriesResponse) updates) => super.copyWith((message) => updates(message as UnmarkEntityCategoriesResponse)) as UnmarkEntityCategoriesResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static UnmarkEntityCategoriesResponse create() => UnmarkEntityCategoriesResponse._();
  UnmarkEntityCategoriesResponse createEmptyInstance() => create();
  static $pb.PbList<UnmarkEntityCategoriesResponse> createRepeated() => $pb.PbList<UnmarkEntityCategoriesResponse>();
  @$core.pragma('dart2js:noInline')
  static UnmarkEntityCategoriesResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<UnmarkEntityCategoriesResponse>(create);
  static UnmarkEntityCategoriesResponse? _defaultInstance;

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
