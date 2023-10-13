//
//  Generated code. Do not modify.
//  source: language_code.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

import 'name.pb.dart' as $0;

/// 新语言编码
class NewLanguageCodeRequest extends $pb.GeneratedMessage {
  factory NewLanguageCodeRequest({
    $0.Name? name,
    $core.String? code,
    $core.String? nativeName,
  }) {
    final $result = create();
    if (name != null) {
      $result.name = name;
    }
    if (code != null) {
      $result.code = code;
    }
    if (nativeName != null) {
      $result.nativeName = nativeName;
    }
    return $result;
  }
  NewLanguageCodeRequest._() : super();
  factory NewLanguageCodeRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory NewLanguageCodeRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'NewLanguageCodeRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOM<$0.Name>(1, _omitFieldNames ? '' : 'name', subBuilder: $0.Name.create)
    ..aOS(2, _omitFieldNames ? '' : 'code')
    ..aOS(3, _omitFieldNames ? '' : 'nativeName')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  NewLanguageCodeRequest clone() => NewLanguageCodeRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  NewLanguageCodeRequest copyWith(void Function(NewLanguageCodeRequest) updates) => super.copyWith((message) => updates(message as NewLanguageCodeRequest)) as NewLanguageCodeRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static NewLanguageCodeRequest create() => NewLanguageCodeRequest._();
  NewLanguageCodeRequest createEmptyInstance() => create();
  static $pb.PbList<NewLanguageCodeRequest> createRepeated() => $pb.PbList<NewLanguageCodeRequest>();
  @$core.pragma('dart2js:noInline')
  static NewLanguageCodeRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<NewLanguageCodeRequest>(create);
  static NewLanguageCodeRequest? _defaultInstance;

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

