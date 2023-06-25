///
//  Generated code. Do not modify.
//  source: country_code.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

import 'name.pb.dart' as $0;

class CountryCode extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'CountryCode', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'code')
    ..m<$core.String, $core.String>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'nameMap', entryClassName: 'CountryCode.NameMapEntry', keyFieldType: $pb.PbFieldType.OS, valueFieldType: $pb.PbFieldType.OS, packageName: const $pb.PackageName('cashmere'))
    ..aOS(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'native')
    ..aOS(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'phoneAreaCode')
    ..pPS(5, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'languages')
    ..hasRequiredFields = false
  ;

  CountryCode._() : super();
  factory CountryCode({
    $core.String? code,
    $core.Map<$core.String, $core.String>? nameMap,
    $core.String? native,
    $core.String? phoneAreaCode,
    $core.Iterable<$core.String>? languages,
  }) {
    final _result = create();
    if (code != null) {
      _result.code = code;
    }
    if (nameMap != null) {
      _result.nameMap.addAll(nameMap);
    }
    if (native != null) {
      _result.native = native;
    }
    if (phoneAreaCode != null) {
      _result.phoneAreaCode = phoneAreaCode;
    }
    if (languages != null) {
      _result.languages.addAll(languages);
    }
    return _result;
  }
  factory CountryCode.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory CountryCode.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  CountryCode clone() => CountryCode()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  CountryCode copyWith(void Function(CountryCode) updates) => super.copyWith((message) => updates(message as CountryCode)) as CountryCode; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static CountryCode create() => CountryCode._();
  CountryCode createEmptyInstance() => create();
  static $pb.PbList<CountryCode> createRepeated() => $pb.PbList<CountryCode>();
  @$core.pragma('dart2js:noInline')
  static CountryCode getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<CountryCode>(create);
  static CountryCode? _defaultInstance;

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

  @$pb.TagNumber(3)
  $core.String get native => $_getSZ(2);
  @$pb.TagNumber(3)
  set native($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasNative() => $_has(2);
  @$pb.TagNumber(3)
  void clearNative() => clearField(3);

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

class NewCountryCodeRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'NewCountryCodeRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOM<$0.Name>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'name', subBuilder: $0.Name.create)
    ..aOS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'native')
    ..aOS(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'code')
    ..aOS(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'phoneAreaCode')
    ..pPS(5, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'languages')
    ..hasRequiredFields = false
  ;

  NewCountryCodeRequest._() : super();
  factory NewCountryCodeRequest({
    $0.Name? name,
    $core.String? native,
    $core.String? code,
    $core.String? phoneAreaCode,
    $core.Iterable<$core.String>? languages,
  }) {
    final _result = create();
    if (name != null) {
      _result.name = name;
    }
    if (native != null) {
      _result.native = native;
    }
    if (code != null) {
      _result.code = code;
    }
    if (phoneAreaCode != null) {
      _result.phoneAreaCode = phoneAreaCode;
    }
    if (languages != null) {
      _result.languages.addAll(languages);
    }
    return _result;
  }
  factory NewCountryCodeRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory NewCountryCodeRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  NewCountryCodeRequest clone() => NewCountryCodeRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  NewCountryCodeRequest copyWith(void Function(NewCountryCodeRequest) updates) => super.copyWith((message) => updates(message as NewCountryCodeRequest)) as NewCountryCodeRequest; // ignore: deprecated_member_use
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
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'NewCountryCodeResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'result')
    ..hasRequiredFields = false
  ;

  NewCountryCodeResponse._() : super();
  factory NewCountryCodeResponse({
    $core.String? result,
  }) {
    final _result = create();
    if (result != null) {
      _result.result = result;
    }
    return _result;
  }
  factory NewCountryCodeResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory NewCountryCodeResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  NewCountryCodeResponse clone() => NewCountryCodeResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  NewCountryCodeResponse copyWith(void Function(NewCountryCodeResponse) updates) => super.copyWith((message) => updates(message as NewCountryCodeResponse)) as NewCountryCodeResponse; // ignore: deprecated_member_use
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

