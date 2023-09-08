//
//  Generated code. Do not modify.
//  source: phone_area_code.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

import 'name.pb.dart' as $0;

class PhoneAreaCode extends $pb.GeneratedMessage {
  factory PhoneAreaCode({
    $core.String? code,
    $core.Map<$core.String, $core.String>? nameMap,
    $core.Iterable<$core.String>? usingAreas,
  }) {
    final $result = create();
    if (code != null) {
      $result.code = code;
    }
    if (nameMap != null) {
      $result.nameMap.addAll(nameMap);
    }
    if (usingAreas != null) {
      $result.usingAreas.addAll(usingAreas);
    }
    return $result;
  }
  PhoneAreaCode._() : super();
  factory PhoneAreaCode.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory PhoneAreaCode.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'PhoneAreaCode', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'code')
    ..m<$core.String, $core.String>(2, _omitFieldNames ? '' : 'nameMap', entryClassName: 'PhoneAreaCode.NameMapEntry', keyFieldType: $pb.PbFieldType.OS, valueFieldType: $pb.PbFieldType.OS, packageName: const $pb.PackageName('cashmere'))
    ..pPS(3, _omitFieldNames ? '' : 'usingAreas')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  PhoneAreaCode clone() => PhoneAreaCode()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  PhoneAreaCode copyWith(void Function(PhoneAreaCode) updates) => super.copyWith((message) => updates(message as PhoneAreaCode)) as PhoneAreaCode;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static PhoneAreaCode create() => PhoneAreaCode._();
  PhoneAreaCode createEmptyInstance() => create();
  static $pb.PbList<PhoneAreaCode> createRepeated() => $pb.PbList<PhoneAreaCode>();
  @$core.pragma('dart2js:noInline')
  static PhoneAreaCode getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<PhoneAreaCode>(create);
  static PhoneAreaCode? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get code => $_getSZ(0);
  @$pb.TagNumber(1)
  set code($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasCode() => $_has(0);
  @$pb.TagNumber(1)
  void clearCode() => clearField(1);

  @$pb.TagNumber(2)
  $core.Map<$core.String, $core.String> get nameMap => $_getMap(1);

  /// 使用地区
  @$pb.TagNumber(3)
  $core.List<$core.String> get usingAreas => $_getList(2);
}

/// 新区号编码
class NewPhoneAreaCodeRequest extends $pb.GeneratedMessage {
  factory NewPhoneAreaCodeRequest({
    $0.Name? name,
    $core.String? code,
    $core.Iterable<$core.String>? areas,
  }) {
    final $result = create();
    if (name != null) {
      $result.name = name;
    }
    if (code != null) {
      $result.code = code;
    }
    if (areas != null) {
      $result.areas.addAll(areas);
    }
    return $result;
  }
  NewPhoneAreaCodeRequest._() : super();
  factory NewPhoneAreaCodeRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory NewPhoneAreaCodeRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'NewPhoneAreaCodeRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOM<$0.Name>(1, _omitFieldNames ? '' : 'name', subBuilder: $0.Name.create)
    ..aOS(2, _omitFieldNames ? '' : 'code')
    ..pPS(3, _omitFieldNames ? '' : 'areas')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  NewPhoneAreaCodeRequest clone() => NewPhoneAreaCodeRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  NewPhoneAreaCodeRequest copyWith(void Function(NewPhoneAreaCodeRequest) updates) => super.copyWith((message) => updates(message as NewPhoneAreaCodeRequest)) as NewPhoneAreaCodeRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static NewPhoneAreaCodeRequest create() => NewPhoneAreaCodeRequest._();
  NewPhoneAreaCodeRequest createEmptyInstance() => create();
  static $pb.PbList<NewPhoneAreaCodeRequest> createRepeated() => $pb.PbList<NewPhoneAreaCodeRequest>();
  @$core.pragma('dart2js:noInline')
  static NewPhoneAreaCodeRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<NewPhoneAreaCodeRequest>(create);
  static NewPhoneAreaCodeRequest? _defaultInstance;

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

  /// 使用地区
  @$pb.TagNumber(3)
  $core.List<$core.String> get areas => $_getList(2);
}