  @$pb.TagNumber(2)
  $core.String get code => $_getSZ(1);
  @$pb.TagNumber(2)
  set code($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasCode() => $_has(1);
  @$pb.TagNumber(2)
  void clearCode() => clearField(2);

  @$pb.TagNumber(3)
  $core.String get nativeName => $_getSZ(2);
  @$pb.TagNumber(3)
  set nativeName($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasNativeName() => $_has(2);
  @$pb.TagNumber(3)
  void clearNativeName() => clearField(3);
}

class NewLanguageCodeResponse extends $pb.GeneratedMessage {
  factory NewLanguageCodeResponse({
    $core.String? result,
  }) {
    final $result = create();
    if (result != null) {
      $result.result = result;
    }
    return $result;
  }
  NewLanguageCodeResponse._() : super();
  factory NewLanguageCodeResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory NewLanguageCodeResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'NewLanguageCodeResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'result')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  NewLanguageCodeResponse clone() => NewLanguageCodeResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  NewLanguageCodeResponse copyWith(void Function(NewLanguageCodeResponse) updates) => super.copyWith((message) => updates(message as NewLanguageCodeResponse)) as NewLanguageCodeResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static NewLanguageCodeResponse create() => NewLanguageCodeResponse._();
  NewLanguageCodeResponse createEmptyInstance() => create();
  static $pb.PbList<NewLanguageCodeResponse> createRepeated() => $pb.PbList<NewLanguageCodeResponse>();
  @$core.pragma('dart2js:noInline')
  static NewLanguageCodeResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<NewLanguageCodeResponse>(create);
  static NewLanguageCodeResponse? _defaultInstance;

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
class GetLanguageCodesRequest extends $pb.GeneratedMessage {
  factory GetLanguageCodesRequest() => create();
  GetLanguageCodesRequest._() : super();
  factory GetLanguageCodesRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetLanguageCodesRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'GetLanguageCodesRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetLanguageCodesRequest clone() => GetLanguageCodesRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetLanguageCodesRequest copyWith(void Function(GetLanguageCodesRequest) updates) => super.copyWith((message) => updates(message as GetLanguageCodesRequest)) as GetLanguageCodesRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GetLanguageCodesRequest create() => GetLanguageCodesRequest._();
  GetLanguageCodesRequest createEmptyInstance() => create();
  static $pb.PbList<GetLanguageCodesRequest> createRepeated() => $pb.PbList<GetLanguageCodesRequest>();
  @$core.pragma('dart2js:noInline')
  static GetLanguageCodesRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetLanguageCodesRequest>(create);
  static GetLanguageCodesRequest? _defaultInstance;
}

class GetLanguageCodesResponse extends $pb.GeneratedMessage {
  factory GetLanguageCodesResponse({
    $core.Iterable<$core.List<$core.int>>? languageCodes,
  }) {
    final $result = create();
    if (languageCodes != null) {
      $result.languageCodes.addAll(languageCodes);
    }
    return $result;
  }
  GetLanguageCodesResponse._() : super();
  factory GetLanguageCodesResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetLanguageCodesResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'GetLanguageCodesResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..p<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'languageCodes', $pb.PbFieldType.PY)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetLanguageCodesResponse clone() => GetLanguageCodesResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetLanguageCodesResponse copyWith(void Function(GetLanguageCodesResponse) updates) => super.copyWith((message) => updates(message as GetLanguageCodesResponse)) as GetLanguageCodesResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GetLanguageCodesResponse create() => GetLanguageCodesResponse._();
  GetLanguageCodesResponse createEmptyInstance() => create();
  static $pb.PbList<GetLanguageCodesResponse> createRepeated() => $pb.PbList<GetLanguageCodesResponse>();
  @$core.pragma('dart2js:noInline')
  static GetLanguageCodesResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetLanguageCodesResponse>(create);
  static GetLanguageCodesResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.List<$core.int>> get languageCodes => $_getList(0);
}

/// TODO: 可能不需要
class UpdateLanguageCodeRequest extends $pb.GeneratedMessage {
  factory UpdateLanguageCodeRequest({
    $core.String? id,
    $core.String? newCode,
    $core.String? newNative,
  }) {
    final $result = create();
    if (id != null) {
      $result.id = id;
    }
    if (newCode != null) {
      $result.newCode = newCode;
    }
    if (newNative != null) {
      $result.newNative = newNative;
    }
    return $result;
  }
  UpdateLanguageCodeRequest._() : super();
  factory UpdateLanguageCodeRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory UpdateLanguageCodeRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'UpdateLanguageCodeRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'newCode')
    ..aOS(3, _omitFieldNames ? '' : 'newNative')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  UpdateLanguageCodeRequest clone() => UpdateLanguageCodeRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  UpdateLanguageCodeRequest copyWith(void Function(UpdateLanguageCodeRequest) updates) => super.copyWith((message) => updates(message as UpdateLanguageCodeRequest)) as UpdateLanguageCodeRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static UpdateLanguageCodeRequest create() => UpdateLanguageCodeRequest._();
  UpdateLanguageCodeRequest createEmptyInstance() => create();
  static $pb.PbList<UpdateLanguageCodeRequest> createRepeated() => $pb.PbList<UpdateLanguageCodeRequest>();
  @$core.pragma('dart2js:noInline')
  static UpdateLanguageCodeRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<UpdateLanguageCodeRequest>(create);
  static UpdateLanguageCodeRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get id => $_getSZ(0);
  @$pb.TagNumber(1)
  set id($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get newCode => $_getSZ(1);
  @$pb.TagNumber(2)
  set newCode($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasNewCode() => $_has(1);
  @$pb.TagNumber(2)
  void clearNewCode() => clearField(2);

  @$pb.TagNumber(3)
  $core.String get newNative => $_getSZ(2);
  @$pb.TagNumber(3)
  set newNative($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasNewNative() => $_has(2);
  @$pb.TagNumber(3)
  void clearNewNative() => clearField(3);
}

class UpdateLanguageCodeResponse extends $pb.GeneratedMessage {
  factory UpdateLanguageCodeResponse({
    $core.String? result,
  }) {
    final $result = create();
    if (result != null) {
      $result.result = result;
    }
    return $result;
  }
  UpdateLanguageCodeResponse._() : super();
  factory UpdateLanguageCodeResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory UpdateLanguageCodeResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'UpdateLanguageCodeResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'result')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  UpdateLanguageCodeResponse clone() => UpdateLanguageCodeResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  UpdateLanguageCodeResponse copyWith(void Function(UpdateLanguageCodeResponse) updates) => super.copyWith((message) => updates(message as UpdateLanguageCodeResponse)) as UpdateLanguageCodeResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static UpdateLanguageCodeResponse create() => UpdateLanguageCodeResponse._();
  UpdateLanguageCodeResponse createEmptyInstance() => create();
  static $pb.PbList<UpdateLanguageCodeResponse> createRepeated() => $pb.PbList<UpdateLanguageCodeResponse>();
  @$core.pragma('dart2js:noInline')
  static UpdateLanguageCodeResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<UpdateLanguageCodeResponse>(create);
  static UpdateLanguageCodeResponse? _defaultInstance;

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
