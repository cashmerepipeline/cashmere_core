//
//  Generated code. Do not modify.
//  source: calendar_book.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

import 'name.pb.dart' as $0;

class NewCalendarBookRequest extends $pb.GeneratedMessage {
  factory NewCalendarBookRequest({
    $core.String? manageId,
    $core.String? entityId,
    $0.Name? name,
    $core.String? description,
    $core.String? mark,
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
    if (mark != null) {
      $result.mark = mark;
    }
    return $result;
  }
  NewCalendarBookRequest._() : super();
  factory NewCalendarBookRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory NewCalendarBookRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'NewCalendarBookRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'manageId')
    ..aOS(2, _omitFieldNames ? '' : 'entityId')
    ..aOM<$0.Name>(3, _omitFieldNames ? '' : 'name', subBuilder: $0.Name.create)
    ..aOS(4, _omitFieldNames ? '' : 'description')
    ..aOS(5, _omitFieldNames ? '' : 'mark')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  NewCalendarBookRequest clone() => NewCalendarBookRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  NewCalendarBookRequest copyWith(void Function(NewCalendarBookRequest) updates) => super.copyWith((message) => updates(message as NewCalendarBookRequest)) as NewCalendarBookRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static NewCalendarBookRequest create() => NewCalendarBookRequest._();
  NewCalendarBookRequest createEmptyInstance() => create();
  static $pb.PbList<NewCalendarBookRequest> createRepeated() => $pb.PbList<NewCalendarBookRequest>();
  @$core.pragma('dart2js:noInline')
  static NewCalendarBookRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<NewCalendarBookRequest>(create);
  static NewCalendarBookRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get manageId => $_getSZ(0);
  @$pb.TagNumber(1)
  set manageId($core.String v) { $_setString(0, v); }
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

  @$pb.TagNumber(5)
  $core.String get mark => $_getSZ(4);
  @$pb.TagNumber(5)
  set mark($core.String v) { $_setString(4, v); }
  @$pb.TagNumber(5)
  $core.bool hasMark() => $_has(4);
  @$pb.TagNumber(5)
  void clearMark() => clearField(5);
}

class NewCalendarBookResponse extends $pb.GeneratedMessage {
  factory NewCalendarBookResponse({
    $core.String? result,
  }) {
    final $result = create();
    if (result != null) {
      $result.result = result;
    }
    return $result;
  }
  NewCalendarBookResponse._() : super();
  factory NewCalendarBookResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory NewCalendarBookResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'NewCalendarBookResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'result')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  NewCalendarBookResponse clone() => NewCalendarBookResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  NewCalendarBookResponse copyWith(void Function(NewCalendarBookResponse) updates) => super.copyWith((message) => updates(message as NewCalendarBookResponse)) as NewCalendarBookResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static NewCalendarBookResponse create() => NewCalendarBookResponse._();
  NewCalendarBookResponse createEmptyInstance() => create();
  static $pb.PbList<NewCalendarBookResponse> createRepeated() => $pb.PbList<NewCalendarBookResponse>();
  @$core.pragma('dart2js:noInline')
  static NewCalendarBookResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<NewCalendarBookResponse>(create);
  static NewCalendarBookResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get result => $_getSZ(0);
  @$pb.TagNumber(1)
  set result($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasResult() => $_has(0);
  @$pb.TagNumber(1)
  void clearResult() => clearField(1);
}

/// 列出所属帐本
class ListCalendarBooksRequest extends $pb.GeneratedMessage {
  factory ListCalendarBooksRequest({
    $core.String? manageId,
    $core.String? entityId,
  }) {
    final $result = create();
    if (manageId != null) {
      $result.manageId = manageId;
    }
    if (entityId != null) {
      $result.entityId = entityId;
    }
    return $result;
  }
  ListCalendarBooksRequest._() : super();
  factory ListCalendarBooksRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ListCalendarBooksRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'ListCalendarBooksRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'manageId')
    ..aOS(2, _omitFieldNames ? '' : 'entityId')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ListCalendarBooksRequest clone() => ListCalendarBooksRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ListCalendarBooksRequest copyWith(void Function(ListCalendarBooksRequest) updates) => super.copyWith((message) => updates(message as ListCalendarBooksRequest)) as ListCalendarBooksRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ListCalendarBooksRequest create() => ListCalendarBooksRequest._();
  ListCalendarBooksRequest createEmptyInstance() => create();
  static $pb.PbList<ListCalendarBooksRequest> createRepeated() => $pb.PbList<ListCalendarBooksRequest>();
  @$core.pragma('dart2js:noInline')
  static ListCalendarBooksRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ListCalendarBooksRequest>(create);
  static ListCalendarBooksRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get manageId => $_getSZ(0);
  @$pb.TagNumber(1)
  set manageId($core.String v) { $_setString(0, v); }
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
}

class ListCalendarBooksResponse extends $pb.GeneratedMessage {
  factory ListCalendarBooksResponse({
    $core.Iterable<$core.List<$core.int>>? books,
  }) {
    final $result = create();
    if (books != null) {
      $result.books.addAll(books);
    }
    return $result;
  }
  ListCalendarBooksResponse._() : super();
  factory ListCalendarBooksResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ListCalendarBooksResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'ListCalendarBooksResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..p<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'books', $pb.PbFieldType.PY)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ListCalendarBooksResponse clone() => ListCalendarBooksResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ListCalendarBooksResponse copyWith(void Function(ListCalendarBooksResponse) updates) => super.copyWith((message) => updates(message as ListCalendarBooksResponse)) as ListCalendarBooksResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ListCalendarBooksResponse create() => ListCalendarBooksResponse._();
  ListCalendarBooksResponse createEmptyInstance() => create();
  static $pb.PbList<ListCalendarBooksResponse> createRepeated() => $pb.PbList<ListCalendarBooksResponse>();
  @$core.pragma('dart2js:noInline')
  static ListCalendarBooksResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ListCalendarBooksResponse>(create);
  static ListCalendarBooksResponse? _defaultInstance;

