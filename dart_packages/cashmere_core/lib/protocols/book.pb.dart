//
//  Generated code. Do not modify.
//  source: book.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

import 'calendar.pb.dart' as $1;
import 'name.pb.dart' as $0;

class NewBookRequest extends $pb.GeneratedMessage {
  factory NewBookRequest({
    $core.int? manageId,
    $core.String? entityId,
    $0.Name? name,
    $core.String? description,
  }) {
    final $result = create();
    if (manageId != null) {
      $result.manageId = manageId;
    }
    if (entityId != null) {
      $result.entityId = entityId;
    }
    if (name != null) {
      $result.name = name;
    }
    if (description != null) {
      $result.description = description;
    }
    return $result;
  }
  NewBookRequest._() : super();
  factory NewBookRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory NewBookRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'NewBookRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..a<$core.int>(1, _omitFieldNames ? '' : 'manageId', $pb.PbFieldType.O3)
    ..aOS(2, _omitFieldNames ? '' : 'entityId')
    ..aOM<$0.Name>(3, _omitFieldNames ? '' : 'name', subBuilder: $0.Name.create)
    ..aOS(4, _omitFieldNames ? '' : 'description')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  NewBookRequest clone() => NewBookRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  NewBookRequest copyWith(void Function(NewBookRequest) updates) => super.copyWith((message) => updates(message as NewBookRequest)) as NewBookRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static NewBookRequest create() => NewBookRequest._();
  NewBookRequest createEmptyInstance() => create();
  static $pb.PbList<NewBookRequest> createRepeated() => $pb.PbList<NewBookRequest>();
  @$core.pragma('dart2js:noInline')
  static NewBookRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<NewBookRequest>(create);
  static NewBookRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get manageId => $_getIZ(0);
  @$pb.TagNumber(1)
  set manageId($core.int v) { $_setSignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasManageId() => $_has(0);
  @$pb.TagNumber(1)
  void clearManageId() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get entityId => $_getSZ(1);
  @$pb.TagNumber(2)
  set entityId($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasEntityId() => $_has(1);
  @$pb.TagNumber(2)
  void clearEntityId() => clearField(2);

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

  @$pb.TagNumber(4)
  $core.String get description => $_getSZ(3);
  @$pb.TagNumber(4)
  set description($core.String v) { $_setString(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasDescription() => $_has(3);
  @$pb.TagNumber(4)
  void clearDescription() => clearField(4);
}

class NewBookResponse extends $pb.GeneratedMessage {
  factory NewBookResponse({
    $core.String? result,
  }) {
    final $result = create();
    if (result != null) {
      $result.result = result;
    }
    return $result;
  }
  NewBookResponse._() : super();
  factory NewBookResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory NewBookResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'NewBookResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'result')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  NewBookResponse clone() => NewBookResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  NewBookResponse copyWith(void Function(NewBookResponse) updates) => super.copyWith((message) => updates(message as NewBookResponse)) as NewBookResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static NewBookResponse create() => NewBookResponse._();
  NewBookResponse createEmptyInstance() => create();
  static $pb.PbList<NewBookResponse> createRepeated() => $pb.PbList<NewBookResponse>();
  @$core.pragma('dart2js:noInline')
  static NewBookResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<NewBookResponse>(create);
  static NewBookResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get result => $_getSZ(0);
  @$pb.TagNumber(1)
  set result($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasResult() => $_has(0);
  @$pb.TagNumber(1)
  void clearResult() => clearField(1);
}

/// 添加日历
class AddCalendarRequest extends $pb.GeneratedMessage {
  factory AddCalendarRequest({
    $core.String? bookId,
    $1.Calendar? calendar,
    $core.String? description,
  }) {
    final $result = create();
    if (bookId != null) {
      $result.bookId = bookId;
    }
    if (calendar != null) {
      $result.calendar = calendar;
    }
    if (description != null) {
      $result.description = description;
    }
    return $result;
  }
  AddCalendarRequest._() : super();
  factory AddCalendarRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory AddCalendarRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'AddCalendarRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'bookId')
    ..aOM<$1.Calendar>(2, _omitFieldNames ? '' : 'calendar', subBuilder: $1.Calendar.create)
    ..aOS(3, _omitFieldNames ? '' : 'description')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  AddCalendarRequest clone() => AddCalendarRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  AddCalendarRequest copyWith(void Function(AddCalendarRequest) updates) => super.copyWith((message) => updates(message as AddCalendarRequest)) as AddCalendarRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static AddCalendarRequest create() => AddCalendarRequest._();
  AddCalendarRequest createEmptyInstance() => create();
  static $pb.PbList<AddCalendarRequest> createRepeated() => $pb.PbList<AddCalendarRequest>();
  @$core.pragma('dart2js:noInline')
  static AddCalendarRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<AddCalendarRequest>(create);
  static AddCalendarRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get bookId => $_getSZ(0);
  @$pb.TagNumber(1)
  set bookId($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasBookId() => $_has(0);
  @$pb.TagNumber(1)
  void clearBookId() => clearField(1);

  @$pb.TagNumber(2)
  $1.Calendar get calendar => $_getN(1);
  @$pb.TagNumber(2)
  set calendar($1.Calendar v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasCalendar() => $_has(1);
  @$pb.TagNumber(2)
  void clearCalendar() => clearField(2);
  @$pb.TagNumber(2)
  $1.Calendar ensureCalendar() => $_ensure(1);

  @$pb.TagNumber(3)
  $core.String get description => $_getSZ(2);
  @$pb.TagNumber(3)
  set description($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasDescription() => $_has(2);
  @$pb.TagNumber(3)
  void clearDescription() => clearField(3);
}

class AddCalendarResponse extends $pb.GeneratedMessage {
  factory AddCalendarResponse({
    $core.String? result,
  }) {
    final $result = create();
    if (result != null) {
      $result.result = result;
    }
    return $result;
  }
  AddCalendarResponse._() : super();
  factory AddCalendarResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory AddCalendarResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'AddCalendarResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'result')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  AddCalendarResponse clone() => AddCalendarResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  AddCalendarResponse copyWith(void Function(AddCalendarResponse) updates) => super.copyWith((message) => updates(message as AddCalendarResponse)) as AddCalendarResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static AddCalendarResponse create() => AddCalendarResponse._();
  AddCalendarResponse createEmptyInstance() => create();
  static $pb.PbList<AddCalendarResponse> createRepeated() => $pb.PbList<AddCalendarResponse>();
  @$core.pragma('dart2js:noInline')
  static AddCalendarResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<AddCalendarResponse>(create);
  static AddCalendarResponse? _defaultInstance;

  /// 成功返回新日历编码，失败返回信息
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
