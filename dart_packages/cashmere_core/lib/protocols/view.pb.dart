//
//  Generated code. Do not modify.
//  source: view.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

/// 映像请求
class GetManageViewRequest extends $pb.GeneratedMessage {
  factory GetManageViewRequest({
    $core.String? manageName,
  }) {
    final $result = create();
    if (manageName != null) {
      $result.manageName = manageName;
    }
    return $result;
  }
  GetManageViewRequest._() : super();
  factory GetManageViewRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetManageViewRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'GetManageViewRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'manageName')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetManageViewRequest clone() => GetManageViewRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetManageViewRequest copyWith(void Function(GetManageViewRequest) updates) => super.copyWith((message) => updates(message as GetManageViewRequest)) as GetManageViewRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GetManageViewRequest create() => GetManageViewRequest._();
  GetManageViewRequest createEmptyInstance() => create();
  static $pb.PbList<GetManageViewRequest> createRepeated() => $pb.PbList<GetManageViewRequest>();
  @$core.pragma('dart2js:noInline')
  static GetManageViewRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetManageViewRequest>(create);
  static GetManageViewRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get manageName => $_getSZ(0);
  @$pb.TagNumber(1)
  set manageName($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasManageName() => $_has(0);
  @$pb.TagNumber(1)
  void clearManageName() => clearField(1);
}

/// 映像返回
class GetManageViewResponse extends $pb.GeneratedMessage {
  factory GetManageViewResponse({
    $core.String? viewToken,
  }) {
    final $result = create();
    if (viewToken != null) {
      $result.viewToken = viewToken;
    }
    return $result;
  }
  GetManageViewResponse._() : super();
  factory GetManageViewResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetManageViewResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'GetManageViewResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'viewToken')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetManageViewResponse clone() => GetManageViewResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetManageViewResponse copyWith(void Function(GetManageViewResponse) updates) => super.copyWith((message) => updates(message as GetManageViewResponse)) as GetManageViewResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GetManageViewResponse create() => GetManageViewResponse._();
  GetManageViewResponse createEmptyInstance() => create();
  static $pb.PbList<GetManageViewResponse> createRepeated() => $pb.PbList<GetManageViewResponse>();
  @$core.pragma('dart2js:noInline')
  static GetManageViewResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetManageViewResponse>(create);
  static GetManageViewResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get viewToken => $_getSZ(0);
  @$pb.TagNumber(1)
  set viewToken($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasViewToken() => $_has(0);
  @$pb.TagNumber(1)
  void clearViewToken() => clearField(1);
}

/// 取得管理模式可视表
class GetSchemaViewRulesMapRequest extends $pb.GeneratedMessage {
  factory GetSchemaViewRulesMapRequest({
    $core.String? manageId,
    $core.String? groupId,
  }) {
    final $result = create();
    if (manageId != null) {
      $result.manageId = manageId;
    }
    if (groupId != null) {
      $result.groupId = groupId;
    }
    return $result;
  }
  GetSchemaViewRulesMapRequest._() : super();
  factory GetSchemaViewRulesMapRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetSchemaViewRulesMapRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'GetSchemaViewRulesMapRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'manageId')
    ..aOS(2, _omitFieldNames ? '' : 'groupId')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetSchemaViewRulesMapRequest clone() => GetSchemaViewRulesMapRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetSchemaViewRulesMapRequest copyWith(void Function(GetSchemaViewRulesMapRequest) updates) => super.copyWith((message) => updates(message as GetSchemaViewRulesMapRequest)) as GetSchemaViewRulesMapRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GetSchemaViewRulesMapRequest create() => GetSchemaViewRulesMapRequest._();
  GetSchemaViewRulesMapRequest createEmptyInstance() => create();
  static $pb.PbList<GetSchemaViewRulesMapRequest> createRepeated() => $pb.PbList<GetSchemaViewRulesMapRequest>();
  @$core.pragma('dart2js:noInline')
  static GetSchemaViewRulesMapRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetSchemaViewRulesMapRequest>(create);
  static GetSchemaViewRulesMapRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get manageId => $_getSZ(0);
  @$pb.TagNumber(1)
  set manageId($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasManageId() => $_has(0);
  @$pb.TagNumber(1)
  void clearManageId() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get groupId => $_getSZ(1);
  @$pb.TagNumber(2)
  set groupId($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasGroupId() => $_has(1);
  @$pb.TagNumber(2)
  void clearGroupId() => clearField(2);
}

class GetSchemaViewRulesMapResponse extends $pb.GeneratedMessage {
  factory GetSchemaViewRulesMapResponse({
    $core.List<$core.int>? rulesMap,
  }) {
    final $result = create();
    if (rulesMap != null) {
      $result.rulesMap = rulesMap;
    }
    return $result;
  }
  GetSchemaViewRulesMapResponse._() : super();
  factory GetSchemaViewRulesMapResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetSchemaViewRulesMapResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'GetSchemaViewRulesMapResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'rulesMap', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetSchemaViewRulesMapResponse clone() => GetSchemaViewRulesMapResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetSchemaViewRulesMapResponse copyWith(void Function(GetSchemaViewRulesMapResponse) updates) => super.copyWith((message) => updates(message as GetSchemaViewRulesMapResponse)) as GetSchemaViewRulesMapResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GetSchemaViewRulesMapResponse create() => GetSchemaViewRulesMapResponse._();
  GetSchemaViewRulesMapResponse createEmptyInstance() => create();
  static $pb.PbList<GetSchemaViewRulesMapResponse> createRepeated() => $pb.PbList<GetSchemaViewRulesMapResponse>();
  @$core.pragma('dart2js:noInline')
  static GetSchemaViewRulesMapResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetSchemaViewRulesMapResponse>(create);
  static GetSchemaViewRulesMapResponse? _defaultInstance;

  /// bson document
  @$pb.TagNumber(1)
  $core.List<$core.int> get rulesMap => $_getN(0);
  @$pb.TagNumber(1)
  set rulesMap($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasRulesMap() => $_has(0);
  @$pb.TagNumber(1)
  void clearRulesMap() => clearField(1);
}

/// 管理权限
class ChangeManageReadRuleRequest extends $pb.GeneratedMessage {
  factory ChangeManageReadRuleRequest({
    $core.String? manageId,
    $core.String? groupId,
    $core.String? readRule,
  }) {
    final $result = create();
    if (manageId != null) {
      $result.manageId = manageId;
    }
    if (groupId != null) {
      $result.groupId = groupId;
    }
    if (readRule != null) {
      $result.readRule = readRule;
    }
    return $result;
  }
  ChangeManageReadRuleRequest._() : super();
  factory ChangeManageReadRuleRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChangeManageReadRuleRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'ChangeManageReadRuleRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'manageId')
    ..aOS(2, _omitFieldNames ? '' : 'groupId')
    ..aOS(3, _omitFieldNames ? '' : 'readRule')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ChangeManageReadRuleRequest clone() => ChangeManageReadRuleRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ChangeManageReadRuleRequest copyWith(void Function(ChangeManageReadRuleRequest) updates) => super.copyWith((message) => updates(message as ChangeManageReadRuleRequest)) as ChangeManageReadRuleRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ChangeManageReadRuleRequest create() => ChangeManageReadRuleRequest._();
  ChangeManageReadRuleRequest createEmptyInstance() => create();
  static $pb.PbList<ChangeManageReadRuleRequest> createRepeated() => $pb.PbList<ChangeManageReadRuleRequest>();
  @$core.pragma('dart2js:noInline')
  static ChangeManageReadRuleRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChangeManageReadRuleRequest>(create);
  static ChangeManageReadRuleRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get manageId => $_getSZ(0);
  @$pb.TagNumber(1)
  set manageId($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasManageId() => $_has(0);
  @$pb.TagNumber(1)
  void clearManageId() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get groupId => $_getSZ(1);
  @$pb.TagNumber(2)
  set groupId($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasGroupId() => $_has(1);
  @$pb.TagNumber(2)
  void clearGroupId() => clearField(2);

  @$pb.TagNumber(3)
  $core.String get readRule => $_getSZ(2);
  @$pb.TagNumber(3)
  set readRule($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasReadRule() => $_has(2);
  @$pb.TagNumber(3)
  void clearReadRule() => clearField(3);
}

class ChangeManageReadRuleResponse extends $pb.GeneratedMessage {
  factory ChangeManageReadRuleResponse({
    $core.String? result,
  }) {
    final $result = create();
    if (result != null) {
      $result.result = result;
    }
    return $result;
  }
  ChangeManageReadRuleResponse._() : super();
  factory ChangeManageReadRuleResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChangeManageReadRuleResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'ChangeManageReadRuleResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'result')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ChangeManageReadRuleResponse clone() => ChangeManageReadRuleResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ChangeManageReadRuleResponse copyWith(void Function(ChangeManageReadRuleResponse) updates) => super.copyWith((message) => updates(message as ChangeManageReadRuleResponse)) as ChangeManageReadRuleResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ChangeManageReadRuleResponse create() => ChangeManageReadRuleResponse._();
  ChangeManageReadRuleResponse createEmptyInstance() => create();
  static $pb.PbList<ChangeManageReadRuleResponse> createRepeated() => $pb.PbList<ChangeManageReadRuleResponse>();
  @$core.pragma('dart2js:noInline')
  static ChangeManageReadRuleResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChangeManageReadRuleResponse>(create);
  static ChangeManageReadRuleResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get result => $_getSZ(0);
  @$pb.TagNumber(1)
  set result($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasResult() => $_has(0);
  @$pb.TagNumber(1)
  void clearResult() => clearField(1);
}

class ChangeManageWriteRuleRequest extends $pb.GeneratedMessage {
  factory ChangeManageWriteRuleRequest({
    $core.String? manageId,
    $core.String? groupId,
    $core.String? writeRule,
  }) {
    final $result = create();
    if (manageId != null) {
      $result.manageId = manageId;
    }
    if (groupId != null) {
      $result.groupId = groupId;
    }
    if (writeRule != null) {
      $result.writeRule = writeRule;
    }
    return $result;
  }
  ChangeManageWriteRuleRequest._() : super();
  factory ChangeManageWriteRuleRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChangeManageWriteRuleRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'ChangeManageWriteRuleRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'manageId')
    ..aOS(2, _omitFieldNames ? '' : 'groupId')
    ..aOS(3, _omitFieldNames ? '' : 'writeRule')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ChangeManageWriteRuleRequest clone() => ChangeManageWriteRuleRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ChangeManageWriteRuleRequest copyWith(void Function(ChangeManageWriteRuleRequest) updates) => super.copyWith((message) => updates(message as ChangeManageWriteRuleRequest)) as ChangeManageWriteRuleRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ChangeManageWriteRuleRequest create() => ChangeManageWriteRuleRequest._();
  ChangeManageWriteRuleRequest createEmptyInstance() => create();
  static $pb.PbList<ChangeManageWriteRuleRequest> createRepeated() => $pb.PbList<ChangeManageWriteRuleRequest>();
  @$core.pragma('dart2js:noInline')
  static ChangeManageWriteRuleRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChangeManageWriteRuleRequest>(create);
  static ChangeManageWriteRuleRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get manageId => $_getSZ(0);
  @$pb.TagNumber(1)
  set manageId($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasManageId() => $_has(0);
  @$pb.TagNumber(1)
  void clearManageId() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get groupId => $_getSZ(1);
  @$pb.TagNumber(2)
  set groupId($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasGroupId() => $_has(1);
  @$pb.TagNumber(2)
  void clearGroupId() => clearField(2);

  @$pb.TagNumber(3)
  $core.String get writeRule => $_getSZ(2);
  @$pb.TagNumber(3)
  set writeRule($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasWriteRule() => $_has(2);
  @$pb.TagNumber(3)
  void clearWriteRule() => clearField(3);
}

class ChangeManageWriteRuleResponse extends $pb.GeneratedMessage {
  factory ChangeManageWriteRuleResponse({
    $core.String? result,
  }) {
    final $result = create();
    if (result != null) {
      $result.result = result;
    }
    return $result;
  }
  ChangeManageWriteRuleResponse._() : super();
  factory ChangeManageWriteRuleResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChangeManageWriteRuleResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'ChangeManageWriteRuleResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'result')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ChangeManageWriteRuleResponse clone() => ChangeManageWriteRuleResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ChangeManageWriteRuleResponse copyWith(void Function(ChangeManageWriteRuleResponse) updates) => super.copyWith((message) => updates(message as ChangeManageWriteRuleResponse)) as ChangeManageWriteRuleResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ChangeManageWriteRuleResponse create() => ChangeManageWriteRuleResponse._();
  ChangeManageWriteRuleResponse createEmptyInstance() => create();
  static $pb.PbList<ChangeManageWriteRuleResponse> createRepeated() => $pb.PbList<ChangeManageWriteRuleResponse>();
  @$core.pragma('dart2js:noInline')
  static ChangeManageWriteRuleResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChangeManageWriteRuleResponse>(create);
  static ChangeManageWriteRuleResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get result => $_getSZ(0);
  @$pb.TagNumber(1)
  set result($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasResult() => $_has(0);
  @$pb.TagNumber(1)
  void clearResult() => clearField(1);
}

/// 集合权限
class ChangeCollectionReadRuleRequest extends $pb.GeneratedMessage {
  factory ChangeCollectionReadRuleRequest({
    $core.String? manageId,
    $core.String? groupId,
    $core.String? readRule,
  }) {
    final $result = create();
    if (manageId != null) {
      $result.manageId = manageId;
    }
    if (groupId != null) {
      $result.groupId = groupId;
    }
    if (readRule != null) {
      $result.readRule = readRule;
    }
    return $result;
  }
  ChangeCollectionReadRuleRequest._() : super();
  factory ChangeCollectionReadRuleRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChangeCollectionReadRuleRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'ChangeCollectionReadRuleRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'manageId')
    ..aOS(2, _omitFieldNames ? '' : 'groupId')
    ..aOS(3, _omitFieldNames ? '' : 'readRule')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ChangeCollectionReadRuleRequest clone() => ChangeCollectionReadRuleRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ChangeCollectionReadRuleRequest copyWith(void Function(ChangeCollectionReadRuleRequest) updates) => super.copyWith((message) => updates(message as ChangeCollectionReadRuleRequest)) as ChangeCollectionReadRuleRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ChangeCollectionReadRuleRequest create() => ChangeCollectionReadRuleRequest._();
  ChangeCollectionReadRuleRequest createEmptyInstance() => create();
  static $pb.PbList<ChangeCollectionReadRuleRequest> createRepeated() => $pb.PbList<ChangeCollectionReadRuleRequest>();
  @$core.pragma('dart2js:noInline')
  static ChangeCollectionReadRuleRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChangeCollectionReadRuleRequest>(create);
  static ChangeCollectionReadRuleRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get manageId => $_getSZ(0);
  @$pb.TagNumber(1)
  set manageId($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasManageId() => $_has(0);
  @$pb.TagNumber(1)
  void clearManageId() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get groupId => $_getSZ(1);
  @$pb.TagNumber(2)
  set groupId($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasGroupId() => $_has(1);
  @$pb.TagNumber(2)
  void clearGroupId() => clearField(2);

  @$pb.TagNumber(3)
  $core.String get readRule => $_getSZ(2);
  @$pb.TagNumber(3)
  set readRule($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasReadRule() => $_has(2);
  @$pb.TagNumber(3)
  void clearReadRule() => clearField(3);
}

class ChangeCollectionReadRuleResponse extends $pb.GeneratedMessage {
  factory ChangeCollectionReadRuleResponse({
    $core.String? result,
  }) {
    final $result = create();
    if (result != null) {
      $result.result = result;
    }
    return $result;
  }
  ChangeCollectionReadRuleResponse._() : super();
  factory ChangeCollectionReadRuleResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChangeCollectionReadRuleResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'ChangeCollectionReadRuleResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'result')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ChangeCollectionReadRuleResponse clone() => ChangeCollectionReadRuleResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ChangeCollectionReadRuleResponse copyWith(void Function(ChangeCollectionReadRuleResponse) updates) => super.copyWith((message) => updates(message as ChangeCollectionReadRuleResponse)) as ChangeCollectionReadRuleResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ChangeCollectionReadRuleResponse create() => ChangeCollectionReadRuleResponse._();
  ChangeCollectionReadRuleResponse createEmptyInstance() => create();
  static $pb.PbList<ChangeCollectionReadRuleResponse> createRepeated() => $pb.PbList<ChangeCollectionReadRuleResponse>();
  @$core.pragma('dart2js:noInline')
  static ChangeCollectionReadRuleResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChangeCollectionReadRuleResponse>(create);
  static ChangeCollectionReadRuleResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get result => $_getSZ(0);
  @$pb.TagNumber(1)
  set result($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasResult() => $_has(0);
  @$pb.TagNumber(1)
  void clearResult() => clearField(1);
}

class ChangeCollectionWriteRuleRequest extends $pb.GeneratedMessage {
  factory ChangeCollectionWriteRuleRequest({
    $core.String? manageId,
    $core.String? groupId,
    $core.String? writeRule,
  }) {
    final $result = create();
    if (manageId != null) {
      $result.manageId = manageId;
    }
    if (groupId != null) {
      $result.groupId = groupId;
    }
    if (writeRule != null) {
      $result.writeRule = writeRule;
    }
    return $result;
  }
  ChangeCollectionWriteRuleRequest._() : super();
  factory ChangeCollectionWriteRuleRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChangeCollectionWriteRuleRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'ChangeCollectionWriteRuleRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'manageId')
    ..aOS(2, _omitFieldNames ? '' : 'groupId')
    ..aOS(3, _omitFieldNames ? '' : 'writeRule')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ChangeCollectionWriteRuleRequest clone() => ChangeCollectionWriteRuleRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ChangeCollectionWriteRuleRequest copyWith(void Function(ChangeCollectionWriteRuleRequest) updates) => super.copyWith((message) => updates(message as ChangeCollectionWriteRuleRequest)) as ChangeCollectionWriteRuleRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ChangeCollectionWriteRuleRequest create() => ChangeCollectionWriteRuleRequest._();
  ChangeCollectionWriteRuleRequest createEmptyInstance() => create();
  static $pb.PbList<ChangeCollectionWriteRuleRequest> createRepeated() => $pb.PbList<ChangeCollectionWriteRuleRequest>();
  @$core.pragma('dart2js:noInline')
  static ChangeCollectionWriteRuleRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChangeCollectionWriteRuleRequest>(create);
  static ChangeCollectionWriteRuleRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get manageId => $_getSZ(0);
  @$pb.TagNumber(1)
  set manageId($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasManageId() => $_has(0);
  @$pb.TagNumber(1)
  void clearManageId() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get groupId => $_getSZ(1);
  @$pb.TagNumber(2)
  set groupId($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasGroupId() => $_has(1);
  @$pb.TagNumber(2)
  void clearGroupId() => clearField(2);

  @$pb.TagNumber(3)
  $core.String get writeRule => $_getSZ(2);
  @$pb.TagNumber(3)
  set writeRule($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasWriteRule() => $_has(2);
  @$pb.TagNumber(3)
  void clearWriteRule() => clearField(3);
}

class ChangeCollectionWriteRuleResponse extends $pb.GeneratedMessage {
  factory ChangeCollectionWriteRuleResponse({
    $core.String? result,
  }) {
    final $result = create();
    if (result != null) {
      $result.result = result;
    }
    return $result;
  }
  ChangeCollectionWriteRuleResponse._() : super();
  factory ChangeCollectionWriteRuleResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChangeCollectionWriteRuleResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'ChangeCollectionWriteRuleResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'result')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ChangeCollectionWriteRuleResponse clone() => ChangeCollectionWriteRuleResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ChangeCollectionWriteRuleResponse copyWith(void Function(ChangeCollectionWriteRuleResponse) updates) => super.copyWith((message) => updates(message as ChangeCollectionWriteRuleResponse)) as ChangeCollectionWriteRuleResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ChangeCollectionWriteRuleResponse create() => ChangeCollectionWriteRuleResponse._();
  ChangeCollectionWriteRuleResponse createEmptyInstance() => create();
  static $pb.PbList<ChangeCollectionWriteRuleResponse> createRepeated() => $pb.PbList<ChangeCollectionWriteRuleResponse>();
  @$core.pragma('dart2js:noInline')
  static ChangeCollectionWriteRuleResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChangeCollectionWriteRuleResponse>(create);
  static ChangeCollectionWriteRuleResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get result => $_getSZ(0);
  @$pb.TagNumber(1)
  set result($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasResult() => $_has(0);
  @$pb.TagNumber(1)
  void clearResult() => clearField(1);
}

/// 描写字段权限
class ChangeFieldReadRuleRequest extends $pb.GeneratedMessage {
  factory ChangeFieldReadRuleRequest({
    $core.String? manageId,
    $core.String? groupId,
    $core.String? fieldId,
    $core.String? readRule,
  }) {
    final $result = create();
    if (manageId != null) {
      $result.manageId = manageId;
    }
    if (groupId != null) {
      $result.groupId = groupId;
    }
    if (fieldId != null) {
      $result.fieldId = fieldId;
    }
    if (readRule != null) {
      $result.readRule = readRule;
    }
    return $result;
  }
  ChangeFieldReadRuleRequest._() : super();
  factory ChangeFieldReadRuleRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChangeFieldReadRuleRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'ChangeFieldReadRuleRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'manageId')
    ..aOS(2, _omitFieldNames ? '' : 'groupId')
    ..aOS(3, _omitFieldNames ? '' : 'fieldId')
    ..aOS(4, _omitFieldNames ? '' : 'readRule')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ChangeFieldReadRuleRequest clone() => ChangeFieldReadRuleRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ChangeFieldReadRuleRequest copyWith(void Function(ChangeFieldReadRuleRequest) updates) => super.copyWith((message) => updates(message as ChangeFieldReadRuleRequest)) as ChangeFieldReadRuleRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ChangeFieldReadRuleRequest create() => ChangeFieldReadRuleRequest._();
  ChangeFieldReadRuleRequest createEmptyInstance() => create();
  static $pb.PbList<ChangeFieldReadRuleRequest> createRepeated() => $pb.PbList<ChangeFieldReadRuleRequest>();
  @$core.pragma('dart2js:noInline')
  static ChangeFieldReadRuleRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChangeFieldReadRuleRequest>(create);
  static ChangeFieldReadRuleRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get manageId => $_getSZ(0);
  @$pb.TagNumber(1)
  set manageId($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasManageId() => $_has(0);
  @$pb.TagNumber(1)
  void clearManageId() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get groupId => $_getSZ(1);
  @$pb.TagNumber(2)
  set groupId($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasGroupId() => $_has(1);
  @$pb.TagNumber(2)
  void clearGroupId() => clearField(2);

  @$pb.TagNumber(3)
  $core.String get fieldId => $_getSZ(2);
  @$pb.TagNumber(3)
  set fieldId($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasFieldId() => $_has(2);
  @$pb.TagNumber(3)
  void clearFieldId() => clearField(3);

  @$pb.TagNumber(4)
  $core.String get readRule => $_getSZ(3);
  @$pb.TagNumber(4)
  set readRule($core.String v) { $_setString(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasReadRule() => $_has(3);
  @$pb.TagNumber(4)
  void clearReadRule() => clearField(4);
}

class ChangeFieldReadRuleResponse extends $pb.GeneratedMessage {
  factory ChangeFieldReadRuleResponse({
    $core.String? result,
  }) {
    final $result = create();
    if (result != null) {
      $result.result = result;
    }
    return $result;
  }
  ChangeFieldReadRuleResponse._() : super();
  factory ChangeFieldReadRuleResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChangeFieldReadRuleResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'ChangeFieldReadRuleResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'result')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ChangeFieldReadRuleResponse clone() => ChangeFieldReadRuleResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ChangeFieldReadRuleResponse copyWith(void Function(ChangeFieldReadRuleResponse) updates) => super.copyWith((message) => updates(message as ChangeFieldReadRuleResponse)) as ChangeFieldReadRuleResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ChangeFieldReadRuleResponse create() => ChangeFieldReadRuleResponse._();
  ChangeFieldReadRuleResponse createEmptyInstance() => create();
  static $pb.PbList<ChangeFieldReadRuleResponse> createRepeated() => $pb.PbList<ChangeFieldReadRuleResponse>();
  @$core.pragma('dart2js:noInline')
  static ChangeFieldReadRuleResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChangeFieldReadRuleResponse>(create);
  static ChangeFieldReadRuleResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get result => $_getSZ(0);
  @$pb.TagNumber(1)
  set result($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasResult() => $_has(0);
  @$pb.TagNumber(1)
  void clearResult() => clearField(1);
}

class ChangeFieldWriteRuleRequest extends $pb.GeneratedMessage {
  factory ChangeFieldWriteRuleRequest({
    $core.String? manageId,
    $core.String? groupId,
    $core.String? fieldId,
    $core.String? writeRule,
  }) {
    final $result = create();
    if (manageId != null) {
      $result.manageId = manageId;
    }
    if (groupId != null) {
      $result.groupId = groupId;
    }
    if (fieldId != null) {
      $result.fieldId = fieldId;
    }
    if (writeRule != null) {
      $result.writeRule = writeRule;
    }
    return $result;
  }
  ChangeFieldWriteRuleRequest._() : super();
  factory ChangeFieldWriteRuleRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChangeFieldWriteRuleRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'ChangeFieldWriteRuleRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'manageId')
    ..aOS(2, _omitFieldNames ? '' : 'groupId')
    ..aOS(3, _omitFieldNames ? '' : 'fieldId')
    ..aOS(4, _omitFieldNames ? '' : 'writeRule')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ChangeFieldWriteRuleRequest clone() => ChangeFieldWriteRuleRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ChangeFieldWriteRuleRequest copyWith(void Function(ChangeFieldWriteRuleRequest) updates) => super.copyWith((message) => updates(message as ChangeFieldWriteRuleRequest)) as ChangeFieldWriteRuleRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ChangeFieldWriteRuleRequest create() => ChangeFieldWriteRuleRequest._();
  ChangeFieldWriteRuleRequest createEmptyInstance() => create();
  static $pb.PbList<ChangeFieldWriteRuleRequest> createRepeated() => $pb.PbList<ChangeFieldWriteRuleRequest>();
  @$core.pragma('dart2js:noInline')
  static ChangeFieldWriteRuleRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChangeFieldWriteRuleRequest>(create);
  static ChangeFieldWriteRuleRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get manageId => $_getSZ(0);
  @$pb.TagNumber(1)
  set manageId($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasManageId() => $_has(0);
  @$pb.TagNumber(1)
  void clearManageId() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get groupId => $_getSZ(1);
  @$pb.TagNumber(2)
  set groupId($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasGroupId() => $_has(1);
  @$pb.TagNumber(2)
  void clearGroupId() => clearField(2);

  @$pb.TagNumber(3)
  $core.String get fieldId => $_getSZ(2);
  @$pb.TagNumber(3)
  set fieldId($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasFieldId() => $_has(2);
  @$pb.TagNumber(3)
  void clearFieldId() => clearField(3);

  @$pb.TagNumber(4)
  $core.String get writeRule => $_getSZ(3);
  @$pb.TagNumber(4)
  set writeRule($core.String v) { $_setString(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasWriteRule() => $_has(3);
  @$pb.TagNumber(4)
  void clearWriteRule() => clearField(4);
}

class ChangeFieldWriteRuleResponse extends $pb.GeneratedMessage {
  factory ChangeFieldWriteRuleResponse({
    $core.String? result,
  }) {
    final $result = create();
    if (result != null) {
      $result.result = result;
    }
    return $result;
  }
  ChangeFieldWriteRuleResponse._() : super();
  factory ChangeFieldWriteRuleResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChangeFieldWriteRuleResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'ChangeFieldWriteRuleResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'result')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ChangeFieldWriteRuleResponse clone() => ChangeFieldWriteRuleResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ChangeFieldWriteRuleResponse copyWith(void Function(ChangeFieldWriteRuleResponse) updates) => super.copyWith((message) => updates(message as ChangeFieldWriteRuleResponse)) as ChangeFieldWriteRuleResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ChangeFieldWriteRuleResponse create() => ChangeFieldWriteRuleResponse._();
  ChangeFieldWriteRuleResponse createEmptyInstance() => create();
  static $pb.PbList<ChangeFieldWriteRuleResponse> createRepeated() => $pb.PbList<ChangeFieldWriteRuleResponse>();
  @$core.pragma('dart2js:noInline')
  static ChangeFieldWriteRuleResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChangeFieldWriteRuleResponse>(create);
  static ChangeFieldWriteRuleResponse? _defaultInstance;

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
