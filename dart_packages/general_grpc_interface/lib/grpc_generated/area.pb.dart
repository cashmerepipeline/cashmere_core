///
//  Generated code. Do not modify.
//  source: area.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

import 'name.pb.dart' as $0;

import 'area.pbenum.dart';

export 'area.pbenum.dart';

class NewAreaRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'NewAreaRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOM<$0.Name>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'name', subBuilder: $0.Name.create)
    ..aOS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'parentId')
    ..aOS(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'code')
    ..e<AreaLevel>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'level', $pb.PbFieldType.OE, defaultOrMaker: AreaLevel.Country, valueOf: AreaLevel.valueOf, enumValues: AreaLevel.values)
    ..hasRequiredFields = false
  ;

  NewAreaRequest._() : super();
  factory NewAreaRequest({
    $0.Name? name,
    $core.String? parentId,
    $core.String? code,
    AreaLevel? level,
  }) {
    final _result = create();
    if (name != null) {
      _result.name = name;
    }
    if (parentId != null) {
      _result.parentId = parentId;
    }
    if (code != null) {
      _result.code = code;
    }
    if (level != null) {
      _result.level = level;
    }
    return _result;
  }
  factory NewAreaRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory NewAreaRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  NewAreaRequest clone() => NewAreaRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  NewAreaRequest copyWith(void Function(NewAreaRequest) updates) => super.copyWith((message) => updates(message as NewAreaRequest)) as NewAreaRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static NewAreaRequest create() => NewAreaRequest._();
  NewAreaRequest createEmptyInstance() => create();
  static $pb.PbList<NewAreaRequest> createRepeated() => $pb.PbList<NewAreaRequest>();
  @$core.pragma('dart2js:noInline')
  static NewAreaRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<NewAreaRequest>(create);
  static NewAreaRequest? _defaultInstance;

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
  $core.String get parentId => $_getSZ(1);
  @$pb.TagNumber(2)
  set parentId($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasParentId() => $_has(1);
  @$pb.TagNumber(2)
  void clearParentId() => clearField(2);

  @$pb.TagNumber(3)
  $core.String get code => $_getSZ(2);
  @$pb.TagNumber(3)
  set code($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasCode() => $_has(2);
  @$pb.TagNumber(3)
  void clearCode() => clearField(3);

  @$pb.TagNumber(4)
  AreaLevel get level => $_getN(3);
  @$pb.TagNumber(4)
  set level(AreaLevel v) { setField(4, v); }
  @$pb.TagNumber(4)
  $core.bool hasLevel() => $_has(3);
  @$pb.TagNumber(4)
  void clearLevel() => clearField(4);
}

class NewAreaResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'NewAreaResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'result')
    ..hasRequiredFields = false
  ;

  NewAreaResponse._() : super();
  factory NewAreaResponse({
    $core.String? result,
  }) {
    final _result = create();
    if (result != null) {
      _result.result = result;
    }
    return _result;
  }
  factory NewAreaResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory NewAreaResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  NewAreaResponse clone() => NewAreaResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  NewAreaResponse copyWith(void Function(NewAreaResponse) updates) => super.copyWith((message) => updates(message as NewAreaResponse)) as NewAreaResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static NewAreaResponse create() => NewAreaResponse._();
  NewAreaResponse createEmptyInstance() => create();
  static $pb.PbList<NewAreaResponse> createRepeated() => $pb.PbList<NewAreaResponse>();
  @$core.pragma('dart2js:noInline')
  static NewAreaResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<NewAreaResponse>(create);
  static NewAreaResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get result => $_getSZ(0);
  @$pb.TagNumber(1)
  set result($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasResult() => $_has(0);
  @$pb.TagNumber(1)
  void clearResult() => clearField(1);
}

class EditAreaRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'EditAreaRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'areaId')
    ..aOS(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'newParentId')
    ..e<AreaLevel>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'newLevel', $pb.PbFieldType.OE, defaultOrMaker: AreaLevel.Country, valueOf: AreaLevel.valueOf, enumValues: AreaLevel.values)
    ..hasRequiredFields = false
  ;

  EditAreaRequest._() : super();
  factory EditAreaRequest({
    $core.String? areaId,
    $core.String? newParentId,
    AreaLevel? newLevel,
  }) {
    final _result = create();
    if (areaId != null) {
      _result.areaId = areaId;
    }
    if (newParentId != null) {
      _result.newParentId = newParentId;
    }
    if (newLevel != null) {
      _result.newLevel = newLevel;
    }
    return _result;
  }
  factory EditAreaRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory EditAreaRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  EditAreaRequest clone() => EditAreaRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  EditAreaRequest copyWith(void Function(EditAreaRequest) updates) => super.copyWith((message) => updates(message as EditAreaRequest)) as EditAreaRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static EditAreaRequest create() => EditAreaRequest._();
  EditAreaRequest createEmptyInstance() => create();
  static $pb.PbList<EditAreaRequest> createRepeated() => $pb.PbList<EditAreaRequest>();
  @$core.pragma('dart2js:noInline')
  static EditAreaRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<EditAreaRequest>(create);
  static EditAreaRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get areaId => $_getSZ(0);
  @$pb.TagNumber(1)
  set areaId($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasAreaId() => $_has(0);
  @$pb.TagNumber(1)
  void clearAreaId() => clearField(1);

  @$pb.TagNumber(3)
  $core.String get newParentId => $_getSZ(1);
  @$pb.TagNumber(3)
  set newParentId($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(3)
  $core.bool hasNewParentId() => $_has(1);
  @$pb.TagNumber(3)
  void clearNewParentId() => clearField(3);

  @$pb.TagNumber(4)
  AreaLevel get newLevel => $_getN(2);
  @$pb.TagNumber(4)
  set newLevel(AreaLevel v) { setField(4, v); }
  @$pb.TagNumber(4)
  $core.bool hasNewLevel() => $_has(2);
  @$pb.TagNumber(4)
  void clearNewLevel() => clearField(4);
}

class EditAreaResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'EditAreaResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'result')
    ..hasRequiredFields = false
  ;

  EditAreaResponse._() : super();
  factory EditAreaResponse({
    $core.String? result,
  }) {
    final _result = create();
    if (result != null) {
      _result.result = result;
    }
    return _result;
  }
  factory EditAreaResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory EditAreaResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  EditAreaResponse clone() => EditAreaResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  EditAreaResponse copyWith(void Function(EditAreaResponse) updates) => super.copyWith((message) => updates(message as EditAreaResponse)) as EditAreaResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static EditAreaResponse create() => EditAreaResponse._();
  EditAreaResponse createEmptyInstance() => create();
  static $pb.PbList<EditAreaResponse> createRepeated() => $pb.PbList<EditAreaResponse>();
  @$core.pragma('dart2js:noInline')
  static EditAreaResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<EditAreaResponse>(create);
  static EditAreaResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get result => $_getSZ(0);
  @$pb.TagNumber(1)
  set result($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasResult() => $_has(0);
  @$pb.TagNumber(1)
  void clearResult() => clearField(1);
}

