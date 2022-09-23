///
//  Generated code. Do not modify.
//  source: view.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields

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

class AddGroupIntoManageReadRuleRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'AddGroupIntoManageReadRuleRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..a<$core.int>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'manageId', $pb.PbFieldType.O3)
    ..aOS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'groupId')
    ..aOS(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'readRule')
    ..hasRequiredFields = false
  ;

  AddGroupIntoManageReadRuleRequest._() : super();
  factory AddGroupIntoManageReadRuleRequest({
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
  factory AddGroupIntoManageReadRuleRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory AddGroupIntoManageReadRuleRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  AddGroupIntoManageReadRuleRequest clone() => AddGroupIntoManageReadRuleRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  AddGroupIntoManageReadRuleRequest copyWith(void Function(AddGroupIntoManageReadRuleRequest) updates) => super.copyWith((message) => updates(message as AddGroupIntoManageReadRuleRequest)) as AddGroupIntoManageReadRuleRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static AddGroupIntoManageReadRuleRequest create() => AddGroupIntoManageReadRuleRequest._();
  AddGroupIntoManageReadRuleRequest createEmptyInstance() => create();
  static $pb.PbList<AddGroupIntoManageReadRuleRequest> createRepeated() => $pb.PbList<AddGroupIntoManageReadRuleRequest>();
  @$core.pragma('dart2js:noInline')
  static AddGroupIntoManageReadRuleRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<AddGroupIntoManageReadRuleRequest>(create);
  static AddGroupIntoManageReadRuleRequest? _defaultInstance;

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

class AddGroupIntoManageReadRuleResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'AddGroupIntoManageReadRuleResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'result')
    ..hasRequiredFields = false
  ;

  AddGroupIntoManageReadRuleResponse._() : super();
  factory AddGroupIntoManageReadRuleResponse({
    $core.String? result,
  }) {
    final _result = create();
    if (result != null) {
      _result.result = result;
    }
    return _result;
  }
  factory AddGroupIntoManageReadRuleResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory AddGroupIntoManageReadRuleResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  AddGroupIntoManageReadRuleResponse clone() => AddGroupIntoManageReadRuleResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  AddGroupIntoManageReadRuleResponse copyWith(void Function(AddGroupIntoManageReadRuleResponse) updates) => super.copyWith((message) => updates(message as AddGroupIntoManageReadRuleResponse)) as AddGroupIntoManageReadRuleResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static AddGroupIntoManageReadRuleResponse create() => AddGroupIntoManageReadRuleResponse._();
  AddGroupIntoManageReadRuleResponse createEmptyInstance() => create();
  static $pb.PbList<AddGroupIntoManageReadRuleResponse> createRepeated() => $pb.PbList<AddGroupIntoManageReadRuleResponse>();
  @$core.pragma('dart2js:noInline')
  static AddGroupIntoManageReadRuleResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<AddGroupIntoManageReadRuleResponse>(create);
  static AddGroupIntoManageReadRuleResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get result => $_getSZ(0);
  @$pb.TagNumber(1)
  set result($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasResult() => $_has(0);
  @$pb.TagNumber(1)
  void clearResult() => clearField(1);
}

class AddGroupIntoManageWriteRuleRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'AddGroupIntoManageWriteRuleRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..a<$core.int>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'manageId', $pb.PbFieldType.O3)
    ..aOS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'groupId')
    ..aOS(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'readRule')
    ..hasRequiredFields = false
  ;

  AddGroupIntoManageWriteRuleRequest._() : super();
  factory AddGroupIntoManageWriteRuleRequest({
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
  factory AddGroupIntoManageWriteRuleRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory AddGroupIntoManageWriteRuleRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  AddGroupIntoManageWriteRuleRequest clone() => AddGroupIntoManageWriteRuleRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  AddGroupIntoManageWriteRuleRequest copyWith(void Function(AddGroupIntoManageWriteRuleRequest) updates) => super.copyWith((message) => updates(message as AddGroupIntoManageWriteRuleRequest)) as AddGroupIntoManageWriteRuleRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static AddGroupIntoManageWriteRuleRequest create() => AddGroupIntoManageWriteRuleRequest._();
  AddGroupIntoManageWriteRuleRequest createEmptyInstance() => create();
  static $pb.PbList<AddGroupIntoManageWriteRuleRequest> createRepeated() => $pb.PbList<AddGroupIntoManageWriteRuleRequest>();
  @$core.pragma('dart2js:noInline')
  static AddGroupIntoManageWriteRuleRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<AddGroupIntoManageWriteRuleRequest>(create);
  static AddGroupIntoManageWriteRuleRequest? _defaultInstance;

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

class AddGroupIntoManageWriteRuleResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'AddGroupIntoManageWriteRuleResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'result')
    ..hasRequiredFields = false
  ;

  AddGroupIntoManageWriteRuleResponse._() : super();
  factory AddGroupIntoManageWriteRuleResponse({
    $core.String? result,
  }) {
    final _result = create();
    if (result != null) {
      _result.result = result;
    }
    return _result;
  }
  factory AddGroupIntoManageWriteRuleResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory AddGroupIntoManageWriteRuleResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  AddGroupIntoManageWriteRuleResponse clone() => AddGroupIntoManageWriteRuleResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  AddGroupIntoManageWriteRuleResponse copyWith(void Function(AddGroupIntoManageWriteRuleResponse) updates) => super.copyWith((message) => updates(message as AddGroupIntoManageWriteRuleResponse)) as AddGroupIntoManageWriteRuleResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static AddGroupIntoManageWriteRuleResponse create() => AddGroupIntoManageWriteRuleResponse._();
  AddGroupIntoManageWriteRuleResponse createEmptyInstance() => create();
  static $pb.PbList<AddGroupIntoManageWriteRuleResponse> createRepeated() => $pb.PbList<AddGroupIntoManageWriteRuleResponse>();
  @$core.pragma('dart2js:noInline')
  static AddGroupIntoManageWriteRuleResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<AddGroupIntoManageWriteRuleResponse>(create);
  static AddGroupIntoManageWriteRuleResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get result => $_getSZ(0);
  @$pb.TagNumber(1)
  set result($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasResult() => $_has(0);
  @$pb.TagNumber(1)
  void clearResult() => clearField(1);
}

class RemoveGroupFromManageReadRuleRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'RemoveGroupFromManageReadRuleRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..a<$core.int>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'manageId', $pb.PbFieldType.O3)
    ..aOS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'groupId')
    ..aOS(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'readRule')
    ..hasRequiredFields = false
  ;

  RemoveGroupFromManageReadRuleRequest._() : super();
  factory RemoveGroupFromManageReadRuleRequest({
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
  factory RemoveGroupFromManageReadRuleRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory RemoveGroupFromManageReadRuleRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  RemoveGroupFromManageReadRuleRequest clone() => RemoveGroupFromManageReadRuleRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  RemoveGroupFromManageReadRuleRequest copyWith(void Function(RemoveGroupFromManageReadRuleRequest) updates) => super.copyWith((message) => updates(message as RemoveGroupFromManageReadRuleRequest)) as RemoveGroupFromManageReadRuleRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static RemoveGroupFromManageReadRuleRequest create() => RemoveGroupFromManageReadRuleRequest._();
  RemoveGroupFromManageReadRuleRequest createEmptyInstance() => create();
  static $pb.PbList<RemoveGroupFromManageReadRuleRequest> createRepeated() => $pb.PbList<RemoveGroupFromManageReadRuleRequest>();
  @$core.pragma('dart2js:noInline')
  static RemoveGroupFromManageReadRuleRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<RemoveGroupFromManageReadRuleRequest>(create);
  static RemoveGroupFromManageReadRuleRequest? _defaultInstance;

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

class RemoveGroupFromManageReadRuleResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'RemoveGroupFromManageReadRuleResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'result')
    ..hasRequiredFields = false
  ;

  RemoveGroupFromManageReadRuleResponse._() : super();
  factory RemoveGroupFromManageReadRuleResponse({
    $core.String? result,
  }) {
    final _result = create();
    if (result != null) {
      _result.result = result;
    }
    return _result;
  }
  factory RemoveGroupFromManageReadRuleResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory RemoveGroupFromManageReadRuleResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  RemoveGroupFromManageReadRuleResponse clone() => RemoveGroupFromManageReadRuleResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  RemoveGroupFromManageReadRuleResponse copyWith(void Function(RemoveGroupFromManageReadRuleResponse) updates) => super.copyWith((message) => updates(message as RemoveGroupFromManageReadRuleResponse)) as RemoveGroupFromManageReadRuleResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static RemoveGroupFromManageReadRuleResponse create() => RemoveGroupFromManageReadRuleResponse._();
  RemoveGroupFromManageReadRuleResponse createEmptyInstance() => create();
  static $pb.PbList<RemoveGroupFromManageReadRuleResponse> createRepeated() => $pb.PbList<RemoveGroupFromManageReadRuleResponse>();
  @$core.pragma('dart2js:noInline')
  static RemoveGroupFromManageReadRuleResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<RemoveGroupFromManageReadRuleResponse>(create);
  static RemoveGroupFromManageReadRuleResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get result => $_getSZ(0);
  @$pb.TagNumber(1)
  set result($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasResult() => $_has(0);
  @$pb.TagNumber(1)
  void clearResult() => clearField(1);
}

class RemoveGroupFromManageWriteRuleRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'RemoveGroupFromManageWriteRuleRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..a<$core.int>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'manageId', $pb.PbFieldType.O3)
    ..aOS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'groupId')
    ..aOS(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'readRule')
    ..hasRequiredFields = false
  ;

  RemoveGroupFromManageWriteRuleRequest._() : super();
  factory RemoveGroupFromManageWriteRuleRequest({
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
  factory RemoveGroupFromManageWriteRuleRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory RemoveGroupFromManageWriteRuleRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  RemoveGroupFromManageWriteRuleRequest clone() => RemoveGroupFromManageWriteRuleRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  RemoveGroupFromManageWriteRuleRequest copyWith(void Function(RemoveGroupFromManageWriteRuleRequest) updates) => super.copyWith((message) => updates(message as RemoveGroupFromManageWriteRuleRequest)) as RemoveGroupFromManageWriteRuleRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static RemoveGroupFromManageWriteRuleRequest create() => RemoveGroupFromManageWriteRuleRequest._();
  RemoveGroupFromManageWriteRuleRequest createEmptyInstance() => create();
  static $pb.PbList<RemoveGroupFromManageWriteRuleRequest> createRepeated() => $pb.PbList<RemoveGroupFromManageWriteRuleRequest>();
  @$core.pragma('dart2js:noInline')
  static RemoveGroupFromManageWriteRuleRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<RemoveGroupFromManageWriteRuleRequest>(create);
  static RemoveGroupFromManageWriteRuleRequest? _defaultInstance;

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

class RemoveGroupFromManageWriteRuleResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'RemoveGroupFromManageWriteRuleResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'result')
    ..hasRequiredFields = false
  ;

  RemoveGroupFromManageWriteRuleResponse._() : super();
  factory RemoveGroupFromManageWriteRuleResponse({
    $core.String? result,
  }) {
    final _result = create();
    if (result != null) {
      _result.result = result;
    }
    return _result;
  }
  factory RemoveGroupFromManageWriteRuleResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory RemoveGroupFromManageWriteRuleResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  RemoveGroupFromManageWriteRuleResponse clone() => RemoveGroupFromManageWriteRuleResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  RemoveGroupFromManageWriteRuleResponse copyWith(void Function(RemoveGroupFromManageWriteRuleResponse) updates) => super.copyWith((message) => updates(message as RemoveGroupFromManageWriteRuleResponse)) as RemoveGroupFromManageWriteRuleResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static RemoveGroupFromManageWriteRuleResponse create() => RemoveGroupFromManageWriteRuleResponse._();
  RemoveGroupFromManageWriteRuleResponse createEmptyInstance() => create();
  static $pb.PbList<RemoveGroupFromManageWriteRuleResponse> createRepeated() => $pb.PbList<RemoveGroupFromManageWriteRuleResponse>();
  @$core.pragma('dart2js:noInline')
  static RemoveGroupFromManageWriteRuleResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<RemoveGroupFromManageWriteRuleResponse>(create);
  static RemoveGroupFromManageWriteRuleResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get result => $_getSZ(0);
  @$pb.TagNumber(1)
  set result($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasResult() => $_has(0);
  @$pb.TagNumber(1)
  void clearResult() => clearField(1);
}

class AddGroupIntoCollectionReadRuleRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'AddGroupIntoCollectionReadRuleRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..a<$core.int>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'manageId', $pb.PbFieldType.O3)
    ..aOS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'groupId')
    ..aOS(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'readRule')
    ..hasRequiredFields = false
  ;

  AddGroupIntoCollectionReadRuleRequest._() : super();
  factory AddGroupIntoCollectionReadRuleRequest({
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
  factory AddGroupIntoCollectionReadRuleRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory AddGroupIntoCollectionReadRuleRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  AddGroupIntoCollectionReadRuleRequest clone() => AddGroupIntoCollectionReadRuleRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  AddGroupIntoCollectionReadRuleRequest copyWith(void Function(AddGroupIntoCollectionReadRuleRequest) updates) => super.copyWith((message) => updates(message as AddGroupIntoCollectionReadRuleRequest)) as AddGroupIntoCollectionReadRuleRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static AddGroupIntoCollectionReadRuleRequest create() => AddGroupIntoCollectionReadRuleRequest._();
  AddGroupIntoCollectionReadRuleRequest createEmptyInstance() => create();
  static $pb.PbList<AddGroupIntoCollectionReadRuleRequest> createRepeated() => $pb.PbList<AddGroupIntoCollectionReadRuleRequest>();
  @$core.pragma('dart2js:noInline')
  static AddGroupIntoCollectionReadRuleRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<AddGroupIntoCollectionReadRuleRequest>(create);
  static AddGroupIntoCollectionReadRuleRequest? _defaultInstance;

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

class AddGroupIntoCollectionReadRuleResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'AddGroupIntoCollectionReadRuleResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'result')
    ..hasRequiredFields = false
  ;

  AddGroupIntoCollectionReadRuleResponse._() : super();
  factory AddGroupIntoCollectionReadRuleResponse({
    $core.String? result,
  }) {
    final _result = create();
    if (result != null) {
      _result.result = result;
    }
    return _result;
  }
  factory AddGroupIntoCollectionReadRuleResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory AddGroupIntoCollectionReadRuleResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  AddGroupIntoCollectionReadRuleResponse clone() => AddGroupIntoCollectionReadRuleResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  AddGroupIntoCollectionReadRuleResponse copyWith(void Function(AddGroupIntoCollectionReadRuleResponse) updates) => super.copyWith((message) => updates(message as AddGroupIntoCollectionReadRuleResponse)) as AddGroupIntoCollectionReadRuleResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static AddGroupIntoCollectionReadRuleResponse create() => AddGroupIntoCollectionReadRuleResponse._();
  AddGroupIntoCollectionReadRuleResponse createEmptyInstance() => create();
  static $pb.PbList<AddGroupIntoCollectionReadRuleResponse> createRepeated() => $pb.PbList<AddGroupIntoCollectionReadRuleResponse>();
  @$core.pragma('dart2js:noInline')
  static AddGroupIntoCollectionReadRuleResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<AddGroupIntoCollectionReadRuleResponse>(create);
  static AddGroupIntoCollectionReadRuleResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get result => $_getSZ(0);
  @$pb.TagNumber(1)
  set result($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasResult() => $_has(0);
  @$pb.TagNumber(1)
  void clearResult() => clearField(1);
}

class AddGroupIntoCollectionRuleRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'AddGroupIntoCollectionRuleRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..a<$core.int>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'manageId', $pb.PbFieldType.O3)
    ..aOS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'groupId')
    ..aOS(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'readRule')
    ..hasRequiredFields = false
  ;

  AddGroupIntoCollectionRuleRequest._() : super();
  factory AddGroupIntoCollectionRuleRequest({
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
  factory AddGroupIntoCollectionRuleRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory AddGroupIntoCollectionRuleRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  AddGroupIntoCollectionRuleRequest clone() => AddGroupIntoCollectionRuleRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  AddGroupIntoCollectionRuleRequest copyWith(void Function(AddGroupIntoCollectionRuleRequest) updates) => super.copyWith((message) => updates(message as AddGroupIntoCollectionRuleRequest)) as AddGroupIntoCollectionRuleRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static AddGroupIntoCollectionRuleRequest create() => AddGroupIntoCollectionRuleRequest._();
  AddGroupIntoCollectionRuleRequest createEmptyInstance() => create();
  static $pb.PbList<AddGroupIntoCollectionRuleRequest> createRepeated() => $pb.PbList<AddGroupIntoCollectionRuleRequest>();
  @$core.pragma('dart2js:noInline')
  static AddGroupIntoCollectionRuleRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<AddGroupIntoCollectionRuleRequest>(create);
  static AddGroupIntoCollectionRuleRequest? _defaultInstance;

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

class AddGroupIntoCollectionWriteRuleResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'AddGroupIntoCollectionWriteRuleResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'result')
    ..hasRequiredFields = false
  ;

  AddGroupIntoCollectionWriteRuleResponse._() : super();
  factory AddGroupIntoCollectionWriteRuleResponse({
    $core.String? result,
  }) {
    final _result = create();
    if (result != null) {
      _result.result = result;
    }
    return _result;
  }
  factory AddGroupIntoCollectionWriteRuleResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory AddGroupIntoCollectionWriteRuleResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  AddGroupIntoCollectionWriteRuleResponse clone() => AddGroupIntoCollectionWriteRuleResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  AddGroupIntoCollectionWriteRuleResponse copyWith(void Function(AddGroupIntoCollectionWriteRuleResponse) updates) => super.copyWith((message) => updates(message as AddGroupIntoCollectionWriteRuleResponse)) as AddGroupIntoCollectionWriteRuleResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static AddGroupIntoCollectionWriteRuleResponse create() => AddGroupIntoCollectionWriteRuleResponse._();
  AddGroupIntoCollectionWriteRuleResponse createEmptyInstance() => create();
  static $pb.PbList<AddGroupIntoCollectionWriteRuleResponse> createRepeated() => $pb.PbList<AddGroupIntoCollectionWriteRuleResponse>();
  @$core.pragma('dart2js:noInline')
  static AddGroupIntoCollectionWriteRuleResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<AddGroupIntoCollectionWriteRuleResponse>(create);
  static AddGroupIntoCollectionWriteRuleResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get result => $_getSZ(0);
  @$pb.TagNumber(1)
  set result($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasResult() => $_has(0);
  @$pb.TagNumber(1)
  void clearResult() => clearField(1);
}