class NewPhoneAreaCodeResponse extends $pb.GeneratedMessage {
  factory NewPhoneAreaCodeResponse({
    $core.String? result,
  }) {
    final $result = create();
    if (result != null) {
      $result.result = result;
    }
    return $result;
  }
  NewPhoneAreaCodeResponse._() : super();
  factory NewPhoneAreaCodeResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory NewPhoneAreaCodeResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'NewPhoneAreaCodeResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'result')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  NewPhoneAreaCodeResponse clone() => NewPhoneAreaCodeResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  NewPhoneAreaCodeResponse copyWith(void Function(NewPhoneAreaCodeResponse) updates) => super.copyWith((message) => updates(message as NewPhoneAreaCodeResponse)) as NewPhoneAreaCodeResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static NewPhoneAreaCodeResponse create() => NewPhoneAreaCodeResponse._();
  NewPhoneAreaCodeResponse createEmptyInstance() => create();
  static $pb.PbList<NewPhoneAreaCodeResponse> createRepeated() => $pb.PbList<NewPhoneAreaCodeResponse>();
  @$core.pragma('dart2js:noInline')
  static NewPhoneAreaCodeResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<NewPhoneAreaCodeResponse>(create);
  static NewPhoneAreaCodeResponse? _defaultInstance;

  /// 成功返回新区号编码
  @$pb.TagNumber(1)
  $core.String get result => $_getSZ(0);
  @$pb.TagNumber(1)
  set result($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasResult() => $_has(0);
  @$pb.TagNumber(1)
  void clearResult() => clearField(1);
}

/// 取得区号编码列表, 读取不需要权限
/// 客户端应该缓存这个列表
class GetPhoneAreaCodesRequest extends $pb.GeneratedMessage {
  factory GetPhoneAreaCodesRequest() => create();
  GetPhoneAreaCodesRequest._() : super();
  factory GetPhoneAreaCodesRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetPhoneAreaCodesRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'GetPhoneAreaCodesRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetPhoneAreaCodesRequest clone() => GetPhoneAreaCodesRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetPhoneAreaCodesRequest copyWith(void Function(GetPhoneAreaCodesRequest) updates) => super.copyWith((message) => updates(message as GetPhoneAreaCodesRequest)) as GetPhoneAreaCodesRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GetPhoneAreaCodesRequest create() => GetPhoneAreaCodesRequest._();
  GetPhoneAreaCodesRequest createEmptyInstance() => create();
  static $pb.PbList<GetPhoneAreaCodesRequest> createRepeated() => $pb.PbList<GetPhoneAreaCodesRequest>();
  @$core.pragma('dart2js:noInline')
  static GetPhoneAreaCodesRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetPhoneAreaCodesRequest>(create);
  static GetPhoneAreaCodesRequest? _defaultInstance;
}

class GetPhoneAreaCodesResponse extends $pb.GeneratedMessage {
  factory GetPhoneAreaCodesResponse({
    $core.Iterable<PhoneAreaCode>? phoneAreaCodes,
  }) {
    final $result = create();
    if (phoneAreaCodes != null) {
      $result.phoneAreaCodes.addAll(phoneAreaCodes);
    }
    return $result;
  }
  GetPhoneAreaCodesResponse._() : super();
  factory GetPhoneAreaCodesResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetPhoneAreaCodesResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'GetPhoneAreaCodesResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..pc<PhoneAreaCode>(1, _omitFieldNames ? '' : 'phoneAreaCodes', $pb.PbFieldType.PM, subBuilder: PhoneAreaCode.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetPhoneAreaCodesResponse clone() => GetPhoneAreaCodesResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetPhoneAreaCodesResponse copyWith(void Function(GetPhoneAreaCodesResponse) updates) => super.copyWith((message) => updates(message as GetPhoneAreaCodesResponse)) as GetPhoneAreaCodesResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GetPhoneAreaCodesResponse create() => GetPhoneAreaCodesResponse._();
  GetPhoneAreaCodesResponse createEmptyInstance() => create();
  static $pb.PbList<GetPhoneAreaCodesResponse> createRepeated() => $pb.PbList<GetPhoneAreaCodesResponse>();
  @$core.pragma('dart2js:noInline')
  static GetPhoneAreaCodesResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetPhoneAreaCodesResponse>(create);
  static GetPhoneAreaCodesResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<PhoneAreaCode> get phoneAreaCodes => $_getList(0);
}


const _omitFieldNames = $core.bool.fromEnvironment('protobuf.omit_field_names');
const _omitMessageNames = $core.bool.fromEnvironment('protobuf.omit_message_names');
