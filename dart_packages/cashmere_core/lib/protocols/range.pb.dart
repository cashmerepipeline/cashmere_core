//
//  Generated code. Do not modify.
//  source: range.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

class Range extends $pb.GeneratedMessage {
  factory Range({
    $core.double? min,
    $core.double? max,
  }) {
    final $result = create();
    if (min != null) {
      $result.min = min;
    }
    if (max != null) {
      $result.max = max;
    }
    return $result;
  }
  Range._() : super();
  factory Range.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Range.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'Range', package: const $pb.PackageName(_omitMessageNames ? '' : 'cashmere'), createEmptyInstance: create)
    ..a<$core.double>(1, _omitFieldNames ? '' : 'min', $pb.PbFieldType.OF)
    ..a<$core.double>(2, _omitFieldNames ? '' : 'max', $pb.PbFieldType.OF)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Range clone() => Range()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Range copyWith(void Function(Range) updates) => super.copyWith((message) => updates(message as Range)) as Range;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Range create() => Range._();
  Range createEmptyInstance() => create();
  static $pb.PbList<Range> createRepeated() => $pb.PbList<Range>();
  @$core.pragma('dart2js:noInline')
  static Range getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Range>(create);
  static Range? _defaultInstance;

  @$pb.TagNumber(1)
  $core.double get min => $_getN(0);
  @$pb.TagNumber(1)
  set min($core.double v) { $_setFloat(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasMin() => $_has(0);
  @$pb.TagNumber(1)
  void clearMin() => clearField(1);

  @$pb.TagNumber(2)
  $core.double get max => $_getN(1);
  @$pb.TagNumber(2)
  set max($core.double v) { $_setFloat(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasMax() => $_has(1);
  @$pb.TagNumber(2)
  void clearMax() => clearField(2);
}


const _omitFieldNames = $core.bool.fromEnvironment('protobuf.omit_field_names');
const _omitMessageNames = $core.bool.fromEnvironment('protobuf.omit_message_names');
