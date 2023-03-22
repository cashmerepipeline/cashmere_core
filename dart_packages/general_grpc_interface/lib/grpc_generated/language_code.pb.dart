///
//  Generated code. Do not modify.
//  source: language_code.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

import 'name.pb.dart' as $0;

class LanguageCode extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'LanguageCode', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'code')
    ..a<$core.List<$core.int>>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'nameMap', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  LanguageCode._() : super();
  factory LanguageCode({
    $core.String? code,
    $core.List<$core.int>? nameMap,
  }) {
    final _result = create();
    if (code != null) {
      _result.code = code;
    }
    if (nameMap != null) {
      _result.nameMap = nameMap;
    }
    return _result;
  }
  factory LanguageCode.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory LanguageCode.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  LanguageCode clone() => LanguageCode()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  LanguageCode copyWith(void Function(LanguageCode) updates) => super.copyWith((message) => updates(message as LanguageCode)) as LanguageCode; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static LanguageCode create() => LanguageCode._();
  LanguageCode createEmptyInstance() => create();
  static $pb.PbList<LanguageCode> createRepeated() => $pb.PbList<LanguageCode>();
  @$core.pragma('dart2js:noInline')
  static LanguageCode getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<LanguageCode>(create);
  static LanguageCode? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get code => $_getSZ(0);
  @$pb.TagNumber(1)
  set code($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasCode() => $_has(0);
  @$pb.TagNumber(1)
  void clearCode() => clearField(1);

  @$pb.TagNumber(2)
  $core.List<$core.int> get nameMap => $_getN(1);
  @$pb.TagNumber(2)
  set nameMap($core.List<$core.int> v) { $_setBytes(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasNameMap() => $_has(1);
  @$pb.TagNumber(2)
  void clearNameMap() => clearField(2);
}

class NewLanguageCodeRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'NewLanguageCodeRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOM<$0.Name>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'name', subBuilder: $0.Name.create)
    ..aOS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'code')
    ..aOS(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'nativeName')
    ..hasRequiredFields = false
  ;

  NewLanguageCodeRequest._() : super();
  factory NewLanguageCodeRequest({
    $0.Name? name,
    $core.String? code,
    $core.String? nativeName,
  }) {
    final _result = create();
    if (name != null) {
      _result.name = name;
    }
    if (code != null) {
      _result.code = code;
    }
    if (nativeName != null) {
      _result.nativeName = nativeName;
    }
    return _result;
  }
  factory NewLanguageCodeRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory NewLanguageCodeRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  NewLanguageCodeRequest clone() => NewLanguageCodeRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  NewLanguageCodeRequest copyWith(void Function(NewLanguageCodeRequest) updates) => super.copyWith((message) => updates(message as NewLanguageCodeRequest)) as NewLanguageCodeRequest; // ignore: deprecated_member_use
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
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'NewLanguageCodeResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'result')
    ..hasRequiredFields = false
  ;

  NewLanguageCodeResponse._() : super();
  factory NewLanguageCodeResponse({
    $core.String? result,
  }) {
    final _result = create();
    if (result != null) {
      _result.result = result;
    }
    return _result;
  }
  factory NewLanguageCodeResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory NewLanguageCodeResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  NewLanguageCodeResponse clone() => NewLanguageCodeResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  NewLanguageCodeResponse copyWith(void Function(NewLanguageCodeResponse) updates) => super.copyWith((message) => updates(message as NewLanguageCodeResponse)) as NewLanguageCodeResponse; // ignore: deprecated_member_use
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

class EditLanguageCodeRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'EditLanguageCodeRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'id')
    ..aOS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'newCode')
    ..aOS(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'newNative')
    ..hasRequiredFields = false
  ;

  EditLanguageCodeRequest._() : super();
  factory EditLanguageCodeRequest({
    $core.String? id,
    $core.String? newCode,
    $core.String? newNative,
  }) {
    final _result = create();
    if (id != null) {
      _result.id = id;
    }
    if (newCode != null) {
      _result.newCode = newCode;
    }
    if (newNative != null) {
      _result.newNative = newNative;
    }
    return _result;
  }
  factory EditLanguageCodeRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory EditLanguageCodeRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  EditLanguageCodeRequest clone() => EditLanguageCodeRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  EditLanguageCodeRequest copyWith(void Function(EditLanguageCodeRequest) updates) => super.copyWith((message) => updates(message as EditLanguageCodeRequest)) as EditLanguageCodeRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static EditLanguageCodeRequest create() => EditLanguageCodeRequest._();
  EditLanguageCodeRequest createEmptyInstance() => create();
  static $pb.PbList<EditLanguageCodeRequest> createRepeated() => $pb.PbList<EditLanguageCodeRequest>();
  @$core.pragma('dart2js:noInline')
  static EditLanguageCodeRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<EditLanguageCodeRequest>(create);
  static EditLanguageCodeRequest? _defaultInstance;

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

class EditLanguageCodeResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'EditLanguageCodeResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'result')
    ..hasRequiredFields = false
  ;

  EditLanguageCodeResponse._() : super();
  factory EditLanguageCodeResponse({
    $core.String? result,
  }) {
    final _result = create();
    if (result != null) {
      _result.result = result;
    }
    return _result;
  }
  factory EditLanguageCodeResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory EditLanguageCodeResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  EditLanguageCodeResponse clone() => EditLanguageCodeResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  EditLanguageCodeResponse copyWith(void Function(EditLanguageCodeResponse) updates) => super.copyWith((message) => updates(message as EditLanguageCodeResponse)) as EditLanguageCodeResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static EditLanguageCodeResponse create() => EditLanguageCodeResponse._();
  EditLanguageCodeResponse createEmptyInstance() => create();
  static $pb.PbList<EditLanguageCodeResponse> createRepeated() => $pb.PbList<EditLanguageCodeResponse>();
  @$core.pragma('dart2js:noInline')
  static EditLanguageCodeResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<EditLanguageCodeResponse>(create);
  static EditLanguageCodeResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get result => $_getSZ(0);
  @$pb.TagNumber(1)
  set result($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasResult() => $_has(0);
  @$pb.TagNumber(1)
  void clearResult() => clearField(1);
}

