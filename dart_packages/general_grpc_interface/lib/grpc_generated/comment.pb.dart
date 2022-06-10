///
//  Generated code. Do not modify.
//  source: comment.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

class NewCommentRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'NewCommentRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'targetManageId')
    ..aOS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'targetEntityId')
    ..aOS(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'contents')
    ..hasRequiredFields = false
  ;

  NewCommentRequest._() : super();
  factory NewCommentRequest({
    $core.String? targetManageId,
    $core.String? targetEntityId,
    $core.String? contents,
  }) {
    final _result = create();
    if (targetManageId != null) {
      _result.targetManageId = targetManageId;
    }
    if (targetEntityId != null) {
      _result.targetEntityId = targetEntityId;
    }
    if (contents != null) {
      _result.contents = contents;
    }
    return _result;
  }
  factory NewCommentRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory NewCommentRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  NewCommentRequest clone() => NewCommentRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  NewCommentRequest copyWith(void Function(NewCommentRequest) updates) => super.copyWith((message) => updates(message as NewCommentRequest)) as NewCommentRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static NewCommentRequest create() => NewCommentRequest._();
  NewCommentRequest createEmptyInstance() => create();
  static $pb.PbList<NewCommentRequest> createRepeated() => $pb.PbList<NewCommentRequest>();
  @$core.pragma('dart2js:noInline')
  static NewCommentRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<NewCommentRequest>(create);
  static NewCommentRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get targetManageId => $_getSZ(0);
  @$pb.TagNumber(1)
  set targetManageId($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasTargetManageId() => $_has(0);
  @$pb.TagNumber(1)
  void clearTargetManageId() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get targetEntityId => $_getSZ(1);
  @$pb.TagNumber(2)
  set targetEntityId($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasTargetEntityId() => $_has(1);
  @$pb.TagNumber(2)
  void clearTargetEntityId() => clearField(2);

  @$pb.TagNumber(3)
  $core.String get contents => $_getSZ(2);
  @$pb.TagNumber(3)
  set contents($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasContents() => $_has(2);
  @$pb.TagNumber(3)
  void clearContents() => clearField(3);
}

class NewCommentResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'NewCommentResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'result')
    ..hasRequiredFields = false
  ;

  NewCommentResponse._() : super();
  factory NewCommentResponse({
    $core.String? result,
  }) {
    final _result = create();
    if (result != null) {
      _result.result = result;
    }
    return _result;
  }
  factory NewCommentResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory NewCommentResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  NewCommentResponse clone() => NewCommentResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  NewCommentResponse copyWith(void Function(NewCommentResponse) updates) => super.copyWith((message) => updates(message as NewCommentResponse)) as NewCommentResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static NewCommentResponse create() => NewCommentResponse._();
  NewCommentResponse createEmptyInstance() => create();
  static $pb.PbList<NewCommentResponse> createRepeated() => $pb.PbList<NewCommentResponse>();
  @$core.pragma('dart2js:noInline')
  static NewCommentResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<NewCommentResponse>(create);
  static NewCommentResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get result => $_getSZ(0);
  @$pb.TagNumber(1)
  set result($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasResult() => $_has(0);
  @$pb.TagNumber(1)
  void clearResult() => clearField(1);
}

class EditCommentRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'EditCommentRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'commentId')
    ..aOS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'newContents')
    ..hasRequiredFields = false
  ;

  EditCommentRequest._() : super();
  factory EditCommentRequest({
    $core.String? commentId,
    $core.String? newContents,
  }) {
    final _result = create();
    if (commentId != null) {
      _result.commentId = commentId;
    }
    if (newContents != null) {
      _result.newContents = newContents;
    }
    return _result;
  }
  factory EditCommentRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory EditCommentRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  EditCommentRequest clone() => EditCommentRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  EditCommentRequest copyWith(void Function(EditCommentRequest) updates) => super.copyWith((message) => updates(message as EditCommentRequest)) as EditCommentRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static EditCommentRequest create() => EditCommentRequest._();
  EditCommentRequest createEmptyInstance() => create();
  static $pb.PbList<EditCommentRequest> createRepeated() => $pb.PbList<EditCommentRequest>();
  @$core.pragma('dart2js:noInline')
  static EditCommentRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<EditCommentRequest>(create);
  static EditCommentRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get commentId => $_getSZ(0);
  @$pb.TagNumber(1)
  set commentId($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasCommentId() => $_has(0);
  @$pb.TagNumber(1)
  void clearCommentId() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get newContents => $_getSZ(1);
  @$pb.TagNumber(2)
  set newContents($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasNewContents() => $_has(1);
  @$pb.TagNumber(2)
  void clearNewContents() => clearField(2);
}

class EditCommentResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'EditCommentResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'result')
    ..hasRequiredFields = false
  ;

  EditCommentResponse._() : super();
  factory EditCommentResponse({
    $core.String? result,
  }) {
    final _result = create();
    if (result != null) {
      _result.result = result;
    }
    return _result;
  }
  factory EditCommentResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory EditCommentResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  EditCommentResponse clone() => EditCommentResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  EditCommentResponse copyWith(void Function(EditCommentResponse) updates) => super.copyWith((message) => updates(message as EditCommentResponse)) as EditCommentResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static EditCommentResponse create() => EditCommentResponse._();
  EditCommentResponse createEmptyInstance() => create();
  static $pb.PbList<EditCommentResponse> createRepeated() => $pb.PbList<EditCommentResponse>();
  @$core.pragma('dart2js:noInline')
  static EditCommentResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<EditCommentResponse>(create);
  static EditCommentResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get result => $_getSZ(0);
  @$pb.TagNumber(1)
  set result($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasResult() => $_has(0);
  @$pb.TagNumber(1)
  void clearResult() => clearField(1);
}

class RemoveCommentRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'RemoveCommentRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'targetManageId')
    ..aOS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'targetEntityId')
    ..aOS(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'commentId')
    ..hasRequiredFields = false
  ;

  RemoveCommentRequest._() : super();
  factory RemoveCommentRequest({
    $core.String? targetManageId,
    $core.String? targetEntityId,
    $core.String? commentId,
  }) {
    final _result = create();
    if (targetManageId != null) {
      _result.targetManageId = targetManageId;
    }
    if (targetEntityId != null) {
      _result.targetEntityId = targetEntityId;
    }
    if (commentId != null) {
      _result.commentId = commentId;
    }
    return _result;
  }
  factory RemoveCommentRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory RemoveCommentRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  RemoveCommentRequest clone() => RemoveCommentRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  RemoveCommentRequest copyWith(void Function(RemoveCommentRequest) updates) => super.copyWith((message) => updates(message as RemoveCommentRequest)) as RemoveCommentRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static RemoveCommentRequest create() => RemoveCommentRequest._();
  RemoveCommentRequest createEmptyInstance() => create();
  static $pb.PbList<RemoveCommentRequest> createRepeated() => $pb.PbList<RemoveCommentRequest>();
  @$core.pragma('dart2js:noInline')
  static RemoveCommentRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<RemoveCommentRequest>(create);
  static RemoveCommentRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get targetManageId => $_getSZ(0);
  @$pb.TagNumber(1)
  set targetManageId($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasTargetManageId() => $_has(0);
  @$pb.TagNumber(1)
  void clearTargetManageId() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get targetEntityId => $_getSZ(1);
  @$pb.TagNumber(2)
  set targetEntityId($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasTargetEntityId() => $_has(1);
  @$pb.TagNumber(2)
  void clearTargetEntityId() => clearField(2);

  @$pb.TagNumber(3)
  $core.String get commentId => $_getSZ(2);
  @$pb.TagNumber(3)
  set commentId($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasCommentId() => $_has(2);
  @$pb.TagNumber(3)
  void clearCommentId() => clearField(3);
}

class RemoveCommentResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'RemoveCommentResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'result')
    ..hasRequiredFields = false
  ;

  RemoveCommentResponse._() : super();
  factory RemoveCommentResponse({
    $core.String? result,
  }) {
    final _result = create();
    if (result != null) {
      _result.result = result;
    }
    return _result;
  }
  factory RemoveCommentResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory RemoveCommentResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  RemoveCommentResponse clone() => RemoveCommentResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  RemoveCommentResponse copyWith(void Function(RemoveCommentResponse) updates) => super.copyWith((message) => updates(message as RemoveCommentResponse)) as RemoveCommentResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static RemoveCommentResponse create() => RemoveCommentResponse._();
  RemoveCommentResponse createEmptyInstance() => create();
  static $pb.PbList<RemoveCommentResponse> createRepeated() => $pb.PbList<RemoveCommentResponse>();
  @$core.pragma('dart2js:noInline')
  static RemoveCommentResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<RemoveCommentResponse>(create);
  static RemoveCommentResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get result => $_getSZ(0);
  @$pb.TagNumber(1)
  set result($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasResult() => $_has(0);
  @$pb.TagNumber(1)
  void clearResult() => clearField(1);
}

