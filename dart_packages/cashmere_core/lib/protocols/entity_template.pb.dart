//
//  Generated code. Do not modify.
//  source: entity_template.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

class NewEntityTemplateRequest extends $pb.GeneratedMessage {
  factory NewEntityTemplateRequest({
    $core.int? manageId,
    $core.Iterable<$core.List<$core.int>>? fields,
  }) {
    final $result = create();
    if (manageId != null) {
      $result.manageId = manageId;
    }
    if (fields != null) {
      $result.fields.addAll(fields);
    }
    return $result;
  }
  NewEntityTemplateRequest._() : super();
  factory NewEntityTemplateRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory NewEntityTemplateRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'NewEntityTemplateRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..a<$core.int>(1, _omitFieldNames ? '' : 'manageId', $pb.PbFieldType.O3)
    ..p<$core.List<$core.int>>(2, _omitFieldNames ? '' : 'fields', $pb.PbFieldType.PY)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  NewEntityTemplateRequest clone() => NewEntityTemplateRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  NewEntityTemplateRequest copyWith(void Function(NewEntityTemplateRequest) updates) => super.copyWith((message) => updates(message as NewEntityTemplateRequest)) as NewEntityTemplateRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static NewEntityTemplateRequest create() => NewEntityTemplateRequest._();
  NewEntityTemplateRequest createEmptyInstance() => create();
  static $pb.PbList<NewEntityTemplateRequest> createRepeated() => $pb.PbList<NewEntityTemplateRequest>();
  @$core.pragma('dart2js:noInline')
  static NewEntityTemplateRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<NewEntityTemplateRequest>(create);
  static NewEntityTemplateRequest? _defaultInstance;

  /// 模板对应管理
  @$pb.TagNumber(1)
  $core.int get manageId => $_getIZ(0);
  @$pb.TagNumber(1)
  set manageId($core.int v) { $_setSignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasManageId() => $_has(0);
  @$pb.TagNumber(1)
  void clearManageId() => clearField(1);

  /// 属性:值 列表
  @$pb.TagNumber(2)
  $core.List<$core.List<$core.int>> get fields => $_getList(1);
}

class NewEntityTemplateResponse extends $pb.GeneratedMessage {
  factory NewEntityTemplateResponse({
    $core.String? result,
  }) {
    final $result = create();
    if (result != null) {
      $result.result = result;
    }
    return $result;
  }
  NewEntityTemplateResponse._() : super();
  factory NewEntityTemplateResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory NewEntityTemplateResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'NewEntityTemplateResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'result')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  NewEntityTemplateResponse clone() => NewEntityTemplateResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  NewEntityTemplateResponse copyWith(void Function(NewEntityTemplateResponse) updates) => super.copyWith((message) => updates(message as NewEntityTemplateResponse)) as NewEntityTemplateResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static NewEntityTemplateResponse create() => NewEntityTemplateResponse._();
  NewEntityTemplateResponse createEmptyInstance() => create();
  static $pb.PbList<NewEntityTemplateResponse> createRepeated() => $pb.PbList<NewEntityTemplateResponse>();
  @$core.pragma('dart2js:noInline')
  static NewEntityTemplateResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<NewEntityTemplateResponse>(create);
  static NewEntityTemplateResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get result => $_getSZ(0);
  @$pb.TagNumber(1)
  set result($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasResult() => $_has(0);
  @$pb.TagNumber(1)
  void clearResult() => clearField(1);
}

class EditEntityTemplateRequest extends $pb.GeneratedMessage {
  factory EditEntityTemplateRequest({
    $core.String? templateId,
    $core.Iterable<$core.List<$core.int>>? fields,
  }) {
    final $result = create();
    if (templateId != null) {
      $result.templateId = templateId;
    }
    if (fields != null) {
      $result.fields.addAll(fields);
    }
    return $result;
  }
  EditEntityTemplateRequest._() : super();
  factory EditEntityTemplateRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory EditEntityTemplateRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'EditEntityTemplateRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'templateId')
    ..p<$core.List<$core.int>>(2, _omitFieldNames ? '' : 'fields', $pb.PbFieldType.PY)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  EditEntityTemplateRequest clone() => EditEntityTemplateRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  EditEntityTemplateRequest copyWith(void Function(EditEntityTemplateRequest) updates) => super.copyWith((message) => updates(message as EditEntityTemplateRequest)) as EditEntityTemplateRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static EditEntityTemplateRequest create() => EditEntityTemplateRequest._();
  EditEntityTemplateRequest createEmptyInstance() => create();
  static $pb.PbList<EditEntityTemplateRequest> createRepeated() => $pb.PbList<EditEntityTemplateRequest>();
  @$core.pragma('dart2js:noInline')
  static EditEntityTemplateRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<EditEntityTemplateRequest>(create);
  static EditEntityTemplateRequest? _defaultInstance;

