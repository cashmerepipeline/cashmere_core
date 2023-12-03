//
//  Generated code. Do not modify.
//  source: member.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

import 'name.pb.dart' as $0;

class AddMemberRequest extends $pb.GeneratedMessage {
  factory AddMemberRequest({
    $0.Name? name,
    $core.int? ownerManageId,
    $core.String? ownerEntityId,
    $core.int? selfManageId,
    $core.String? selfEntityId,
    $core.String? description,
  }) {
    final $result = create();
    if (name != null) {
      $result.name = name;
    }
    if (ownerManageId != null) {
      $result.ownerManageId = ownerManageId;
    }
    if (ownerEntityId != null) {
      $result.ownerEntityId = ownerEntityId;
    }
    if (selfManageId != null) {
      $result.selfManageId = selfManageId;
    }
    if (selfEntityId != null) {
      $result.selfEntityId = selfEntityId;
    }
    if (description != null) {
      $result.description = description;
    }
    return $result;
  }
  AddMemberRequest._() : super();
  factory AddMemberRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory AddMemberRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'AddMemberRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOM<$0.Name>(1, _omitFieldNames ? '' : 'name', subBuilder: $0.Name.create)
    ..a<$core.int>(2, _omitFieldNames ? '' : 'ownerManageId', $pb.PbFieldType.O3)
    ..aOS(3, _omitFieldNames ? '' : 'ownerEntityId')
    ..a<$core.int>(4, _omitFieldNames ? '' : 'selfManageId', $pb.PbFieldType.O3)
    ..aOS(5, _omitFieldNames ? '' : 'selfEntityId')
    ..aOS(6, _omitFieldNames ? '' : 'description')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  AddMemberRequest clone() => AddMemberRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  AddMemberRequest copyWith(void Function(AddMemberRequest) updates) => super.copyWith((message) => updates(message as AddMemberRequest)) as AddMemberRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static AddMemberRequest create() => AddMemberRequest._();
  AddMemberRequest createEmptyInstance() => create();
  static $pb.PbList<AddMemberRequest> createRepeated() => $pb.PbList<AddMemberRequest>();
  @$core.pragma('dart2js:noInline')
  static AddMemberRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<AddMemberRequest>(create);
  static AddMemberRequest? _defaultInstance;

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
  $core.int get ownerManageId => $_getIZ(1);
  @$pb.TagNumber(2)
  set ownerManageId($core.int v) { $_setSignedInt32(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasOwnerManageId() => $_has(1);
  @$pb.TagNumber(2)
  void clearOwnerManageId() => clearField(2);

  @$pb.TagNumber(3)
  $core.String get ownerEntityId => $_getSZ(2);
  @$pb.TagNumber(3)
  set ownerEntityId($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasOwnerEntityId() => $_has(2);
  @$pb.TagNumber(3)
  void clearOwnerEntityId() => clearField(3);

  @$pb.TagNumber(4)
  $core.int get selfManageId => $_getIZ(3);
  @$pb.TagNumber(4)
  set selfManageId($core.int v) { $_setSignedInt32(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasSelfManageId() => $_has(3);
  @$pb.TagNumber(4)
  void clearSelfManageId() => clearField(4);

  @$pb.TagNumber(5)
  $core.String get selfEntityId => $_getSZ(4);
  @$pb.TagNumber(5)
  set selfEntityId($core.String v) { $_setString(4, v); }
  @$pb.TagNumber(5)
  $core.bool hasSelfEntityId() => $_has(4);
  @$pb.TagNumber(5)
  void clearSelfEntityId() => clearField(5);

  @$pb.TagNumber(6)
  $core.String get description => $_getSZ(5);
  @$pb.TagNumber(6)
  set description($core.String v) { $_setString(5, v); }
  @$pb.TagNumber(6)
  $core.bool hasDescription() => $_has(5);
  @$pb.TagNumber(6)
  void clearDescription() => clearField(6);
}

class AddMemberResponse extends $pb.GeneratedMessage {
  factory AddMemberResponse({
    $core.String? result,
  }) {
    final $result = create();
    if (result != null) {
      $result.result = result;
    }
    return $result;
  }
  AddMemberResponse._() : super();
  factory AddMemberResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory AddMemberResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'AddMemberResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'result')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  AddMemberResponse clone() => AddMemberResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  AddMemberResponse copyWith(void Function(AddMemberResponse) updates) => super.copyWith((message) => updates(message as AddMemberResponse)) as AddMemberResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static AddMemberResponse create() => AddMemberResponse._();
  AddMemberResponse createEmptyInstance() => create();
  static $pb.PbList<AddMemberResponse> createRepeated() => $pb.PbList<AddMemberResponse>();
  @$core.pragma('dart2js:noInline')
  static AddMemberResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<AddMemberResponse>(create);
  static AddMemberResponse? _defaultInstance;

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
