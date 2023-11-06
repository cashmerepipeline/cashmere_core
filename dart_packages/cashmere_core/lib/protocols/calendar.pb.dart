//
//  Generated code. Do not modify.
//  source: calendar.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

import 'calendar.pbenum.dart';
import 'name.pb.dart' as $0;

export 'calendar.pbenum.dart';

class Calendar extends $pb.GeneratedMessage {
  factory Calendar({
    CalendarType? type,
    $core.String? every,
    $core.Map<$core.String, $core.String>? daytime,
  }) {
    final $result = create();
    if (type != null) {
      $result.type = type;
    }
    if (every != null) {
      $result.every = every;
    }
    if (daytime != null) {
      $result.daytime.addAll(daytime);
    }
    return $result;
  }
  Calendar._() : super();
  factory Calendar.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Calendar.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'Calendar', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..e<CalendarType>(1, _omitFieldNames ? '' : 'type', $pb.PbFieldType.OE, defaultOrMaker: CalendarType.Specified, valueOf: CalendarType.valueOf, enumValues: CalendarType.values)
    ..aOS(2, _omitFieldNames ? '' : 'every')
    ..m<$core.String, $core.String>(3, _omitFieldNames ? '' : 'daytime', entryClassName: 'Calendar.DaytimeEntry', keyFieldType: $pb.PbFieldType.OS, valueFieldType: $pb.PbFieldType.OS, packageName: const $pb.PackageName('cashmere'))
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Calendar clone() => Calendar()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Calendar copyWith(void Function(Calendar) updates) => super.copyWith((message) => updates(message as Calendar)) as Calendar;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Calendar create() => Calendar._();
  Calendar createEmptyInstance() => create();
  static $pb.PbList<Calendar> createRepeated() => $pb.PbList<Calendar>();
  @$core.pragma('dart2js:noInline')
  static Calendar getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Calendar>(create);
  static Calendar? _defaultInstance;

  @$pb.TagNumber(1)
  CalendarType get type => $_getN(0);
  @$pb.TagNumber(1)
  set type(CalendarType v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasType() => $_has(0);
  @$pb.TagNumber(1)
  void clearType() => clearField(1);

  /// {"year"| "month"| "week"}
  @$pb.TagNumber(2)
  $core.String get every => $_getSZ(1);
  @$pb.TagNumber(2)
  set every($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasEvery() => $_has(1);
  @$pb.TagNumber(2)
  void clearEvery() => clearField(2);

  /// {"day": day, "hour": hour, "minute": minute}
  @$pb.TagNumber(3)
  $core.Map<$core.String, $core.String> get daytime => $_getMap(2);
}

/// 新日历
class NewCalendarRequest extends $pb.GeneratedMessage {
  factory NewCalendarRequest({
    $core.String? bookId,
    Calendar? calendar,
    $core.String? description,
    $0.Name? name,
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
    if (name != null) {
      $result.name = name;
    }
    return $result;
  }
  NewCalendarRequest._() : super();
  factory NewCalendarRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory NewCalendarRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'NewCalendarRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'bookId')
    ..aOM<Calendar>(2, _omitFieldNames ? '' : 'calendar', subBuilder: Calendar.create)
    ..aOS(3, _omitFieldNames ? '' : 'description')
    ..aOM<$0.Name>(4, _omitFieldNames ? '' : 'name', subBuilder: $0.Name.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  NewCalendarRequest clone() => NewCalendarRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  NewCalendarRequest copyWith(void Function(NewCalendarRequest) updates) => super.copyWith((message) => updates(message as NewCalendarRequest)) as NewCalendarRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static NewCalendarRequest create() => NewCalendarRequest._();
  NewCalendarRequest createEmptyInstance() => create();
  static $pb.PbList<NewCalendarRequest> createRepeated() => $pb.PbList<NewCalendarRequest>();
  @$core.pragma('dart2js:noInline')
  static NewCalendarRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<NewCalendarRequest>(create);
  static NewCalendarRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get bookId => $_getSZ(0);
  @$pb.TagNumber(1)
  set bookId($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasBookId() => $_has(0);
  @$pb.TagNumber(1)
  void clearBookId() => clearField(1);

  @$pb.TagNumber(2)
  Calendar get calendar => $_getN(1);
  @$pb.TagNumber(2)
  set calendar(Calendar v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasCalendar() => $_has(1);
  @$pb.TagNumber(2)
  void clearCalendar() => clearField(2);
  @$pb.TagNumber(2)
  Calendar ensureCalendar() => $_ensure(1);

  @$pb.TagNumber(3)
  $core.String get description => $_getSZ(2);
  @$pb.TagNumber(3)
  set description($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasDescription() => $_has(2);
  @$pb.TagNumber(3)
  void clearDescription() => clearField(3);

  @$pb.TagNumber(4)
  $0.Name get name => $_getN(3);
  @$pb.TagNumber(4)
  set name($0.Name v) { setField(4, v); }
  @$pb.TagNumber(4)
  $core.bool hasName() => $_has(3);
  @$pb.TagNumber(4)
  void clearName() => clearField(4);
  @$pb.TagNumber(4)
  $0.Name ensureName() => $_ensure(3);
}

class NewCalendarResponse extends $pb.GeneratedMessage {
  factory NewCalendarResponse({
    $core.String? result,
  }) {
    final $result = create();
    if (result != null) {
      $result.result = result;
    }
    return $result;
  }
  NewCalendarResponse._() : super();
  factory NewCalendarResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory NewCalendarResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'NewCalendarResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'result')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  NewCalendarResponse clone() => NewCalendarResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  NewCalendarResponse copyWith(void Function(NewCalendarResponse) updates) => super.copyWith((message) => updates(message as NewCalendarResponse)) as NewCalendarResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static NewCalendarResponse create() => NewCalendarResponse._();
  NewCalendarResponse createEmptyInstance() => create();
  static $pb.PbList<NewCalendarResponse> createRepeated() => $pb.PbList<NewCalendarResponse>();
  @$core.pragma('dart2js:noInline')
  static NewCalendarResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<NewCalendarResponse>(create);
  static NewCalendarResponse? _defaultInstance;

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