  /// 模板编号
  @$pb.TagNumber(1)
  $core.String get templateId => $_getSZ(0);
  @$pb.TagNumber(1)
  set templateId($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasTemplateId() => $_has(0);
  @$pb.TagNumber(1)
  void clearTemplateId() => clearField(1);

  /// 属性:值 列表
  @$pb.TagNumber(2)
  $core.List<$core.List<$core.int>> get fields => $_getList(1);
}

class EditEntityTemplateResponse extends $pb.GeneratedMessage {
  factory EditEntityTemplateResponse({
    $core.String? result,
  }) {
    final $result = create();
    if (result != null) {
      $result.result = result;
    }
    return $result;
  }
  EditEntityTemplateResponse._() : super();
  factory EditEntityTemplateResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory EditEntityTemplateResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'EditEntityTemplateResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'result')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  EditEntityTemplateResponse clone() => EditEntityTemplateResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  EditEntityTemplateResponse copyWith(void Function(EditEntityTemplateResponse) updates) => super.copyWith((message) => updates(message as EditEntityTemplateResponse)) as EditEntityTemplateResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static EditEntityTemplateResponse create() => EditEntityTemplateResponse._();
  EditEntityTemplateResponse createEmptyInstance() => create();
  static $pb.PbList<EditEntityTemplateResponse> createRepeated() => $pb.PbList<EditEntityTemplateResponse>();
  @$core.pragma('dart2js:noInline')
  static EditEntityTemplateResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<EditEntityTemplateResponse>(create);
  static EditEntityTemplateResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get result => $_getSZ(0);
  @$pb.TagNumber(1)
  set result($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasResult() => $_has(0);
  @$pb.TagNumber(1)
  void clearResult() => clearField(1);
}

class RemoveEntityTemplateRequest extends $pb.GeneratedMessage {
  factory RemoveEntityTemplateRequest({
    $core.String? templateId,
  }) {
    final $result = create();
    if (templateId != null) {
      $result.templateId = templateId;
    }
    return $result;
  }
  RemoveEntityTemplateRequest._() : super();
  factory RemoveEntityTemplateRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory RemoveEntityTemplateRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'RemoveEntityTemplateRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'templateId')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  RemoveEntityTemplateRequest clone() => RemoveEntityTemplateRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  RemoveEntityTemplateRequest copyWith(void Function(RemoveEntityTemplateRequest) updates) => super.copyWith((message) => updates(message as RemoveEntityTemplateRequest)) as RemoveEntityTemplateRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static RemoveEntityTemplateRequest create() => RemoveEntityTemplateRequest._();
  RemoveEntityTemplateRequest createEmptyInstance() => create();
  static $pb.PbList<RemoveEntityTemplateRequest> createRepeated() => $pb.PbList<RemoveEntityTemplateRequest>();
  @$core.pragma('dart2js:noInline')
  static RemoveEntityTemplateRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<RemoveEntityTemplateRequest>(create);
  static RemoveEntityTemplateRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get templateId => $_getSZ(0);
  @$pb.TagNumber(1)
  set templateId($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasTemplateId() => $_has(0);
  @$pb.TagNumber(1)
  void clearTemplateId() => clearField(1);
}

class RemoveEntityTemplateResponse extends $pb.GeneratedMessage {
  factory RemoveEntityTemplateResponse({
    $core.String? result,
  }) {
    final $result = create();
    if (result != null) {
      $result.result = result;
    }
    return $result;
  }
  RemoveEntityTemplateResponse._() : super();
  factory RemoveEntityTemplateResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory RemoveEntityTemplateResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'RemoveEntityTemplateResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'result')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  RemoveEntityTemplateResponse clone() => RemoveEntityTemplateResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  RemoveEntityTemplateResponse copyWith(void Function(RemoveEntityTemplateResponse) updates) => super.copyWith((message) => updates(message as RemoveEntityTemplateResponse)) as RemoveEntityTemplateResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static RemoveEntityTemplateResponse create() => RemoveEntityTemplateResponse._();
  RemoveEntityTemplateResponse createEmptyInstance() => create();
  static $pb.PbList<RemoveEntityTemplateResponse> createRepeated() => $pb.PbList<RemoveEntityTemplateResponse>();
  @$core.pragma('dart2js:noInline')
  static RemoveEntityTemplateResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<RemoveEntityTemplateResponse>(create);
  static RemoveEntityTemplateResponse? _defaultInstance;

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
