///
//  Generated code. Do not modify.
//  source: view.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

class GetManageViewRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GetManageViewRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'manageName')
    ..hasRequiredFields = false
  ;

  GetManageViewRequest._() : super();
  factory GetManageViewRequest({
    $core.String? manageName,
  }) {
    final _result = create();
    if (manageName != null) {
      _result.manageName = manageName;
    }
    return _result;
  }
  factory GetManageViewRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetManageViewRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetManageViewRequest clone() => GetManageViewRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetManageViewRequest copyWith(void Function(GetManageViewRequest) updates) => super.copyWith((message) => updates(message as GetManageViewRequest)) as GetManageViewRequest; // ignore: deprecated_member_use
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

class GetManageViewResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GetManageViewResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'viewToken')
    ..hasRequiredFields = false
  ;

  GetManageViewResponse._() : super();
  factory GetManageViewResponse({
    $core.String? viewToken,
  }) {
    final _result = create();
    if (viewToken != null) {
      _result.viewToken = viewToken;
    }
    return _result;
  }
  factory GetManageViewResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetManageViewResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetManageViewResponse clone() => GetManageViewResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetManageViewResponse copyWith(void Function(GetManageViewResponse) updates) => super.copyWith((message) => updates(message as GetManageViewResponse)) as GetManageViewResponse; // ignore: deprecated_member_use
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

class ChangeManageReadRuleRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'ChangeManageReadRuleRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..a<$core.int>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'manageId', $pb.PbFieldType.O3)
    ..aOS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'groupId')
    ..aOS(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'readRule')
    ..hasRequiredFields = false
  ;

  ChangeManageReadRuleRequest._() : super();
  factory ChangeManageReadRuleRequest({
    $core.int? manageId,
    $core.String? groupId,
    $core.String? readRule,
  }) {
    final _result = create();
    if (manageId != null) {
      _result.manageId = manageId;
    }
    if (groupId != null) {
      _result.groupId = groupId;
    }
    if (readRule != null) {
      _result.readRule = readRule;
    }
    return _result;
  }
  factory ChangeManageReadRuleRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChangeManageReadRuleRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ChangeManageReadRuleRequest clone() => ChangeManageReadRuleRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ChangeManageReadRuleRequest copyWith(void Function(ChangeManageReadRuleRequest) updates) => super.copyWith((message) => updates(message as ChangeManageReadRuleRequest)) as ChangeManageReadRuleRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChangeManageReadRuleRequest create() => ChangeManageReadRuleRequest._();
  ChangeManageReadRuleRequest createEmptyInstance() => create();
  static $pb.PbList<ChangeManageReadRuleRequest> createRepeated() => $pb.PbList<ChangeManageReadRuleRequest>();
  @$core.pragma('dart2js:noInline')
  static ChangeManageReadRuleRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChangeManageReadRuleRequest>(create);
  static ChangeManageReadRuleRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get manageId => $_getIZ(0);
  @$pb.TagNumber(1)
  set manageId($core.int v) { $_setSignedInt32(0, v); }
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
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'ChangeManageReadRuleResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'result')
    ..hasRequiredFields = false
  ;

  ChangeManageReadRuleResponse._() : super();
  factory ChangeManageReadRuleResponse({
    $core.String? result,
  }) {
    final _result = create();
    if (result != null) {
      _result.result = result;
    }
    return _result;
  }
  factory ChangeManageReadRuleResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChangeManageReadRuleResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ChangeManageReadRuleResponse clone() => ChangeManageReadRuleResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ChangeManageReadRuleResponse copyWith(void Function(ChangeManageReadRuleResponse) updates) => super.copyWith((message) => updates(message as ChangeManageReadRuleResponse)) as ChangeManageReadRuleResponse; // ignore: deprecated_member_use
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
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'ChangeManageWriteRuleRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..a<$core.int>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'manageId', $pb.PbFieldType.O3)
    ..aOS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'groupId')
    ..aOS(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'writeRule')
    ..hasRequiredFields = false
  ;

  ChangeManageWriteRuleRequest._() : super();
  factory ChangeManageWriteRuleRequest({
    $core.int? manageId,
    $core.String? groupId,
    $core.String? writeRule,
  }) {
    final _result = create();
    if (manageId != null) {
      _result.manageId = manageId;
    }
    if (groupId != null) {
      _result.groupId = groupId;
    }
    if (writeRule != null) {
      _result.writeRule = writeRule;
    }
    return _result;
  }
  factory ChangeManageWriteRuleRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChangeManageWriteRuleRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ChangeManageWriteRuleRequest clone() => ChangeManageWriteRuleRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ChangeManageWriteRuleRequest copyWith(void Function(ChangeManageWriteRuleRequest) updates) => super.copyWith((message) => updates(message as ChangeManageWriteRuleRequest)) as ChangeManageWriteRuleRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChangeManageWriteRuleRequest create() => ChangeManageWriteRuleRequest._();
  ChangeManageWriteRuleRequest createEmptyInstance() => create();
  static $pb.PbList<ChangeManageWriteRuleRequest> createRepeated() => $pb.PbList<ChangeManageWriteRuleRequest>();
  @$core.pragma('dart2js:noInline')
  static ChangeManageWriteRuleRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChangeManageWriteRuleRequest>(create);
  static ChangeManageWriteRuleRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get manageId => $_getIZ(0);
  @$pb.TagNumber(1)
  set manageId($core.int v) { $_setSignedInt32(0, v); }
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
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'ChangeManageWriteRuleResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'result')
    ..hasRequiredFields = false
  ;

  ChangeManageWriteRuleResponse._() : super();
  factory ChangeManageWriteRuleResponse({
    $core.String? result,
  }) {
    final _result = create();
    if (result != null) {
      _result.result = result;
    }
    return _result;
  }
  factory ChangeManageWriteRuleResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChangeManageWriteRuleResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ChangeManageWriteRuleResponse clone() => ChangeManageWriteRuleResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ChangeManageWriteRuleResponse copyWith(void Function(ChangeManageWriteRuleResponse) updates) => super.copyWith((message) => updates(message as ChangeManageWriteRuleResponse)) as ChangeManageWriteRuleResponse; // ignore: deprecated_member_use
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

class ChangeCollectionReadRuleRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'ChangeCollectionReadRuleRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..a<$core.int>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'manageId', $pb.PbFieldType.O3)
    ..aOS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'groupId')
    ..aOS(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'readRule')
    ..hasRequiredFields = false
  ;

  ChangeCollectionReadRuleRequest._() : super();
  factory ChangeCollectionReadRuleRequest({
    $core.int? manageId,
    $core.String? groupId,
    $core.String? readRule,
  }) {
    final _result = create();
    if (manageId != null) {
      _result.manageId = manageId;
    }
    if (groupId != null) {
      _result.groupId = groupId;
    }
    if (readRule != null) {
      _result.readRule = readRule;
    }
    return _result;
  }
  factory ChangeCollectionReadRuleRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChangeCollectionReadRuleRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ChangeCollectionReadRuleRequest clone() => ChangeCollectionReadRuleRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ChangeCollectionReadRuleRequest copyWith(void Function(ChangeCollectionReadRuleRequest) updates) => super.copyWith((message) => updates(message as ChangeCollectionReadRuleRequest)) as ChangeCollectionReadRuleRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChangeCollectionReadRuleRequest create() => ChangeCollectionReadRuleRequest._();
  ChangeCollectionReadRuleRequest createEmptyInstance() => create();
  static $pb.PbList<ChangeCollectionReadRuleRequest> createRepeated() => $pb.PbList<ChangeCollectionReadRuleRequest>();
  @$core.pragma('dart2js:noInline')
  static ChangeCollectionReadRuleRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChangeCollectionReadRuleRequest>(create);
  static ChangeCollectionReadRuleRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get manageId => $_getIZ(0);
  @$pb.TagNumber(1)
  set manageId($core.int v) { $_setSignedInt32(0, v); }
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
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'ChangeCollectionReadRuleResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'result')
    ..hasRequiredFields = false
  ;

  ChangeCollectionReadRuleResponse._() : super();
  factory ChangeCollectionReadRuleResponse({
    $core.String? result,
  }) {
    final _result = create();
    if (result != null) {
      _result.result = result;
    }
    return _result;
  }
  factory ChangeCollectionReadRuleResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChangeCollectionReadRuleResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ChangeCollectionReadRuleResponse clone() => ChangeCollectionReadRuleResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ChangeCollectionReadRuleResponse copyWith(void Function(ChangeCollectionReadRuleResponse) updates) => super.copyWith((message) => updates(message as ChangeCollectionReadRuleResponse)) as ChangeCollectionReadRuleResponse; // ignore: deprecated_member_use
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
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'ChangeCollectionWriteRuleRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..a<$core.int>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'manageId', $pb.PbFieldType.O3)
    ..aOS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'groupId')
    ..aOS(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'writeRule')
    ..hasRequiredFields = false
  ;

  ChangeCollectionWriteRuleRequest._() : super();
  factory ChangeCollectionWriteRuleRequest({
    $core.int? manageId,
    $core.String? groupId,
    $core.String? writeRule,
  }) {
    final _result = create();
    if (manageId != null) {
      _result.manageId = manageId;
    }
    if (groupId != null) {
      _result.groupId = groupId;
    }
    if (writeRule != null) {
      _result.writeRule = writeRule;
    }
    return _result;
  }
  factory ChangeCollectionWriteRuleRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChangeCollectionWriteRuleRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ChangeCollectionWriteRuleRequest clone() => ChangeCollectionWriteRuleRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ChangeCollectionWriteRuleRequest copyWith(void Function(ChangeCollectionWriteRuleRequest) updates) => super.copyWith((message) => updates(message as ChangeCollectionWriteRuleRequest)) as ChangeCollectionWriteRuleRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChangeCollectionWriteRuleRequest create() => ChangeCollectionWriteRuleRequest._();
  ChangeCollectionWriteRuleRequest createEmptyInstance() => create();
  static $pb.PbList<ChangeCollectionWriteRuleRequest> createRepeated() => $pb.PbList<ChangeCollectionWriteRuleRequest>();
  @$core.pragma('dart2js:noInline')
  static ChangeCollectionWriteRuleRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChangeCollectionWriteRuleRequest>(create);
  static ChangeCollectionWriteRuleRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get manageId => $_getIZ(0);
  @$pb.TagNumber(1)
  set manageId($core.int v) { $_setSignedInt32(0, v); }
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
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'ChangeCollectionWriteRuleResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'result')
    ..hasRequiredFields = false
  ;

  ChangeCollectionWriteRuleResponse._() : super();
  factory ChangeCollectionWriteRuleResponse({
    $core.String? result,
  }) {
    final _result = create();
    if (result != null) {
      _result.result = result;
    }
    return _result;
  }
  factory ChangeCollectionWriteRuleResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChangeCollectionWriteRuleResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ChangeCollectionWriteRuleResponse clone() => ChangeCollectionWriteRuleResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ChangeCollectionWriteRuleResponse copyWith(void Function(ChangeCollectionWriteRuleResponse) updates) => super.copyWith((message) => updates(message as ChangeCollectionWriteRuleResponse)) as ChangeCollectionWriteRuleResponse; // ignore: deprecated_member_use
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

class ChangeFieldReadRuleRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'ChangeFieldReadRuleRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..a<$core.int>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'manageId', $pb.PbFieldType.O3)
    ..aOS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'groupId')
    ..aOS(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'fieldId')
    ..aOS(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'readRule')
    ..hasRequiredFields = false
  ;

  ChangeFieldReadRuleRequest._() : super();
  factory ChangeFieldReadRuleRequest({
    $core.int? manageId,
    $core.String? groupId,
    $core.String? fieldId,
    $core.String? readRule,
  }) {
    final _result = create();
    if (manageId != null) {
      _result.manageId = manageId;
    }
    if (groupId != null) {
      _result.groupId = groupId;
    }
    if (fieldId != null) {
      _result.fieldId = fieldId;
    }
    if (readRule != null) {
      _result.readRule = readRule;
    }
    return _result;
  }
  factory ChangeFieldReadRuleRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChangeFieldReadRuleRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ChangeFieldReadRuleRequest clone() => ChangeFieldReadRuleRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ChangeFieldReadRuleRequest copyWith(void Function(ChangeFieldReadRuleRequest) updates) => super.copyWith((message) => updates(message as ChangeFieldReadRuleRequest)) as ChangeFieldReadRuleRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChangeFieldReadRuleRequest create() => ChangeFieldReadRuleRequest._();
  ChangeFieldReadRuleRequest createEmptyInstance() => create();
  static $pb.PbList<ChangeFieldReadRuleRequest> createRepeated() => $pb.PbList<ChangeFieldReadRuleRequest>();
  @$core.pragma('dart2js:noInline')
  static ChangeFieldReadRuleRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChangeFieldReadRuleRequest>(create);
  static ChangeFieldReadRuleRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get manageId => $_getIZ(0);
  @$pb.TagNumber(1)
  set manageId($core.int v) { $_setSignedInt32(0, v); }
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
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'ChangeFieldReadRuleResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'result')
    ..hasRequiredFields = false
  ;

  ChangeFieldReadRuleResponse._() : super();
  factory ChangeFieldReadRuleResponse({
    $core.String? result,
  }) {
    final _result = create();
    if (result != null) {
      _result.result = result;
    }
    return _result;
  }
  factory ChangeFieldReadRuleResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChangeFieldReadRuleResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ChangeFieldReadRuleResponse clone() => ChangeFieldReadRuleResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ChangeFieldReadRuleResponse copyWith(void Function(ChangeFieldReadRuleResponse) updates) => super.copyWith((message) => updates(message as ChangeFieldReadRuleResponse)) as ChangeFieldReadRuleResponse; // ignore: deprecated_member_use
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
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'ChangeFieldWriteRuleRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..a<$core.int>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'manageId', $pb.PbFieldType.O3)
    ..aOS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'groupId')
    ..aOS(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'fieldId')
    ..aOS(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'writeRule')
    ..hasRequiredFields = false
  ;

  ChangeFieldWriteRuleRequest._() : super();
  factory ChangeFieldWriteRuleRequest({
    $core.int? manageId,
    $core.String? groupId,
    $core.String? fieldId,
    $core.String? writeRule,
  }) {
    final _result = create();
    if (manageId != null) {
      _result.manageId = manageId;
    }
    if (groupId != null) {
      _result.groupId = groupId;
    }
    if (fieldId != null) {
      _result.fieldId = fieldId;
    }
    if (writeRule != null) {
      _result.writeRule = writeRule;
    }
    return _result;
  }
  factory ChangeFieldWriteRuleRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChangeFieldWriteRuleRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ChangeFieldWriteRuleRequest clone() => ChangeFieldWriteRuleRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ChangeFieldWriteRuleRequest copyWith(void Function(ChangeFieldWriteRuleRequest) updates) => super.copyWith((message) => updates(message as ChangeFieldWriteRuleRequest)) as ChangeFieldWriteRuleRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChangeFieldWriteRuleRequest create() => ChangeFieldWriteRuleRequest._();
  ChangeFieldWriteRuleRequest createEmptyInstance() => create();
  static $pb.PbList<ChangeFieldWriteRuleRequest> createRepeated() => $pb.PbList<ChangeFieldWriteRuleRequest>();
  @$core.pragma('dart2js:noInline')
  static ChangeFieldWriteRuleRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChangeFieldWriteRuleRequest>(create);
  static ChangeFieldWriteRuleRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get manageId => $_getIZ(0);
  @$pb.TagNumber(1)
  set manageId($core.int v) { $_setSignedInt32(0, v); }
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
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'ChangeFieldWriteRuleResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'result')
    ..hasRequiredFields = false
  ;

  ChangeFieldWriteRuleResponse._() : super();
  factory ChangeFieldWriteRuleResponse({
    $core.String? result,
  }) {
    final _result = create();
    if (result != null) {
      _result.result = result;
    }
    return _result;
  }
  factory ChangeFieldWriteRuleResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChangeFieldWriteRuleResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ChangeFieldWriteRuleResponse clone() => ChangeFieldWriteRuleResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ChangeFieldWriteRuleResponse copyWith(void Function(ChangeFieldWriteRuleResponse) updates) => super.copyWith((message) => updates(message as ChangeFieldWriteRuleResponse)) as ChangeFieldWriteRuleResponse; // ignore: deprecated_member_use
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

