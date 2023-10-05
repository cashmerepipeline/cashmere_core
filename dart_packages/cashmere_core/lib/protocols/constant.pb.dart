//
//  Generated code. Do not modify.
//  source: constant.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

/// 通用常量不需要参数的取得接口，简化api
/// 如果常量需要参数，则需要单独定义接口
/// 每种可能有提供有自己的访问接口
/// 常量一般不需要权限控制
class GetConstantsRequest extends $pb.GeneratedMessage {
  factory GetConstantsRequest({
    $core.int? manageId,
  }) {
    final $result = create();
    if (manageId != null) {
      $result.manageId = manageId;
    }
    return $result;
  }
  GetConstantsRequest._() : super();
  factory GetConstantsRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetConstantsRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'GetConstantsRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..a<$core.int>(1, _omitFieldNames ? '' : 'manageId', $pb.PbFieldType.O3)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetConstantsRequest clone() => GetConstantsRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetConstantsRequest copyWith(void Function(GetConstantsRequest) updates) => super.copyWith((message) => updates(message as GetConstantsRequest)) as GetConstantsRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GetConstantsRequest create() => GetConstantsRequest._();
  GetConstantsRequest createEmptyInstance() => create();
  static $pb.PbList<GetConstantsRequest> createRepeated() => $pb.PbList<GetConstantsRequest>();
  @$core.pragma('dart2js:noInline')
  static GetConstantsRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetConstantsRequest>(create);
  static GetConstantsRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get manageId => $_getIZ(0);
  @$pb.TagNumber(1)
  set manageId($core.int v) { $_setSignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasManageId() => $_has(0);
  @$pb.TagNumber(1)
  void clearManageId() => clearField(1);
}

class GetConstantsResponse extends $pb.GeneratedMessage {
  factory GetConstantsResponse({
    $core.Iterable<$core.List<$core.int>>? constants,
  }) {
    final $result = create();
    if (constants != null) {
      $result.constants.addAll(constants);
    }
    return $result;
  }
  GetConstantsResponse._() : super();
  factory GetConstantsResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetConstantsResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'GetConstantsResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..p<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'constants', $pb.PbFieldType.PY)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetConstantsResponse clone() => GetConstantsResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetConstantsResponse copyWith(void Function(GetConstantsResponse) updates) => super.copyWith((message) => updates(message as GetConstantsResponse)) as GetConstantsResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GetConstantsResponse create() => GetConstantsResponse._();
  GetConstantsResponse createEmptyInstance() => create();
  static $pb.PbList<GetConstantsResponse> createRepeated() => $pb.PbList<GetConstantsResponse>();
  @$core.pragma('dart2js:noInline')
  static GetConstantsResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetConstantsResponse>(create);
  static GetConstantsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.List<$core.int>> get constants => $_getList(0);
}


const _omitFieldNames = $core.bool.fromEnvironment('protobuf.omit_field_names');
const _omitMessageNames = $core.bool.fromEnvironment('protobuf.omit_message_names');