  /// bson documents
  @$pb.TagNumber(1)
  $core.List<$core.List<$core.int>> get books => $_getList(0);
}

/// 列出帐本日历
class ListBookCalendarsRequest extends $pb.GeneratedMessage {
  factory ListBookCalendarsRequest({
    $core.String? bookId,
  }) {
    final $result = create();
    if (bookId != null) {
      $result.bookId = bookId;
    }
    return $result;
  }
  ListBookCalendarsRequest._() : super();
  factory ListBookCalendarsRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ListBookCalendarsRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'ListBookCalendarsRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'bookId')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ListBookCalendarsRequest clone() => ListBookCalendarsRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ListBookCalendarsRequest copyWith(void Function(ListBookCalendarsRequest) updates) => super.copyWith((message) => updates(message as ListBookCalendarsRequest)) as ListBookCalendarsRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ListBookCalendarsRequest create() => ListBookCalendarsRequest._();
  ListBookCalendarsRequest createEmptyInstance() => create();
  static $pb.PbList<ListBookCalendarsRequest> createRepeated() => $pb.PbList<ListBookCalendarsRequest>();
  @$core.pragma('dart2js:noInline')
  static ListBookCalendarsRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ListBookCalendarsRequest>(create);
  static ListBookCalendarsRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get bookId => $_getSZ(0);
  @$pb.TagNumber(1)
  set bookId($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasBookId() => $_has(0);
  @$pb.TagNumber(1)
  void clearBookId() => clearField(1);
}

class ListBookCalendarsResponse extends $pb.GeneratedMessage {
  factory ListBookCalendarsResponse({
    $core.Iterable<$core.List<$core.int>>? calendars,
  }) {
    final $result = create();
    if (calendars != null) {
      $result.calendars.addAll(calendars);
    }
    return $result;
  }
  ListBookCalendarsResponse._() : super();
  factory ListBookCalendarsResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ListBookCalendarsResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'ListBookCalendarsResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..p<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'calendars', $pb.PbFieldType.PY)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ListBookCalendarsResponse clone() => ListBookCalendarsResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ListBookCalendarsResponse copyWith(void Function(ListBookCalendarsResponse) updates) => super.copyWith((message) => updates(message as ListBookCalendarsResponse)) as ListBookCalendarsResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ListBookCalendarsResponse create() => ListBookCalendarsResponse._();
  ListBookCalendarsResponse createEmptyInstance() => create();
  static $pb.PbList<ListBookCalendarsResponse> createRepeated() => $pb.PbList<ListBookCalendarsResponse>();
  @$core.pragma('dart2js:noInline')
  static ListBookCalendarsResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ListBookCalendarsResponse>(create);
  static ListBookCalendarsResponse? _defaultInstance;

  /// bson documents
  @$pb.TagNumber(1)
  $core.List<$core.List<$core.int>> get calendars => $_getList(0);
}


const _omitFieldNames = $core.bool.fromEnvironment('protobuf.omit_field_names');
const _omitMessageNames = $core.bool.fromEnvironment('protobuf.omit_message_names');
