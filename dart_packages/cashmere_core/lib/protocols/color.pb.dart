//
//  Generated code. Do not modify.
//  source: color.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

import 'name.pb.dart' as $0;

/// 新颜色
class NewColorRequest extends $pb.GeneratedMessage {
  factory NewColorRequest({
    $0.Name? name,
    $core.Iterable<$core.int>? color,
    $core.String? description,
  }) {
    final $result = create();
    if (name != null) {
      $result.name = name;
    }
    if (color != null) {
      $result.color.addAll(color);
    }
    if (description != null) {
      $result.description = description;
    }
    return $result;
  }
  NewColorRequest._() : super();
  factory NewColorRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory NewColorRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'NewColorRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOM<$0.Name>(1, _omitFieldNames ? '' : 'name', subBuilder: $0.Name.create)
    ..p<$core.int>(2, _omitFieldNames ? '' : 'color', $pb.PbFieldType.KU3)
    ..aOS(3, _omitFieldNames ? '' : 'description')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  NewColorRequest clone() => NewColorRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  NewColorRequest copyWith(void Function(NewColorRequest) updates) => super.copyWith((message) => updates(message as NewColorRequest)) as NewColorRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static NewColorRequest create() => NewColorRequest._();
  NewColorRequest createEmptyInstance() => create();
  static $pb.PbList<NewColorRequest> createRepeated() => $pb.PbList<NewColorRequest>();
  @$core.pragma('dart2js:noInline')
  static NewColorRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<NewColorRequest>(create);
  static NewColorRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $0.Name get name => $_getN(0);
  @$pb.TagNumber(1)
  set name($0.Name v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasName() => $_has(0);
  @$pb.TagNumber(1)
  void clearName() => clearField(1);
  @$pb.TagNumber(1)
  $0.Name ensureName() => $_ensure(0);

  /// [r,g,b]
  @$pb.TagNumber(2)
  $core.List<$core.int> get color => $_getList(1);

  @$pb.TagNumber(3)
  $core.String get description => $_getSZ(2);
  @$pb.TagNumber(3)
  set description($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasDescription() => $_has(2);
  @$pb.TagNumber(3)
  void clearDescription() => clearField(3);
}

class NewColorResponse extends $pb.GeneratedMessage {
  factory NewColorResponse({
    $core.String? result,
  }) {
    final $result = create();
    if (result != null) {
      $result.result = result;
    }
    return $result;
  }
  NewColorResponse._() : super();
  factory NewColorResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory NewColorResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'NewColorResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'result')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  NewColorResponse clone() => NewColorResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  NewColorResponse copyWith(void Function(NewColorResponse) updates) => super.copyWith((message) => updates(message as NewColorResponse)) as NewColorResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static NewColorResponse create() => NewColorResponse._();
  NewColorResponse createEmptyInstance() => create();
  static $pb.PbList<NewColorResponse> createRepeated() => $pb.PbList<NewColorResponse>();
  @$core.pragma('dart2js:noInline')
  static NewColorResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<NewColorResponse>(create);
  static NewColorResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get result => $_getSZ(0);
  @$pb.TagNumber(1)
  set result($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasResult() => $_has(0);
  @$pb.TagNumber(1)
  void clearResult() => clearField(1);
}

/// 获取颜色
class GetColorsRequest extends $pb.GeneratedMessage {
  factory GetColorsRequest() => create();
  GetColorsRequest._() : super();
  factory GetColorsRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetColorsRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'GetColorsRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetColorsRequest clone() => GetColorsRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetColorsRequest copyWith(void Function(GetColorsRequest) updates) => super.copyWith((message) => updates(message as GetColorsRequest)) as GetColorsRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GetColorsRequest create() => GetColorsRequest._();
  GetColorsRequest createEmptyInstance() => create();
  static $pb.PbList<GetColorsRequest> createRepeated() => $pb.PbList<GetColorsRequest>();
  @$core.pragma('dart2js:noInline')
  static GetColorsRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetColorsRequest>(create);
  static GetColorsRequest? _defaultInstance;
}

class GetColorsResponse extends $pb.GeneratedMessage {
  factory GetColorsResponse({
    $core.Iterable<$core.List<$core.int>>? colors,
  }) {
    final $result = create();
    if (colors != null) {
      $result.colors.addAll(colors);
    }
    return $result;
  }
  GetColorsResponse._() : super();
  factory GetColorsResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetColorsResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'GetColorsResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..p<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'colors', $pb.PbFieldType.PY)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetColorsResponse clone() => GetColorsResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetColorsResponse copyWith(void Function(GetColorsResponse) updates) => super.copyWith((message) => updates(message as GetColorsResponse)) as GetColorsResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GetColorsResponse create() => GetColorsResponse._();
  GetColorsResponse createEmptyInstance() => create();
  static $pb.PbList<GetColorsResponse> createRepeated() => $pb.PbList<GetColorsResponse>();
  @$core.pragma('dart2js:noInline')
  static GetColorsResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetColorsResponse>(create);
  static GetColorsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.List<$core.int>> get colors => $_getList(0);
}


const _omitFieldNames = $core.bool.fromEnvironment('protobuf.omit_field_names');
const _omitMessageNames = $core.bool.fromEnvironment('protobuf.omit_message_names');
