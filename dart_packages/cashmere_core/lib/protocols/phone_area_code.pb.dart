///
//  Generated code. Do not modify.
//  source: phone_area_code.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

import 'name.pb.dart' as $0;

class PhoneAreaCode extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'PhoneAreaCode', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'code')
    ..pPS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'usingAreas')
    ..hasRequiredFields = false
  ;

  PhoneAreaCode._() : super();
  factory PhoneAreaCode({
    $core.String? code,
    $core.Iterable<$core.String>? usingAreas,
  }) {
    final _result = create();
    if (code != null) {
      _result.code = code;
    }
    if (usingAreas != null) {
      _result.usingAreas.addAll(usingAreas);
    }
    return _result;
  }
  factory PhoneAreaCode.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory PhoneAreaCode.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  PhoneAreaCode clone() => PhoneAreaCode()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  PhoneAreaCode copyWith(void Function(PhoneAreaCode) updates) => super.copyWith((message) => updates(message as PhoneAreaCode)) as PhoneAreaCode; // ignore: deprecated_member_use
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
  $core.List<$core.String> get usingAreas => $_getList(1);
}

class NewPhoneAreaCodeRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'NewPhoneAreaCodeRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOM<$0.Name>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'name', subBuilder: $0.Name.create)
    ..aOS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'code')
    ..pPS(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'areas')
    ..hasRequiredFields = false
  ;

  NewPhoneAreaCodeRequest._() : super();
  factory NewPhoneAreaCodeRequest({
    $0.Name? name,
    $core.String? code,
    $core.Iterable<$core.String>? areas,
  }) {
    final _result = create();
    if (name != null) {
      _result.name = name;
    }
    if (code != null) {
      _result.code = code;
    }
    if (areas != null) {
      _result.areas.addAll(areas);
    }
    return _result;
  }
  factory NewPhoneAreaCodeRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory NewPhoneAreaCodeRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  NewPhoneAreaCodeRequest clone() => NewPhoneAreaCodeRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  NewPhoneAreaCodeRequest copyWith(void Function(NewPhoneAreaCodeRequest) updates) => super.copyWith((message) => updates(message as NewPhoneAreaCodeRequest)) as NewPhoneAreaCodeRequest; // ignore: deprecated_member_use
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

  @$pb.TagNumber(3)
  $core.List<$core.String> get areas => $_getList(2);
}

class NewPhoneAreaCodeResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'NewPhoneAreaCodeResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'result')
    ..hasRequiredFields = false
  ;

  NewPhoneAreaCodeResponse._() : super();
  factory NewPhoneAreaCodeResponse({
    $core.String? result,
  }) {
    final _result = create();
    if (result != null) {
      _result.result = result;
    }
    return _result;
  }
  factory NewPhoneAreaCodeResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory NewPhoneAreaCodeResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  NewPhoneAreaCodeResponse clone() => NewPhoneAreaCodeResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  NewPhoneAreaCodeResponse copyWith(void Function(NewPhoneAreaCodeResponse) updates) => super.copyWith((message) => updates(message as NewPhoneAreaCodeResponse)) as NewPhoneAreaCodeResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static NewPhoneAreaCodeResponse create() => NewPhoneAreaCodeResponse._();
  NewPhoneAreaCodeResponse createEmptyInstance() => create();
  static $pb.PbList<NewPhoneAreaCodeResponse> createRepeated() => $pb.PbList<NewPhoneAreaCodeResponse>();
  @$core.pragma('dart2js:noInline')
  static NewPhoneAreaCodeResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<NewPhoneAreaCodeResponse>(create);
  static NewPhoneAreaCodeResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get result => $_getSZ(0);
  @$pb.TagNumber(1)
  set result($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasResult() => $_has(0);
  @$pb.TagNumber(1)
  void clearResult() => clearField(1);
}

