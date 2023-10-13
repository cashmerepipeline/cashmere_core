//
//  Generated code. Do not modify.
//  source: country_code.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

import 'name.pb.dart' as $0;

/// 新国家编码
class NewCountryCodeRequest extends $pb.GeneratedMessage {
  factory NewCountryCodeRequest({
    $0.Name? name,
    $core.String? native,
    $core.String? code,
    $core.String? phoneAreaCode,
    $core.Iterable<$core.String>? languages,
  }) {
    final $result = create();
    if (name != null) {
      $result.name = name;
    }
    if (native != null) {
      $result.native = native;
    }
    if (code != null) {
      $result.code = code;
    }
    if (phoneAreaCode != null) {
      $result.phoneAreaCode = phoneAreaCode;
    }
    if (languages != null) {
      $result.languages.addAll(languages);
    }
    return $result;
  }
  NewCountryCodeRequest._() : super();
  factory NewCountryCodeRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory NewCountryCodeRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'NewCountryCodeRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOM<$0.Name>(1, _omitFieldNames ? '' : 'name', subBuilder: $0.Name.create)
    ..aOS(2, _omitFieldNames ? '' : 'native')
    ..aOS(3, _omitFieldNames ? '' : 'code')
    ..aOS(4, _omitFieldNames ? '' : 'phoneAreaCode')
    ..pPS(5, _omitFieldNames ? '' : 'languages')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  NewCountryCodeRequest clone() => NewCountryCodeRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  NewCountryCodeRequest copyWith(void Function(NewCountryCodeRequest) updates) => super.copyWith((message) => updates(message as NewCountryCodeRequest)) as NewCountryCodeRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static NewCountryCodeRequest create() => NewCountryCodeRequest._();
  NewCountryCodeRequest createEmptyInstance() => create();
  static $pb.PbList<NewCountryCodeRequest> createRepeated() => $pb.PbList<NewCountryCodeRequest>();
  @$core.pragma('dart2js:noInline')
  static NewCountryCodeRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<NewCountryCodeRequest>(create);
  static NewCountryCodeRequest? _defaultInstance;

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

  /// 母语名
  @$pb.TagNumber(2)
  $core.String get native => $_getSZ(1);
  @$pb.TagNumber(2)
  set native($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasNative() => $_has(1);
  @$pb.TagNumber(2)
  void clearNative() => clearField(2);

  @$pb.TagNumber(3)
  $core.String get code => $_getSZ(2);
  @$pb.TagNumber(3)
  set code($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasCode() => $_has(2);
  @$pb.TagNumber(3)
  void clearCode() => clearField(3);

  @$pb.TagNumber(4)
  $core.String get phoneAreaCode => $_getSZ(3);
  @$pb.TagNumber(4)
  set phoneAreaCode($core.String v) { $_setString(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasPhoneAreaCode() => $_has(3);
  @$pb.TagNumber(4)
  void clearPhoneAreaCode() => clearField(4);

  @$pb.TagNumber(5)
  $core.List<$core.String> get languages => $_getList(4);
}

class NewCountryCodeResponse extends $pb.GeneratedMessage {
  factory NewCountryCodeResponse({
    $core.String? result,
  }) {
    final $result = create();
    if (result != null) {
      $result.result = result;
    }
    return $result;
  }
  NewCountryCodeResponse._() : super();
  factory NewCountryCodeResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory NewCountryCodeResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'NewCountryCodeResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'result')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  NewCountryCodeResponse clone() => NewCountryCodeResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  NewCountryCodeResponse copyWith(void Function(NewCountryCodeResponse) updates) => super.copyWith((message) => updates(message as NewCountryCodeResponse)) as NewCountryCodeResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static NewCountryCodeResponse create() => NewCountryCodeResponse._();
  NewCountryCodeResponse createEmptyInstance() => create();
  static $pb.PbList<NewCountryCodeResponse> createRepeated() => $pb.PbList<NewCountryCodeResponse>();
  @$core.pragma('dart2js:noInline')
  static NewCountryCodeResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<NewCountryCodeResponse>(create);
  static NewCountryCodeResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get result => $_getSZ(0);
  @$pb.TagNumber(1)
  set result($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasResult() => $_has(0);
  @$pb.TagNumber(1)
  void clearResult() => clearField(1);
}

/// 取得编码列表, 读取不需要权限
/// 客户端应该缓存这个列表
class GetCountryCodesRequest extends $pb.GeneratedMessage {
  factory GetCountryCodesRequest() => create();
  GetCountryCodesRequest._() : super();
  factory GetCountryCodesRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetCountryCodesRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'GetCountryCodesRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetCountryCodesRequest clone() => GetCountryCodesRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetCountryCodesRequest copyWith(void Function(GetCountryCodesRequest) updates) => super.copyWith((message) => updates(message as GetCountryCodesRequest)) as GetCountryCodesRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GetCountryCodesRequest create() => GetCountryCodesRequest._();
  GetCountryCodesRequest createEmptyInstance() => create();
  static $pb.PbList<GetCountryCodesRequest> createRepeated() => $pb.PbList<GetCountryCodesRequest>();
  @$core.pragma('dart2js:noInline')
  static GetCountryCodesRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetCountryCodesRequest>(create);
  static GetCountryCodesRequest? _defaultInstance;
}

class GetCountryCodesResponse extends $pb.GeneratedMessage {
  factory GetCountryCodesResponse({
    $core.Iterable<$core.List<$core.int>>? countryCodes,
  }) {
    final $result = create();
    if (countryCodes != null) {
      $result.countryCodes.addAll(countryCodes);
    }
    return $result;
  }
  GetCountryCodesResponse._() : super();
  factory GetCountryCodesResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetCountryCodesResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'GetCountryCodesResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..p<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'countryCodes', $pb.PbFieldType.PY)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetCountryCodesResponse clone() => GetCountryCodesResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetCountryCodesResponse copyWith(void Function(GetCountryCodesResponse) updates) => super.copyWith((message) => updates(message as GetCountryCodesResponse)) as GetCountryCodesResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GetCountryCodesResponse create() => GetCountryCodesResponse._();
  GetCountryCodesResponse createEmptyInstance() => create();
  static $pb.PbList<GetCountryCodesResponse> createRepeated() => $pb.PbList<GetCountryCodesResponse>();
  @$core.pragma('dart2js:noInline')
  static GetCountryCodesResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetCountryCodesResponse>(create);
  static GetCountryCodesResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.List<$core.int>> get countryCodes => $_getList(0);
}


const _omitFieldNames = $core.bool.fromEnvironment('protobuf.omit_field_names');
const _omitMessageNames = $core.bool.fromEnvironment('protobuf.omit_message_names');
