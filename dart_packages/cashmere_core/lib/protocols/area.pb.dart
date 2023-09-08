//
//  Generated code. Do not modify.
//  source: area.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

import 'area.pbenum.dart';
import 'name.pb.dart' as $0;

export 'area.pbenum.dart';

class NewAreaRequest extends $pb.GeneratedMessage {
  factory NewAreaRequest({
    $0.Name? name,
    $core.String? parentId,
    $core.String? code,
    AreaLevel? level,
  }) {
    final $result = create();
    if (name != null) {
      $result.name = name;
    }
    if (parentId != null) {
      $result.parentId = parentId;
    }
    if (code != null) {
      $result.code = code;
    }
    if (level != null) {
      $result.level = level;
    }
    return $result;
  }
  NewAreaRequest._() : super();
  factory NewAreaRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory NewAreaRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'NewAreaRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOM<$0.Name>(1, _omitFieldNames ? '' : 'name', subBuilder: $0.Name.create)
    ..aOS(2, _omitFieldNames ? '' : 'parentId')
    ..aOS(3, _omitFieldNames ? '' : 'code')
    ..e<AreaLevel>(4, _omitFieldNames ? '' : 'level', $pb.PbFieldType.OE, defaultOrMaker: AreaLevel.Country, valueOf: AreaLevel.valueOf, enumValues: AreaLevel.values)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  NewAreaRequest clone() => NewAreaRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  NewAreaRequest copyWith(void Function(NewAreaRequest) updates) => super.copyWith((message) => updates(message as NewAreaRequest)) as NewAreaRequest;

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
  factory NewAreaResponse({
    $core.String? result,
  }) {
    final $result = create();
    if (result != null) {
      $result.result = result;
    }
    return $result;
  }
  NewAreaResponse._() : super();
  factory NewAreaResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory NewAreaResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'NewAreaResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'result')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  NewAreaResponse clone() => NewAreaResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  NewAreaResponse copyWith(void Function(NewAreaResponse) updates) => super.copyWith((message) => updates(message as NewAreaResponse)) as NewAreaResponse;

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
  factory EditAreaRequest({
    $core.String? areaId,
    $core.String? newParentId,
    AreaLevel? newLevel,
  }) {
    final $result = create();
    if (areaId != null) {
      $result.areaId = areaId;
    }
    if (newParentId != null) {
      $result.newParentId = newParentId;
    }
    if (newLevel != null) {
      $result.newLevel = newLevel;
    }
    return $result;
  }
  EditAreaRequest._() : super();
  factory EditAreaRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory EditAreaRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'EditAreaRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'areaId')
    ..aOS(3, _omitFieldNames ? '' : 'newParentId')
    ..e<AreaLevel>(4, _omitFieldNames ? '' : 'newLevel', $pb.PbFieldType.OE, defaultOrMaker: AreaLevel.Country, valueOf: AreaLevel.valueOf, enumValues: AreaLevel.values)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  EditAreaRequest clone() => EditAreaRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  EditAreaRequest copyWith(void Function(EditAreaRequest) updates) => super.copyWith((message) => updates(message as EditAreaRequest)) as EditAreaRequest;

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
  factory EditAreaResponse({
    $core.String? result,
  }) {
    final $result = create();
    if (result != null) {
      $result.result = result;
    }
    return $result;
  }
  EditAreaResponse._() : super();
  factory EditAreaResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory EditAreaResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'EditAreaResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'result')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  EditAreaResponse clone() => EditAreaResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  EditAreaResponse copyWith(void Function(EditAreaResponse) updates) => super.copyWith((message) => updates(message as EditAreaResponse)) as EditAreaResponse;

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


const _omitFieldNames = $core.bool.fromEnvironment('protobuf.omit_field_names');
const _omitMessageNames = $core.bool.fromEnvironment('protobuf.omit_message_names');
