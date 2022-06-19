///
//  Generated code. Do not modify.
//  source: set_data.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

class SetDataInfo extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'SetDataInfo', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..pPS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'files')
    ..a<$fixnum.Int64>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'totalSize', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..aOS(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'md5')
    ..hasRequiredFields = false
  ;

  SetDataInfo._() : super();
  factory SetDataInfo({
    $core.Iterable<$core.String>? files,
    $fixnum.Int64? totalSize,
    $core.String? md5,
  }) {
    final _result = create();
    if (files != null) {
      _result.files.addAll(files);
    }
    if (totalSize != null) {
      _result.totalSize = totalSize;
    }
    if (md5 != null) {
      _result.md5 = md5;
    }
    return _result;
  }
  factory SetDataInfo.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory SetDataInfo.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  SetDataInfo clone() => SetDataInfo()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  SetDataInfo copyWith(void Function(SetDataInfo) updates) => super.copyWith((message) => updates(message as SetDataInfo)) as SetDataInfo; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static SetDataInfo create() => SetDataInfo._();
  SetDataInfo createEmptyInstance() => create();
  static $pb.PbList<SetDataInfo> createRepeated() => $pb.PbList<SetDataInfo>();
  @$core.pragma('dart2js:noInline')
  static SetDataInfo getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<SetDataInfo>(create);
  static SetDataInfo? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.String> get files => $_getList(0);

  @$pb.TagNumber(2)
  $fixnum.Int64 get totalSize => $_getI64(1);
  @$pb.TagNumber(2)
  set totalSize($fixnum.Int64 v) { $_setInt64(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasTotalSize() => $_has(1);
  @$pb.TagNumber(2)
  void clearTotalSize() => clearField(2);

  @$pb.TagNumber(3)
  $core.String get md5 => $_getSZ(2);
  @$pb.TagNumber(3)
  set md5($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasMd5() => $_has(2);
  @$pb.TagNumber(3)
  void clearMd5() => clearField(3);
}

class SetDataUploadSetRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'SetDataUploadSetRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'dataId')
    ..aOS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'setName')
    ..a<$core.int>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'fileCounts', $pb.PbFieldType.OU3)
    ..aOS(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'currentFile')
    ..a<$fixnum.Int64>(5, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'totalChancks', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$core.int>(6, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'currentChunck', $pb.PbFieldType.OU3)
    ..a<$core.List<$core.int>>(7, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'chunck', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  SetDataUploadSetRequest._() : super();
  factory SetDataUploadSetRequest({
    $core.String? dataId,
    $core.String? setName,
    $core.int? fileCounts,
    $core.String? currentFile,
    $fixnum.Int64? totalChancks,
    $core.int? currentChunck,
    $core.List<$core.int>? chunck,
  }) {
    final _result = create();
    if (dataId != null) {
      _result.dataId = dataId;
    }
    if (setName != null) {
      _result.setName = setName;
    }
    if (fileCounts != null) {
      _result.fileCounts = fileCounts;
    }
    if (currentFile != null) {
      _result.currentFile = currentFile;
    }
    if (totalChancks != null) {
      _result.totalChancks = totalChancks;
    }
    if (currentChunck != null) {
      _result.currentChunck = currentChunck;
    }
    if (chunck != null) {
      _result.chunck = chunck;
    }
    return _result;
  }
  factory SetDataUploadSetRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory SetDataUploadSetRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  SetDataUploadSetRequest clone() => SetDataUploadSetRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  SetDataUploadSetRequest copyWith(void Function(SetDataUploadSetRequest) updates) => super.copyWith((message) => updates(message as SetDataUploadSetRequest)) as SetDataUploadSetRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static SetDataUploadSetRequest create() => SetDataUploadSetRequest._();
  SetDataUploadSetRequest createEmptyInstance() => create();
  static $pb.PbList<SetDataUploadSetRequest> createRepeated() => $pb.PbList<SetDataUploadSetRequest>();
  @$core.pragma('dart2js:noInline')
  static SetDataUploadSetRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<SetDataUploadSetRequest>(create);
  static SetDataUploadSetRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get dataId => $_getSZ(0);
  @$pb.TagNumber(1)
  set dataId($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasDataId() => $_has(0);
  @$pb.TagNumber(1)
  void clearDataId() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get setName => $_getSZ(1);
  @$pb.TagNumber(2)
  set setName($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasSetName() => $_has(1);
  @$pb.TagNumber(2)
  void clearSetName() => clearField(2);

  @$pb.TagNumber(3)
  $core.int get fileCounts => $_getIZ(2);
  @$pb.TagNumber(3)
  set fileCounts($core.int v) { $_setUnsignedInt32(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasFileCounts() => $_has(2);
  @$pb.TagNumber(3)
  void clearFileCounts() => clearField(3);

  @$pb.TagNumber(4)
  $core.String get currentFile => $_getSZ(3);
  @$pb.TagNumber(4)
  set currentFile($core.String v) { $_setString(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasCurrentFile() => $_has(3);
  @$pb.TagNumber(4)
  void clearCurrentFile() => clearField(4);

  @$pb.TagNumber(5)
  $fixnum.Int64 get totalChancks => $_getI64(4);
  @$pb.TagNumber(5)
  set totalChancks($fixnum.Int64 v) { $_setInt64(4, v); }
  @$pb.TagNumber(5)
  $core.bool hasTotalChancks() => $_has(4);
  @$pb.TagNumber(5)
  void clearTotalChancks() => clearField(5);

  @$pb.TagNumber(6)
  $core.int get currentChunck => $_getIZ(5);
  @$pb.TagNumber(6)
  set currentChunck($core.int v) { $_setUnsignedInt32(5, v); }
  @$pb.TagNumber(6)
  $core.bool hasCurrentChunck() => $_has(5);
  @$pb.TagNumber(6)
  void clearCurrentChunck() => clearField(6);

  @$pb.TagNumber(7)
  $core.List<$core.int> get chunck => $_getN(6);
  @$pb.TagNumber(7)
  set chunck($core.List<$core.int> v) { $_setBytes(6, v); }
  @$pb.TagNumber(7)
  $core.bool hasChunck() => $_has(6);
  @$pb.TagNumber(7)
  void clearChunck() => clearField(7);
}

class SetDataUploadSetResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'SetDataUploadSetResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'result')
    ..hasRequiredFields = false
  ;

  SetDataUploadSetResponse._() : super();
  factory SetDataUploadSetResponse({
    $core.String? result,
  }) {
    final _result = create();
    if (result != null) {
      _result.result = result;
    }
    return _result;
  }
  factory SetDataUploadSetResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory SetDataUploadSetResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  SetDataUploadSetResponse clone() => SetDataUploadSetResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  SetDataUploadSetResponse copyWith(void Function(SetDataUploadSetResponse) updates) => super.copyWith((message) => updates(message as SetDataUploadSetResponse)) as SetDataUploadSetResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static SetDataUploadSetResponse create() => SetDataUploadSetResponse._();
  SetDataUploadSetResponse createEmptyInstance() => create();
  static $pb.PbList<SetDataUploadSetResponse> createRepeated() => $pb.PbList<SetDataUploadSetResponse>();
  @$core.pragma('dart2js:noInline')
  static SetDataUploadSetResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<SetDataUploadSetResponse>(create);
  static SetDataUploadSetResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get result => $_getSZ(0);
  @$pb.TagNumber(1)
  set result($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasResult() => $_has(0);
  @$pb.TagNumber(1)
  void clearResult() => clearField(1);
}

class SetDataDownloadSetRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'SetDataDownloadSetRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'dataId')
    ..hasRequiredFields = false
  ;

  SetDataDownloadSetRequest._() : super();
  factory SetDataDownloadSetRequest({
    $core.String? dataId,
  }) {
    final _result = create();
    if (dataId != null) {
      _result.dataId = dataId;
    }
    return _result;
  }
  factory SetDataDownloadSetRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory SetDataDownloadSetRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  SetDataDownloadSetRequest clone() => SetDataDownloadSetRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  SetDataDownloadSetRequest copyWith(void Function(SetDataDownloadSetRequest) updates) => super.copyWith((message) => updates(message as SetDataDownloadSetRequest)) as SetDataDownloadSetRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static SetDataDownloadSetRequest create() => SetDataDownloadSetRequest._();
  SetDataDownloadSetRequest createEmptyInstance() => create();
  static $pb.PbList<SetDataDownloadSetRequest> createRepeated() => $pb.PbList<SetDataDownloadSetRequest>();
  @$core.pragma('dart2js:noInline')
  static SetDataDownloadSetRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<SetDataDownloadSetRequest>(create);
  static SetDataDownloadSetRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get dataId => $_getSZ(0);
  @$pb.TagNumber(1)
  set dataId($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasDataId() => $_has(0);
  @$pb.TagNumber(1)
  void clearDataId() => clearField(1);
}

class SetDataDownloadSetResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'SetDataDownloadSetResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'dataId')
    ..aOS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'setName')
    ..a<$core.int>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'fileCounts', $pb.PbFieldType.OU3)
    ..aOS(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'currentFile')
    ..a<$fixnum.Int64>(5, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'totalChancks', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$core.int>(6, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'currentChunck', $pb.PbFieldType.OU3)
    ..a<$core.List<$core.int>>(7, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'chunck', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  SetDataDownloadSetResponse._() : super();
  factory SetDataDownloadSetResponse({
    $core.String? dataId,
    $core.String? setName,
    $core.int? fileCounts,
    $core.String? currentFile,
    $fixnum.Int64? totalChancks,
    $core.int? currentChunck,
    $core.List<$core.int>? chunck,
  }) {
    final _result = create();
    if (dataId != null) {
      _result.dataId = dataId;
    }
    if (setName != null) {
      _result.setName = setName;
    }
    if (fileCounts != null) {
      _result.fileCounts = fileCounts;
    }
    if (currentFile != null) {
      _result.currentFile = currentFile;
    }
    if (totalChancks != null) {
      _result.totalChancks = totalChancks;
    }
    if (currentChunck != null) {
      _result.currentChunck = currentChunck;
    }
    if (chunck != null) {
      _result.chunck = chunck;
    }
    return _result;
  }
  factory SetDataDownloadSetResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory SetDataDownloadSetResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  SetDataDownloadSetResponse clone() => SetDataDownloadSetResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  SetDataDownloadSetResponse copyWith(void Function(SetDataDownloadSetResponse) updates) => super.copyWith((message) => updates(message as SetDataDownloadSetResponse)) as SetDataDownloadSetResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static SetDataDownloadSetResponse create() => SetDataDownloadSetResponse._();
  SetDataDownloadSetResponse createEmptyInstance() => create();
  static $pb.PbList<SetDataDownloadSetResponse> createRepeated() => $pb.PbList<SetDataDownloadSetResponse>();
  @$core.pragma('dart2js:noInline')
  static SetDataDownloadSetResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<SetDataDownloadSetResponse>(create);
  static SetDataDownloadSetResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get dataId => $_getSZ(0);
  @$pb.TagNumber(1)
  set dataId($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasDataId() => $_has(0);
  @$pb.TagNumber(1)
  void clearDataId() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get setName => $_getSZ(1);
  @$pb.TagNumber(2)
  set setName($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasSetName() => $_has(1);
  @$pb.TagNumber(2)
  void clearSetName() => clearField(2);

  @$pb.TagNumber(3)
  $core.int get fileCounts => $_getIZ(2);
  @$pb.TagNumber(3)
  set fileCounts($core.int v) { $_setUnsignedInt32(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasFileCounts() => $_has(2);
  @$pb.TagNumber(3)
  void clearFileCounts() => clearField(3);

  @$pb.TagNumber(4)
  $core.String get currentFile => $_getSZ(3);
  @$pb.TagNumber(4)
  set currentFile($core.String v) { $_setString(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasCurrentFile() => $_has(3);
  @$pb.TagNumber(4)
  void clearCurrentFile() => clearField(4);

  @$pb.TagNumber(5)
  $fixnum.Int64 get totalChancks => $_getI64(4);
  @$pb.TagNumber(5)
  set totalChancks($fixnum.Int64 v) { $_setInt64(4, v); }
  @$pb.TagNumber(5)
  $core.bool hasTotalChancks() => $_has(4);
  @$pb.TagNumber(5)
  void clearTotalChancks() => clearField(5);

  @$pb.TagNumber(6)
  $core.int get currentChunck => $_getIZ(5);
  @$pb.TagNumber(6)
  set currentChunck($core.int v) { $_setUnsignedInt32(5, v); }
  @$pb.TagNumber(6)
  $core.bool hasCurrentChunck() => $_has(5);
  @$pb.TagNumber(6)
  void clearCurrentChunck() => clearField(6);

  @$pb.TagNumber(7)
  $core.List<$core.int> get chunck => $_getN(6);
  @$pb.TagNumber(7)
  set chunck($core.List<$core.int> v) { $_setBytes(6, v); }
  @$pb.TagNumber(7)
  $core.bool hasChunck() => $_has(6);
  @$pb.TagNumber(7)
  void clearChunck() => clearField(7);
}

