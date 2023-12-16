//
//  Generated code. Do not modify.
//  source: manage.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

/// 取得管理列表
class GetManagesRequest extends $pb.GeneratedMessage {
  factory GetManagesRequest() => create();
  GetManagesRequest._() : super();
  factory GetManagesRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetManagesRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'GetManagesRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetManagesRequest clone() => GetManagesRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetManagesRequest copyWith(void Function(GetManagesRequest) updates) => super.copyWith((message) => updates(message as GetManagesRequest)) as GetManagesRequest;

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
  factory GetManagesResponse({
    $core.Iterable<$core.List<$core.int>>? manages,
  }) {
    final $result = create();
    if (manages != null) {
      $result.manages.addAll(manages);
    }
    return $result;
  }
  GetManagesResponse._() : super();
  factory GetManagesResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetManagesResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'GetManagesResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..p<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'manages', $pb.PbFieldType.PY)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetManagesResponse clone() => GetManagesResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetManagesResponse copyWith(void Function(GetManagesResponse) updates) => super.copyWith((message) => updates(message as GetManagesResponse)) as GetManagesResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GetManagesResponse create() => GetManagesResponse._();
  GetManagesResponse createEmptyInstance() => create();
  static $pb.PbList<GetManagesResponse> createRepeated() => $pb.PbList<GetManagesResponse>();
  @$core.pragma('dart2js:noInline')
  static GetManagesResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetManagesResponse>(create);
  static GetManagesResponse? _defaultInstance;

  /// 类型为bson document bytes
  @$pb.TagNumber(1)
  $core.List<$core.List<$core.int>> get manages => $_getList(0);
}

/// 取得记录数量
class GetManageEntryCountRequest extends $pb.GeneratedMessage {
  factory GetManageEntryCountRequest({
    $core.String? manageId,
  }) {
    final $result = create();
    if (manageId != null) {
      $result.manageId = manageId;
    }
    return $result;
  }
  GetManageEntryCountRequest._() : super();
  factory GetManageEntryCountRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetManageEntryCountRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'GetManageEntryCountRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'manageId')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetManageEntryCountRequest clone() => GetManageEntryCountRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetManageEntryCountRequest copyWith(void Function(GetManageEntryCountRequest) updates) => super.copyWith((message) => updates(message as GetManageEntryCountRequest)) as GetManageEntryCountRequest;

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
  factory GetManageEntryCountResponse({
    $fixnum.Int64? count,
  }) {
    final $result = create();
    if (count != null) {
      $result.count = count;
    }
    return $result;
  }
  GetManageEntryCountResponse._() : super();
  factory GetManageEntryCountResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetManageEntryCountResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'GetManageEntryCountResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..a<$fixnum.Int64>(1, _omitFieldNames ? '' : 'count', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetManageEntryCountResponse clone() => GetManageEntryCountResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetManageEntryCountResponse copyWith(void Function(GetManageEntryCountResponse) updates) => super.copyWith((message) => updates(message as GetManageEntryCountResponse)) as GetManageEntryCountResponse;

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


const _omitFieldNames = $core.bool.fromEnvironment('protobuf.omit_field_names');
const _omitMessageNames = $core.bool.fromEnvironment('protobuf.omit_message_names');
