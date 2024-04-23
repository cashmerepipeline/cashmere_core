//
//  Generated code. Do not modify.
//  source: recommend.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

/// / 切换推荐状态
class ToggleRecommendRequest extends $pb.GeneratedMessage {
  factory ToggleRecommendRequest({
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
  ToggleRecommendRequest._() : super();
  factory ToggleRecommendRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ToggleRecommendRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'ToggleRecommendRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'manageId')
    ..aOS(2, _omitFieldNames ? '' : 'entityId')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ToggleRecommendRequest clone() => ToggleRecommendRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ToggleRecommendRequest copyWith(void Function(ToggleRecommendRequest) updates) => super.copyWith((message) => updates(message as ToggleRecommendRequest)) as ToggleRecommendRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ToggleRecommendRequest create() => ToggleRecommendRequest._();
  ToggleRecommendRequest createEmptyInstance() => create();
  static $pb.PbList<ToggleRecommendRequest> createRepeated() => $pb.PbList<ToggleRecommendRequest>();
  @$core.pragma('dart2js:noInline')
  static ToggleRecommendRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ToggleRecommendRequest>(create);
  static ToggleRecommendRequest? _defaultInstance;

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

class ToggleRecommendResponse extends $pb.GeneratedMessage {
  factory ToggleRecommendResponse({
    $core.bool? result,
  }) {
    final $result = create();
    if (result != null) {
      $result.result = result;
    }
    return $result;
  }
  ToggleRecommendResponse._() : super();
  factory ToggleRecommendResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ToggleRecommendResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'ToggleRecommendResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOB(1, _omitFieldNames ? '' : 'result')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ToggleRecommendResponse clone() => ToggleRecommendResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ToggleRecommendResponse copyWith(void Function(ToggleRecommendResponse) updates) => super.copyWith((message) => updates(message as ToggleRecommendResponse)) as ToggleRecommendResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ToggleRecommendResponse create() => ToggleRecommendResponse._();
  ToggleRecommendResponse createEmptyInstance() => create();
  static $pb.PbList<ToggleRecommendResponse> createRepeated() => $pb.PbList<ToggleRecommendResponse>();
  @$core.pragma('dart2js:noInline')
  static ToggleRecommendResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ToggleRecommendResponse>(create);
  static ToggleRecommendResponse? _defaultInstance;

  /// 返回推荐状态
  @$pb.TagNumber(1)
  $core.bool get result => $_getBF(0);
  @$pb.TagNumber(1)
  set result($core.bool v) { $_setBool(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasResult() => $_has(0);
  @$pb.TagNumber(1)
  void clearResult() => clearField(1);
}

/// / zh: 获取最多推荐，最多1000个
class GetTopRecommendsRequest extends $pb.GeneratedMessage {
  factory GetTopRecommendsRequest({
    $core.String? manageId,
    $core.int? count,
  }) {
    final $result = create();
    if (manageId != null) {
      $result.manageId = manageId;
    }
    if (count != null) {
      $result.count = count;
    }
    return $result;
  }
  GetTopRecommendsRequest._() : super();
  factory GetTopRecommendsRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetTopRecommendsRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'GetTopRecommendsRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'manageId')
    ..a<$core.int>(3, _omitFieldNames ? '' : 'count', $pb.PbFieldType.O3)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetTopRecommendsRequest clone() => GetTopRecommendsRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetTopRecommendsRequest copyWith(void Function(GetTopRecommendsRequest) updates) => super.copyWith((message) => updates(message as GetTopRecommendsRequest)) as GetTopRecommendsRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GetTopRecommendsRequest create() => GetTopRecommendsRequest._();
  GetTopRecommendsRequest createEmptyInstance() => create();
  static $pb.PbList<GetTopRecommendsRequest> createRepeated() => $pb.PbList<GetTopRecommendsRequest>();
  @$core.pragma('dart2js:noInline')
  static GetTopRecommendsRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetTopRecommendsRequest>(create);
  static GetTopRecommendsRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get manageId => $_getSZ(0);
  @$pb.TagNumber(1)
  set manageId($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasManageId() => $_has(0);
  @$pb.TagNumber(1)
  void clearManageId() => clearField(1);

  /// 数量，最多1000个
  @$pb.TagNumber(3)
  $core.int get count => $_getIZ(1);
  @$pb.TagNumber(3)
  set count($core.int v) { $_setSignedInt32(1, v); }
  @$pb.TagNumber(3)
  $core.bool hasCount() => $_has(1);
  @$pb.TagNumber(3)
  void clearCount() => clearField(3);
}

class GetTopRecommendsResponse extends $pb.GeneratedMessage {
  factory GetTopRecommendsResponse({
    $core.Iterable<$core.List<$core.int>>? recommends,
  }) {
    final $result = create();
    if (recommends != null) {
      $result.recommends.addAll(recommends);
    }
    return $result;
  }
  GetTopRecommendsResponse._() : super();
  factory GetTopRecommendsResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetTopRecommendsResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'GetTopRecommendsResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..p<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'recommends', $pb.PbFieldType.PY)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetTopRecommendsResponse clone() => GetTopRecommendsResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetTopRecommendsResponse copyWith(void Function(GetTopRecommendsResponse) updates) => super.copyWith((message) => updates(message as GetTopRecommendsResponse)) as GetTopRecommendsResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GetTopRecommendsResponse create() => GetTopRecommendsResponse._();
  GetTopRecommendsResponse createEmptyInstance() => create();
  static $pb.PbList<GetTopRecommendsResponse> createRepeated() => $pb.PbList<GetTopRecommendsResponse>();
  @$core.pragma('dart2js:noInline')
  static GetTopRecommendsResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetTopRecommendsResponse>(create);
  static GetTopRecommendsResponse? _defaultInstance;

  /// {id: count}表
  @$pb.TagNumber(1)
  $core.List<$core.List<$core.int>> get recommends => $_getList(0);
}

/// / 取得推荐帐号表
class GetRecommendAccountsRequest extends $pb.GeneratedMessage {
  factory GetRecommendAccountsRequest({
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
  GetRecommendAccountsRequest._() : super();
  factory GetRecommendAccountsRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetRecommendAccountsRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'GetRecommendAccountsRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'manageId')
    ..aOS(2, _omitFieldNames ? '' : 'entityId')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetRecommendAccountsRequest clone() => GetRecommendAccountsRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetRecommendAccountsRequest copyWith(void Function(GetRecommendAccountsRequest) updates) => super.copyWith((message) => updates(message as GetRecommendAccountsRequest)) as GetRecommendAccountsRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GetRecommendAccountsRequest create() => GetRecommendAccountsRequest._();
  GetRecommendAccountsRequest createEmptyInstance() => create();
  static $pb.PbList<GetRecommendAccountsRequest> createRepeated() => $pb.PbList<GetRecommendAccountsRequest>();
  @$core.pragma('dart2js:noInline')
  static GetRecommendAccountsRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetRecommendAccountsRequest>(create);
  static GetRecommendAccountsRequest? _defaultInstance;

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

class GetRecommendAccountsResponse extends $pb.GeneratedMessage {
  factory GetRecommendAccountsResponse({
    $core.Iterable<$core.String>? accounts,
  }) {
    final $result = create();
    if (accounts != null) {
      $result.accounts.addAll(accounts);
    }
    return $result;
  }
  GetRecommendAccountsResponse._() : super();
  factory GetRecommendAccountsResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetRecommendAccountsResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'GetRecommendAccountsResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..pPS(1, _omitFieldNames ? '' : 'accounts')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetRecommendAccountsResponse clone() => GetRecommendAccountsResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetRecommendAccountsResponse copyWith(void Function(GetRecommendAccountsResponse) updates) => super.copyWith((message) => updates(message as GetRecommendAccountsResponse)) as GetRecommendAccountsResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GetRecommendAccountsResponse create() => GetRecommendAccountsResponse._();
  GetRecommendAccountsResponse createEmptyInstance() => create();
  static $pb.PbList<GetRecommendAccountsResponse> createRepeated() => $pb.PbList<GetRecommendAccountsResponse>();
  @$core.pragma('dart2js:noInline')
  static GetRecommendAccountsResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetRecommendAccountsResponse>(create);
  static GetRecommendAccountsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.String> get accounts => $_getList(0);
}

/// / 取得帐号推荐了的实体列表, 只对当前帐号有效
class GetAccountRecommendedEntitiesRequest extends $pb.GeneratedMessage {
  factory GetAccountRecommendedEntitiesRequest({
    $core.String? manageId,
  }) {
    final $result = create();
    if (manageId != null) {
      $result.manageId = manageId;
    }
    return $result;
  }
  GetAccountRecommendedEntitiesRequest._() : super();
  factory GetAccountRecommendedEntitiesRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetAccountRecommendedEntitiesRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'GetAccountRecommendedEntitiesRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'manageId')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetAccountRecommendedEntitiesRequest clone() => GetAccountRecommendedEntitiesRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetAccountRecommendedEntitiesRequest copyWith(void Function(GetAccountRecommendedEntitiesRequest) updates) => super.copyWith((message) => updates(message as GetAccountRecommendedEntitiesRequest)) as GetAccountRecommendedEntitiesRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GetAccountRecommendedEntitiesRequest create() => GetAccountRecommendedEntitiesRequest._();
  GetAccountRecommendedEntitiesRequest createEmptyInstance() => create();
  static $pb.PbList<GetAccountRecommendedEntitiesRequest> createRepeated() => $pb.PbList<GetAccountRecommendedEntitiesRequest>();
  @$core.pragma('dart2js:noInline')
  static GetAccountRecommendedEntitiesRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetAccountRecommendedEntitiesRequest>(create);
  static GetAccountRecommendedEntitiesRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get manageId => $_getSZ(0);
  @$pb.TagNumber(1)
  set manageId($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasManageId() => $_has(0);
  @$pb.TagNumber(1)
  void clearManageId() => clearField(1);
}

class GetAccountRecommendedEntitiesResponse extends $pb.GeneratedMessage {
  factory GetAccountRecommendedEntitiesResponse({
    $core.Iterable<$core.String>? entities,
  }) {
    final $result = create();
    if (entities != null) {
      $result.entities.addAll(entities);
    }
    return $result;
  }
  GetAccountRecommendedEntitiesResponse._() : super();
  factory GetAccountRecommendedEntitiesResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetAccountRecommendedEntitiesResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'GetAccountRecommendedEntitiesResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..pPS(1, _omitFieldNames ? '' : 'entities')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetAccountRecommendedEntitiesResponse clone() => GetAccountRecommendedEntitiesResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetAccountRecommendedEntitiesResponse copyWith(void Function(GetAccountRecommendedEntitiesResponse) updates) => super.copyWith((message) => updates(message as GetAccountRecommendedEntitiesResponse)) as GetAccountRecommendedEntitiesResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GetAccountRecommendedEntitiesResponse create() => GetAccountRecommendedEntitiesResponse._();
  GetAccountRecommendedEntitiesResponse createEmptyInstance() => create();
  static $pb.PbList<GetAccountRecommendedEntitiesResponse> createRepeated() => $pb.PbList<GetAccountRecommendedEntitiesResponse>();
  @$core.pragma('dart2js:noInline')
  static GetAccountRecommendedEntitiesResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetAccountRecommendedEntitiesResponse>(create);
  static GetAccountRecommendedEntitiesResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.String> get entities => $_getList(0);
}


const _omitFieldNames = $core.bool.fromEnvironment('protobuf.omit_field_names');
const _omitMessageNames = $core.bool.fromEnvironment('protobuf.omit_message_names');
