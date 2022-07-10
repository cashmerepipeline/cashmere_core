///
//  Generated code. Do not modify.
//  source: manage.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

class Manage extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'Manage', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'manageId')
    ..a<$core.List<$core.int>>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'nameMap', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  Manage._() : super();
  factory Manage({
    $core.String? manageId,
    $core.List<$core.int>? nameMap,
  }) {
    final _result = create();
    if (manageId != null) {
      _result.manageId = manageId;
    }
    if (nameMap != null) {
      _result.nameMap = nameMap;
    }
    return _result;
  }
  factory Manage.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Manage.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Manage clone() => Manage()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Manage copyWith(void Function(Manage) updates) => super.copyWith((message) => updates(message as Manage)) as Manage; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static Manage create() => Manage._();
  Manage createEmptyInstance() => create();
  static $pb.PbList<Manage> createRepeated() => $pb.PbList<Manage>();
  @$core.pragma('dart2js:noInline')
  static Manage getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Manage>(create);
  static Manage? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get manageId => $_getSZ(0);
  @$pb.TagNumber(1)
  set manageId($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasManageId() => $_has(0);
  @$pb.TagNumber(1)
  void clearManageId() => clearField(1);

  @$pb.TagNumber(2)
  $core.List<$core.int> get nameMap => $_getN(1);
  @$pb.TagNumber(2)
  set nameMap($core.List<$core.int> v) { $_setBytes(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasNameMap() => $_has(1);
  @$pb.TagNumber(2)
  void clearNameMap() => clearField(2);
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

class GetManagesResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GetManagesResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..pc<Manage>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'manages', $pb.PbFieldType.PM, subBuilder: Manage.create)
    ..hasRequiredFields = false
  ;

  GetManagesResponse._() : super();
  factory GetManagesResponse({
    $core.Iterable<Manage>? manages,
  }) {
    final _result = create();
    if (manages != null) {
      _result.manages.addAll(manages);
    }
    return _result;
  }
  factory GetManagesResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetManagesResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetManagesResponse clone() => GetManagesResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetManagesResponse copyWith(void Function(GetManagesResponse) updates) => super.copyWith((message) => updates(message as GetManagesResponse)) as GetManagesResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GetManagesResponse create() => GetManagesResponse._();
  GetManagesResponse createEmptyInstance() => create();
  static $pb.PbList<GetManagesResponse> createRepeated() => $pb.PbList<GetManagesResponse>();
  @$core.pragma('dart2js:noInline')
  static GetManagesResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetManagesResponse>(create);
  static GetManagesResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<Manage> get manages => $_getList(0);
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

