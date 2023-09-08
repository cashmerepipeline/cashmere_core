//
//  Generated code. Do not modify.
//  source: group.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

import 'name.pb.dart' as $0;

class NewGroupRequest extends $pb.GeneratedMessage {
  factory NewGroupRequest({
    $0.Name? name,
    $core.String? newGroupId,
  }) {
    final $result = create();
    if (name != null) {
      $result.name = name;
    }
    if (newGroupId != null) {
      $result.newGroupId = newGroupId;
    }
    return $result;
  }
  NewGroupRequest._() : super();
  factory NewGroupRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory NewGroupRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'NewGroupRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOM<$0.Name>(1, _omitFieldNames ? '' : 'name', subBuilder: $0.Name.create)
    ..aOS(2, _omitFieldNames ? '' : 'newGroupId')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  NewGroupRequest clone() => NewGroupRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  NewGroupRequest copyWith(void Function(NewGroupRequest) updates) => super.copyWith((message) => updates(message as NewGroupRequest)) as NewGroupRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static NewGroupRequest create() => NewGroupRequest._();
  NewGroupRequest createEmptyInstance() => create();
  static $pb.PbList<NewGroupRequest> createRepeated() => $pb.PbList<NewGroupRequest>();
  @$core.pragma('dart2js:noInline')
  static NewGroupRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<NewGroupRequest>(create);
  static NewGroupRequest? _defaultInstance;

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
  $core.String get newGroupId => $_getSZ(1);
  @$pb.TagNumber(2)
  set newGroupId($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasNewGroupId() => $_has(1);
  @$pb.TagNumber(2)
  void clearNewGroupId() => clearField(2);
}

class NewGroupResponse extends $pb.GeneratedMessage {
  factory NewGroupResponse({
    $core.String? result,
  }) {
    final $result = create();
    if (result != null) {
      $result.result = result;
    }
    return $result;
  }
  NewGroupResponse._() : super();
  factory NewGroupResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory NewGroupResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'NewGroupResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'result')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  NewGroupResponse clone() => NewGroupResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  NewGroupResponse copyWith(void Function(NewGroupResponse) updates) => super.copyWith((message) => updates(message as NewGroupResponse)) as NewGroupResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static NewGroupResponse create() => NewGroupResponse._();
  NewGroupResponse createEmptyInstance() => create();
  static $pb.PbList<NewGroupResponse> createRepeated() => $pb.PbList<NewGroupResponse>();
  @$core.pragma('dart2js:noInline')
  static NewGroupResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<NewGroupResponse>(create);
  static NewGroupResponse? _defaultInstance;

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
