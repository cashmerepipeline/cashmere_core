//
//  Generated code. Do not modify.
//  source: template.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

import 'name.pb.dart' as $0;

class NewTemplateRequest extends $pb.GeneratedMessage {
  factory NewTemplateRequest({
    $core.String? manageId,
    $core.Iterable<$core.List<$core.int>>? fields,
    $0.Name? name,
  }) {
    final $result = create();
    if (manageId != null) {
      $result.manageId = manageId;
    }
    if (fields != null) {
      $result.fields.addAll(fields);
    }
    if (name != null) {
      $result.name = name;
    }
    return $result;
  }
  NewTemplateRequest._() : super();
  factory NewTemplateRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory NewTemplateRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'NewTemplateRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'manageId')
    ..p<$core.List<$core.int>>(2, _omitFieldNames ? '' : 'fields', $pb.PbFieldType.PY)
    ..aOM<$0.Name>(3, _omitFieldNames ? '' : 'name', subBuilder: $0.Name.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  NewTemplateRequest clone() => NewTemplateRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  NewTemplateRequest copyWith(void Function(NewTemplateRequest) updates) => super.copyWith((message) => updates(message as NewTemplateRequest)) as NewTemplateRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static NewTemplateRequest create() => NewTemplateRequest._();
  NewTemplateRequest createEmptyInstance() => create();
  static $pb.PbList<NewTemplateRequest> createRepeated() => $pb.PbList<NewTemplateRequest>();
  @$core.pragma('dart2js:noInline')
  static NewTemplateRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<NewTemplateRequest>(create);
  static NewTemplateRequest? _defaultInstance;

  /// 模板对应管理
  @$pb.TagNumber(1)
  $core.String get manageId => $_getSZ(0);
  @$pb.TagNumber(1)
  set manageId($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasManageId() => $_has(0);
  @$pb.TagNumber(1)
  void clearManageId() => clearField(1);

  /// bons Document bytes
  @$pb.TagNumber(2)
  $core.List<$core.List<$core.int>> get fields => $_getList(1);

  @$pb.TagNumber(3)
  $0.Name get name => $_getN(2);
  @$pb.TagNumber(3)
  set name($0.Name v) { setField(3, v); }
  @$pb.TagNumber(3)
  $core.bool hasName() => $_has(2);
  @$pb.TagNumber(3)
  void clearName() => clearField(3);
  @$pb.TagNumber(3)
  $0.Name ensureName() => $_ensure(2);
}

class NewTemplateResponse extends $pb.GeneratedMessage {
  factory NewTemplateResponse({
    $core.String? result,
  }) {
    final $result = create();
    if (result != null) {
      $result.result = result;
    }
    return $result;
  }
  NewTemplateResponse._() : super();
  factory NewTemplateResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory NewTemplateResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'NewTemplateResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'result')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  NewTemplateResponse clone() => NewTemplateResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  NewTemplateResponse copyWith(void Function(NewTemplateResponse) updates) => super.copyWith((message) => updates(message as NewTemplateResponse)) as NewTemplateResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static NewTemplateResponse create() => NewTemplateResponse._();
  NewTemplateResponse createEmptyInstance() => create();
  static $pb.PbList<NewTemplateResponse> createRepeated() => $pb.PbList<NewTemplateResponse>();
  @$core.pragma('dart2js:noInline')
  static NewTemplateResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<NewTemplateResponse>(create);
  static NewTemplateResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get result => $_getSZ(0);
  @$pb.TagNumber(1)
  set result($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasResult() => $_has(0);
  @$pb.TagNumber(1)
  void clearResult() => clearField(1);
}

class EditTemplateRequest extends $pb.GeneratedMessage {
  factory EditTemplateRequest({
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
  EditTemplateRequest._() : super();
  factory EditTemplateRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory EditTemplateRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'EditTemplateRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'templateId')
    ..p<$core.List<$core.int>>(2, _omitFieldNames ? '' : 'fields', $pb.PbFieldType.PY)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  EditTemplateRequest clone() => EditTemplateRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  EditTemplateRequest copyWith(void Function(EditTemplateRequest) updates) => super.copyWith((message) => updates(message as EditTemplateRequest)) as EditTemplateRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static EditTemplateRequest create() => EditTemplateRequest._();
  EditTemplateRequest createEmptyInstance() => create();
  static $pb.PbList<EditTemplateRequest> createRepeated() => $pb.PbList<EditTemplateRequest>();
  @$core.pragma('dart2js:noInline')
  static EditTemplateRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<EditTemplateRequest>(create);
  static EditTemplateRequest? _defaultInstance;

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

class EditTemplateResponse extends $pb.GeneratedMessage {
  factory EditTemplateResponse({
    $core.String? result,
  }) {
    final $result = create();
    if (result != null) {
      $result.result = result;
    }
    return $result;
  }
  EditTemplateResponse._() : super();
  factory EditTemplateResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory EditTemplateResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'EditTemplateResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'result')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  EditTemplateResponse clone() => EditTemplateResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  EditTemplateResponse copyWith(void Function(EditTemplateResponse) updates) => super.copyWith((message) => updates(message as EditTemplateResponse)) as EditTemplateResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static EditTemplateResponse create() => EditTemplateResponse._();
  EditTemplateResponse createEmptyInstance() => create();
  static $pb.PbList<EditTemplateResponse> createRepeated() => $pb.PbList<EditTemplateResponse>();
  @$core.pragma('dart2js:noInline')
  static EditTemplateResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<EditTemplateResponse>(create);
  static EditTemplateResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get result => $_getSZ(0);
  @$pb.TagNumber(1)
  set result($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasResult() => $_has(0);
  @$pb.TagNumber(1)
  void clearResult() => clearField(1);
}

class RemoveTemplateRequest extends $pb.GeneratedMessage {
  factory RemoveTemplateRequest({
    $core.String? templateId,
  }) {
    final $result = create();
    if (templateId != null) {
      $result.templateId = templateId;
    }
    return $result;
  }
  RemoveTemplateRequest._() : super();
  factory RemoveTemplateRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory RemoveTemplateRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'RemoveTemplateRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'templateId')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  RemoveTemplateRequest clone() => RemoveTemplateRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  RemoveTemplateRequest copyWith(void Function(RemoveTemplateRequest) updates) => super.copyWith((message) => updates(message as RemoveTemplateRequest)) as RemoveTemplateRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static RemoveTemplateRequest create() => RemoveTemplateRequest._();
  RemoveTemplateRequest createEmptyInstance() => create();
  static $pb.PbList<RemoveTemplateRequest> createRepeated() => $pb.PbList<RemoveTemplateRequest>();
  @$core.pragma('dart2js:noInline')
  static RemoveTemplateRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<RemoveTemplateRequest>(create);
  static RemoveTemplateRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get templateId => $_getSZ(0);
  @$pb.TagNumber(1)
  set templateId($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasTemplateId() => $_has(0);
  @$pb.TagNumber(1)
  void clearTemplateId() => clearField(1);
}

class RemoveTemplateResponse extends $pb.GeneratedMessage {
  factory RemoveTemplateResponse({
    $core.String? result,
  }) {
    final $result = create();
    if (result != null) {
      $result.result = result;
    }
    return $result;
  }
  RemoveTemplateResponse._() : super();
  factory RemoveTemplateResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory RemoveTemplateResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'RemoveTemplateResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'result')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  RemoveTemplateResponse clone() => RemoveTemplateResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  RemoveTemplateResponse copyWith(void Function(RemoveTemplateResponse) updates) => super.copyWith((message) => updates(message as RemoveTemplateResponse)) as RemoveTemplateResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static RemoveTemplateResponse create() => RemoveTemplateResponse._();
  RemoveTemplateResponse createEmptyInstance() => create();
  static $pb.PbList<RemoveTemplateResponse> createRepeated() => $pb.PbList<RemoveTemplateResponse>();
  @$core.pragma('dart2js:noInline')
  static RemoveTemplateResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<RemoveTemplateResponse>(create);
  static RemoveTemplateResponse? _defaultInstance;

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
