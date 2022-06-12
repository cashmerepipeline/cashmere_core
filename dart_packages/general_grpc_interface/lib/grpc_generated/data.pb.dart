///
//  Generated code. Do not modify.
//  source: data.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

import 'name.pb.dart' as $0;

import 'data.pbenum.dart';

export 'data.pbenum.dart';

class NewDataRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'NewDataRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOM<$0.Name>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'name', subBuilder: $0.Name.create)
    ..e<DataType>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'dataType', $pb.PbFieldType.OE, defaultOrMaker: DataType.File, valueOf: DataType.valueOf, enumValues: DataType.values)
    ..a<$core.int>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'manageId', $pb.PbFieldType.O3)
    ..aOS(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'entityId')
    ..hasRequiredFields = false
  ;

  NewDataRequest._() : super();
  factory NewDataRequest({
    $0.Name? name,
    DataType? dataType,
    $core.int? manageId,
    $core.String? entityId,
  }) {
    final _result = create();
    if (name != null) {
      _result.name = name;
    }
    if (dataType != null) {
      _result.dataType = dataType;
    }
    if (manageId != null) {
      _result.manageId = manageId;
    }
    if (entityId != null) {
      _result.entityId = entityId;
    }
    return _result;
  }
  factory NewDataRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory NewDataRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  NewDataRequest clone() => NewDataRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  NewDataRequest copyWith(void Function(NewDataRequest) updates) => super.copyWith((message) => updates(message as NewDataRequest)) as NewDataRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static NewDataRequest create() => NewDataRequest._();
  NewDataRequest createEmptyInstance() => create();
  static $pb.PbList<NewDataRequest> createRepeated() => $pb.PbList<NewDataRequest>();
  @$core.pragma('dart2js:noInline')
  static NewDataRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<NewDataRequest>(create);
  static NewDataRequest? _defaultInstance;

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
  DataType get dataType => $_getN(1);
  @$pb.TagNumber(2)
  set dataType(DataType v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasDataType() => $_has(1);
  @$pb.TagNumber(2)
  void clearDataType() => clearField(2);

  @$pb.TagNumber(3)
  $core.int get manageId => $_getIZ(2);
  @$pb.TagNumber(3)
  set manageId($core.int v) { $_setSignedInt32(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasManageId() => $_has(2);
  @$pb.TagNumber(3)
  void clearManageId() => clearField(3);

  @$pb.TagNumber(4)
  $core.String get entityId => $_getSZ(3);
  @$pb.TagNumber(4)
  set entityId($core.String v) { $_setString(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasEntityId() => $_has(3);
  @$pb.TagNumber(4)
  void clearEntityId() => clearField(4);
}

class NewDataResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'NewDataResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'result')
    ..hasRequiredFields = false
  ;

  NewDataResponse._() : super();
  factory NewDataResponse({
    $core.String? result,
  }) {
    final _result = create();
    if (result != null) {
      _result.result = result;
    }
    return _result;
  }
  factory NewDataResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory NewDataResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  NewDataResponse clone() => NewDataResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  NewDataResponse copyWith(void Function(NewDataResponse) updates) => super.copyWith((message) => updates(message as NewDataResponse)) as NewDataResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static NewDataResponse create() => NewDataResponse._();
  NewDataResponse createEmptyInstance() => create();
  static $pb.PbList<NewDataResponse> createRepeated() => $pb.PbList<NewDataResponse>();
  @$core.pragma('dart2js:noInline')
  static NewDataResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<NewDataResponse>(create);
  static NewDataResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get result => $_getSZ(0);
  @$pb.TagNumber(1)
  set result($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasResult() => $_has(0);
  @$pb.TagNumber(1)
  void clearResult() => clearField(1);
}

class DataUploadFileRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'DataUploadFileRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'dataId')
    ..a<$fixnum.Int64>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'totalChuncks', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'currentChunck', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$core.List<$core.int>>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'chunck', $pb.PbFieldType.OY)
    ..aOS(5, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'fileName')
    ..aOM<$0.Name>(6, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'name', subBuilder: $0.Name.create)
    ..hasRequiredFields = false
  ;

  DataUploadFileRequest._() : super();
  factory DataUploadFileRequest({
    $core.String? dataId,
    $fixnum.Int64? totalChuncks,
    $fixnum.Int64? currentChunck,
    $core.List<$core.int>? chunck,
    $core.String? fileName,
    $0.Name? name,
  }) {
    final _result = create();
    if (dataId != null) {
      _result.dataId = dataId;
    }
    if (totalChuncks != null) {
      _result.totalChuncks = totalChuncks;
    }
    if (currentChunck != null) {
      _result.currentChunck = currentChunck;
    }
    if (chunck != null) {
      _result.chunck = chunck;
    }
    if (fileName != null) {
      _result.fileName = fileName;
    }
    if (name != null) {
      _result.name = name;
    }
    return _result;
  }
  factory DataUploadFileRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory DataUploadFileRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  DataUploadFileRequest clone() => DataUploadFileRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  DataUploadFileRequest copyWith(void Function(DataUploadFileRequest) updates) => super.copyWith((message) => updates(message as DataUploadFileRequest)) as DataUploadFileRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static DataUploadFileRequest create() => DataUploadFileRequest._();
  DataUploadFileRequest createEmptyInstance() => create();
  static $pb.PbList<DataUploadFileRequest> createRepeated() => $pb.PbList<DataUploadFileRequest>();
  @$core.pragma('dart2js:noInline')
  static DataUploadFileRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<DataUploadFileRequest>(create);
  static DataUploadFileRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get dataId => $_getSZ(0);
  @$pb.TagNumber(1)
  set dataId($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasDataId() => $_has(0);
  @$pb.TagNumber(1)
  void clearDataId() => clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get totalChuncks => $_getI64(1);
  @$pb.TagNumber(2)
  set totalChuncks($fixnum.Int64 v) { $_setInt64(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasTotalChuncks() => $_has(1);
  @$pb.TagNumber(2)
  void clearTotalChuncks() => clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get currentChunck => $_getI64(2);
  @$pb.TagNumber(3)
  set currentChunck($fixnum.Int64 v) { $_setInt64(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasCurrentChunck() => $_has(2);
  @$pb.TagNumber(3)
  void clearCurrentChunck() => clearField(3);

  @$pb.TagNumber(4)
  $core.List<$core.int> get chunck => $_getN(3);
  @$pb.TagNumber(4)
  set chunck($core.List<$core.int> v) { $_setBytes(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasChunck() => $_has(3);
  @$pb.TagNumber(4)
  void clearChunck() => clearField(4);

  @$pb.TagNumber(5)
  $core.String get fileName => $_getSZ(4);
  @$pb.TagNumber(5)
  set fileName($core.String v) { $_setString(4, v); }
  @$pb.TagNumber(5)
  $core.bool hasFileName() => $_has(4);
  @$pb.TagNumber(5)
  void clearFileName() => clearField(5);

  @$pb.TagNumber(6)
  $0.Name get name => $_getN(5);
  @$pb.TagNumber(6)
  set name($0.Name v) { setField(6, v); }
  @$pb.TagNumber(6)
  $core.bool hasName() => $_has(5);
  @$pb.TagNumber(6)
  void clearName() => clearField(6);
  @$pb.TagNumber(6)
  $0.Name ensureName() => $_ensure(5);
}

class DataUploadFileResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'DataUploadFileResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'result')
    ..hasRequiredFields = false
  ;

  DataUploadFileResponse._() : super();
  factory DataUploadFileResponse({
    $core.String? result,
  }) {
    final _result = create();
    if (result != null) {
      _result.result = result;
    }
    return _result;
  }
  factory DataUploadFileResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory DataUploadFileResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  DataUploadFileResponse clone() => DataUploadFileResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  DataUploadFileResponse copyWith(void Function(DataUploadFileResponse) updates) => super.copyWith((message) => updates(message as DataUploadFileResponse)) as DataUploadFileResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static DataUploadFileResponse create() => DataUploadFileResponse._();
  DataUploadFileResponse createEmptyInstance() => create();
  static $pb.PbList<DataUploadFileResponse> createRepeated() => $pb.PbList<DataUploadFileResponse>();
  @$core.pragma('dart2js:noInline')
  static DataUploadFileResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<DataUploadFileResponse>(create);
  static DataUploadFileResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get result => $_getSZ(0);
  @$pb.TagNumber(1)
  set result($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasResult() => $_has(0);
  @$pb.TagNumber(1)
  void clearResult() => clearField(1);
}

class DataDownloadFileRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'DataDownloadFileRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'dataId')
    ..hasRequiredFields = false
  ;

  DataDownloadFileRequest._() : super();
  factory DataDownloadFileRequest({
    $core.String? dataId,
  }) {
    final _result = create();
    if (dataId != null) {
      _result.dataId = dataId;
    }
    return _result;
  }
  factory DataDownloadFileRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory DataDownloadFileRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  DataDownloadFileRequest clone() => DataDownloadFileRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  DataDownloadFileRequest copyWith(void Function(DataDownloadFileRequest) updates) => super.copyWith((message) => updates(message as DataDownloadFileRequest)) as DataDownloadFileRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static DataDownloadFileRequest create() => DataDownloadFileRequest._();
  DataDownloadFileRequest createEmptyInstance() => create();
  static $pb.PbList<DataDownloadFileRequest> createRepeated() => $pb.PbList<DataDownloadFileRequest>();
  @$core.pragma('dart2js:noInline')
  static DataDownloadFileRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<DataDownloadFileRequest>(create);
  static DataDownloadFileRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get dataId => $_getSZ(0);
  @$pb.TagNumber(1)
  set dataId($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasDataId() => $_has(0);
  @$pb.TagNumber(1)
  void clearDataId() => clearField(1);
}

class DataDownloadFileResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'DataDownloadFileResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'dataId')
    ..a<$fixnum.Int64>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'totalChuncks', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'currentChunck', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$core.List<$core.int>>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'chunck', $pb.PbFieldType.OY)
    ..aOS(5, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'fileName')
    ..hasRequiredFields = false
  ;

  DataDownloadFileResponse._() : super();
  factory DataDownloadFileResponse({
    $core.String? dataId,
    $fixnum.Int64? totalChuncks,
    $fixnum.Int64? currentChunck,
    $core.List<$core.int>? chunck,
    $core.String? fileName,
  }) {
    final _result = create();
    if (dataId != null) {
      _result.dataId = dataId;
    }
    if (totalChuncks != null) {
      _result.totalChuncks = totalChuncks;
    }
    if (currentChunck != null) {
      _result.currentChunck = currentChunck;
    }
    if (chunck != null) {
      _result.chunck = chunck;
    }
    if (fileName != null) {
      _result.fileName = fileName;
    }
    return _result;
  }
  factory DataDownloadFileResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory DataDownloadFileResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  DataDownloadFileResponse clone() => DataDownloadFileResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  DataDownloadFileResponse copyWith(void Function(DataDownloadFileResponse) updates) => super.copyWith((message) => updates(message as DataDownloadFileResponse)) as DataDownloadFileResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static DataDownloadFileResponse create() => DataDownloadFileResponse._();
  DataDownloadFileResponse createEmptyInstance() => create();
  static $pb.PbList<DataDownloadFileResponse> createRepeated() => $pb.PbList<DataDownloadFileResponse>();
  @$core.pragma('dart2js:noInline')
  static DataDownloadFileResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<DataDownloadFileResponse>(create);
  static DataDownloadFileResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get dataId => $_getSZ(0);
  @$pb.TagNumber(1)
  set dataId($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasDataId() => $_has(0);
  @$pb.TagNumber(1)
  void clearDataId() => clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get totalChuncks => $_getI64(1);
  @$pb.TagNumber(2)
  set totalChuncks($fixnum.Int64 v) { $_setInt64(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasTotalChuncks() => $_has(1);
  @$pb.TagNumber(2)
  void clearTotalChuncks() => clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get currentChunck => $_getI64(2);
  @$pb.TagNumber(3)
  set currentChunck($fixnum.Int64 v) { $_setInt64(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasCurrentChunck() => $_has(2);
  @$pb.TagNumber(3)
  void clearCurrentChunck() => clearField(3);

  @$pb.TagNumber(4)
  $core.List<$core.int> get chunck => $_getN(3);
  @$pb.TagNumber(4)
  set chunck($core.List<$core.int> v) { $_setBytes(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasChunck() => $_has(3);
  @$pb.TagNumber(4)
  void clearChunck() => clearField(4);

  @$pb.TagNumber(5)
  $core.String get fileName => $_getSZ(4);
  @$pb.TagNumber(5)
  set fileName($core.String v) { $_setString(4, v); }
  @$pb.TagNumber(5)
  $core.bool hasFileName() => $_has(4);
  @$pb.TagNumber(5)
  void clearFileName() => clearField(5);
}

class DataUploadSequenceRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'DataUploadSequenceRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'sequenceName')
    ..aOS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'serialPattern')
    ..a<$core.int>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'sequenceLength', $pb.PbFieldType.OU3)
    ..aOS(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'currentFile')
    ..a<$fixnum.Int64>(5, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'totalChancks', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(6, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'currentChunck', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$core.List<$core.int>>(7, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'chunck', $pb.PbFieldType.OY)
    ..aOS(8, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'dataId')
    ..hasRequiredFields = false
  ;

  DataUploadSequenceRequest._() : super();
  factory DataUploadSequenceRequest({
    $core.String? sequenceName,
    $core.String? serialPattern,
    $core.int? sequenceLength,
    $core.String? currentFile,
    $fixnum.Int64? totalChancks,
    $fixnum.Int64? currentChunck,
    $core.List<$core.int>? chunck,
    $core.String? dataId,
  }) {
    final _result = create();
    if (sequenceName != null) {
      _result.sequenceName = sequenceName;
    }
    if (serialPattern != null) {
      _result.serialPattern = serialPattern;
    }
    if (sequenceLength != null) {
      _result.sequenceLength = sequenceLength;
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
    if (dataId != null) {
      _result.dataId = dataId;
    }
    return _result;
  }
  factory DataUploadSequenceRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory DataUploadSequenceRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  DataUploadSequenceRequest clone() => DataUploadSequenceRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  DataUploadSequenceRequest copyWith(void Function(DataUploadSequenceRequest) updates) => super.copyWith((message) => updates(message as DataUploadSequenceRequest)) as DataUploadSequenceRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static DataUploadSequenceRequest create() => DataUploadSequenceRequest._();
  DataUploadSequenceRequest createEmptyInstance() => create();
  static $pb.PbList<DataUploadSequenceRequest> createRepeated() => $pb.PbList<DataUploadSequenceRequest>();
  @$core.pragma('dart2js:noInline')
  static DataUploadSequenceRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<DataUploadSequenceRequest>(create);
  static DataUploadSequenceRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get sequenceName => $_getSZ(0);
  @$pb.TagNumber(1)
  set sequenceName($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasSequenceName() => $_has(0);
  @$pb.TagNumber(1)
  void clearSequenceName() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get serialPattern => $_getSZ(1);
  @$pb.TagNumber(2)
  set serialPattern($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasSerialPattern() => $_has(1);
  @$pb.TagNumber(2)
  void clearSerialPattern() => clearField(2);

  @$pb.TagNumber(3)
  $core.int get sequenceLength => $_getIZ(2);
  @$pb.TagNumber(3)
  set sequenceLength($core.int v) { $_setUnsignedInt32(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasSequenceLength() => $_has(2);
  @$pb.TagNumber(3)
  void clearSequenceLength() => clearField(3);

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
  $fixnum.Int64 get currentChunck => $_getI64(5);
  @$pb.TagNumber(6)
  set currentChunck($fixnum.Int64 v) { $_setInt64(5, v); }
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

  @$pb.TagNumber(8)
  $core.String get dataId => $_getSZ(7);
  @$pb.TagNumber(8)
  set dataId($core.String v) { $_setString(7, v); }
  @$pb.TagNumber(8)
  $core.bool hasDataId() => $_has(7);
  @$pb.TagNumber(8)
  void clearDataId() => clearField(8);
}

class DataUploadSequenceResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'DataUploadSequenceResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'result')
    ..hasRequiredFields = false
  ;

  DataUploadSequenceResponse._() : super();
  factory DataUploadSequenceResponse({
    $core.String? result,
  }) {
    final _result = create();
    if (result != null) {
      _result.result = result;
    }
    return _result;
  }
  factory DataUploadSequenceResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory DataUploadSequenceResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  DataUploadSequenceResponse clone() => DataUploadSequenceResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  DataUploadSequenceResponse copyWith(void Function(DataUploadSequenceResponse) updates) => super.copyWith((message) => updates(message as DataUploadSequenceResponse)) as DataUploadSequenceResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static DataUploadSequenceResponse create() => DataUploadSequenceResponse._();
  DataUploadSequenceResponse createEmptyInstance() => create();
  static $pb.PbList<DataUploadSequenceResponse> createRepeated() => $pb.PbList<DataUploadSequenceResponse>();
  @$core.pragma('dart2js:noInline')
  static DataUploadSequenceResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<DataUploadSequenceResponse>(create);
  static DataUploadSequenceResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get result => $_getSZ(0);
  @$pb.TagNumber(1)
  set result($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasResult() => $_has(0);
  @$pb.TagNumber(1)
  void clearResult() => clearField(1);
}

class DataDownloadSequenceRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'DataDownloadSequenceRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'dataId')
    ..hasRequiredFields = false
  ;

  DataDownloadSequenceRequest._() : super();
  factory DataDownloadSequenceRequest({
    $core.String? dataId,
  }) {
    final _result = create();
    if (dataId != null) {
      _result.dataId = dataId;
    }
    return _result;
  }
  factory DataDownloadSequenceRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory DataDownloadSequenceRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  DataDownloadSequenceRequest clone() => DataDownloadSequenceRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  DataDownloadSequenceRequest copyWith(void Function(DataDownloadSequenceRequest) updates) => super.copyWith((message) => updates(message as DataDownloadSequenceRequest)) as DataDownloadSequenceRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static DataDownloadSequenceRequest create() => DataDownloadSequenceRequest._();
  DataDownloadSequenceRequest createEmptyInstance() => create();
  static $pb.PbList<DataDownloadSequenceRequest> createRepeated() => $pb.PbList<DataDownloadSequenceRequest>();
  @$core.pragma('dart2js:noInline')
  static DataDownloadSequenceRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<DataDownloadSequenceRequest>(create);
  static DataDownloadSequenceRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get dataId => $_getSZ(0);
  @$pb.TagNumber(1)
  set dataId($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasDataId() => $_has(0);
  @$pb.TagNumber(1)
  void clearDataId() => clearField(1);
}

class DataDownloadSequenceResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'DataDownloadSequenceResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'sequenceName')
    ..aOS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'serialPattern')
    ..a<$core.int>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'sequenceLength', $pb.PbFieldType.OU3)
    ..aOS(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'currentFile')
    ..a<$fixnum.Int64>(5, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'totalChancks', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(6, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'currentChunck', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$core.List<$core.int>>(7, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'chunck', $pb.PbFieldType.OY)
    ..aOS(8, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'dataId')
    ..hasRequiredFields = false
  ;

  DataDownloadSequenceResponse._() : super();
  factory DataDownloadSequenceResponse({
    $core.String? sequenceName,
    $core.String? serialPattern,
    $core.int? sequenceLength,
    $core.String? currentFile,
    $fixnum.Int64? totalChancks,
    $fixnum.Int64? currentChunck,
    $core.List<$core.int>? chunck,
    $core.String? dataId,
  }) {
    final _result = create();
    if (sequenceName != null) {
      _result.sequenceName = sequenceName;
    }
    if (serialPattern != null) {
      _result.serialPattern = serialPattern;
    }
    if (sequenceLength != null) {
      _result.sequenceLength = sequenceLength;
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
    if (dataId != null) {
      _result.dataId = dataId;
    }
    return _result;
  }
  factory DataDownloadSequenceResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory DataDownloadSequenceResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  DataDownloadSequenceResponse clone() => DataDownloadSequenceResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  DataDownloadSequenceResponse copyWith(void Function(DataDownloadSequenceResponse) updates) => super.copyWith((message) => updates(message as DataDownloadSequenceResponse)) as DataDownloadSequenceResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static DataDownloadSequenceResponse create() => DataDownloadSequenceResponse._();
  DataDownloadSequenceResponse createEmptyInstance() => create();
  static $pb.PbList<DataDownloadSequenceResponse> createRepeated() => $pb.PbList<DataDownloadSequenceResponse>();
  @$core.pragma('dart2js:noInline')
  static DataDownloadSequenceResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<DataDownloadSequenceResponse>(create);
  static DataDownloadSequenceResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get sequenceName => $_getSZ(0);
  @$pb.TagNumber(1)
  set sequenceName($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasSequenceName() => $_has(0);
  @$pb.TagNumber(1)
  void clearSequenceName() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get serialPattern => $_getSZ(1);
  @$pb.TagNumber(2)
  set serialPattern($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasSerialPattern() => $_has(1);
  @$pb.TagNumber(2)
  void clearSerialPattern() => clearField(2);

  @$pb.TagNumber(3)
  $core.int get sequenceLength => $_getIZ(2);
  @$pb.TagNumber(3)
  set sequenceLength($core.int v) { $_setUnsignedInt32(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasSequenceLength() => $_has(2);
  @$pb.TagNumber(3)
  void clearSequenceLength() => clearField(3);

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
  $fixnum.Int64 get currentChunck => $_getI64(5);
  @$pb.TagNumber(6)
  set currentChunck($fixnum.Int64 v) { $_setInt64(5, v); }
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

  @$pb.TagNumber(8)
  $core.String get dataId => $_getSZ(7);
  @$pb.TagNumber(8)
  set dataId($core.String v) { $_setString(7, v); }
  @$pb.TagNumber(8)
  $core.bool hasDataId() => $_has(7);
  @$pb.TagNumber(8)
  void clearDataId() => clearField(8);
}

class DataUploadSetRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'DataUploadSetRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'dataId')
    ..aOS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'setName')
    ..a<$core.int>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'fileCounts', $pb.PbFieldType.OU3)
    ..aOS(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'currentFile')
    ..a<$fixnum.Int64>(5, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'totalChancks', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$core.int>(6, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'currentChunck', $pb.PbFieldType.OU3)
    ..a<$core.List<$core.int>>(7, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'chunck', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  DataUploadSetRequest._() : super();
  factory DataUploadSetRequest({
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
  factory DataUploadSetRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory DataUploadSetRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  DataUploadSetRequest clone() => DataUploadSetRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  DataUploadSetRequest copyWith(void Function(DataUploadSetRequest) updates) => super.copyWith((message) => updates(message as DataUploadSetRequest)) as DataUploadSetRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static DataUploadSetRequest create() => DataUploadSetRequest._();
  DataUploadSetRequest createEmptyInstance() => create();
  static $pb.PbList<DataUploadSetRequest> createRepeated() => $pb.PbList<DataUploadSetRequest>();
  @$core.pragma('dart2js:noInline')
  static DataUploadSetRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<DataUploadSetRequest>(create);
  static DataUploadSetRequest? _defaultInstance;

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

class DataUploadSetResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'DataUploadSetResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'result')
    ..hasRequiredFields = false
  ;

  DataUploadSetResponse._() : super();
  factory DataUploadSetResponse({
    $core.String? result,
  }) {
    final _result = create();
    if (result != null) {
      _result.result = result;
    }
    return _result;
  }
  factory DataUploadSetResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory DataUploadSetResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  DataUploadSetResponse clone() => DataUploadSetResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  DataUploadSetResponse copyWith(void Function(DataUploadSetResponse) updates) => super.copyWith((message) => updates(message as DataUploadSetResponse)) as DataUploadSetResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static DataUploadSetResponse create() => DataUploadSetResponse._();
  DataUploadSetResponse createEmptyInstance() => create();
  static $pb.PbList<DataUploadSetResponse> createRepeated() => $pb.PbList<DataUploadSetResponse>();
  @$core.pragma('dart2js:noInline')
  static DataUploadSetResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<DataUploadSetResponse>(create);
  static DataUploadSetResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get result => $_getSZ(0);
  @$pb.TagNumber(1)
  set result($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasResult() => $_has(0);
  @$pb.TagNumber(1)
  void clearResult() => clearField(1);
}

class DataDownloadSetRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'DataDownloadSetRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'dataId')
    ..hasRequiredFields = false
  ;

  DataDownloadSetRequest._() : super();
  factory DataDownloadSetRequest({
    $core.String? dataId,
  }) {
    final _result = create();
    if (dataId != null) {
      _result.dataId = dataId;
    }
    return _result;
  }
  factory DataDownloadSetRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory DataDownloadSetRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  DataDownloadSetRequest clone() => DataDownloadSetRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  DataDownloadSetRequest copyWith(void Function(DataDownloadSetRequest) updates) => super.copyWith((message) => updates(message as DataDownloadSetRequest)) as DataDownloadSetRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static DataDownloadSetRequest create() => DataDownloadSetRequest._();
  DataDownloadSetRequest createEmptyInstance() => create();
  static $pb.PbList<DataDownloadSetRequest> createRepeated() => $pb.PbList<DataDownloadSetRequest>();
  @$core.pragma('dart2js:noInline')
  static DataDownloadSetRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<DataDownloadSetRequest>(create);
  static DataDownloadSetRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get dataId => $_getSZ(0);
  @$pb.TagNumber(1)
  set dataId($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasDataId() => $_has(0);
  @$pb.TagNumber(1)
  void clearDataId() => clearField(1);
}

class DataDownloadSetResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'DataDownloadSetResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'dataId')
    ..aOS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'setName')
    ..a<$core.int>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'fileCounts', $pb.PbFieldType.OU3)
    ..aOS(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'currentFile')
    ..a<$fixnum.Int64>(5, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'totalChancks', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$core.int>(6, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'currentChunck', $pb.PbFieldType.OU3)
    ..a<$core.List<$core.int>>(7, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'chunck', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  DataDownloadSetResponse._() : super();
  factory DataDownloadSetResponse({
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
  factory DataDownloadSetResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory DataDownloadSetResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  DataDownloadSetResponse clone() => DataDownloadSetResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  DataDownloadSetResponse copyWith(void Function(DataDownloadSetResponse) updates) => super.copyWith((message) => updates(message as DataDownloadSetResponse)) as DataDownloadSetResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static DataDownloadSetResponse create() => DataDownloadSetResponse._();
  DataDownloadSetResponse createEmptyInstance() => create();
  static $pb.PbList<DataDownloadSetResponse> createRepeated() => $pb.PbList<DataDownloadSetResponse>();
  @$core.pragma('dart2js:noInline')
  static DataDownloadSetResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<DataDownloadSetResponse>(create);
  static DataDownloadSetResponse? _defaultInstance;

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

class AssociateDataRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'AssociateDataRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'dataId')
    ..a<$core.int>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'manageId', $pb.PbFieldType.O3)
    ..aOS(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'entityId')
    ..hasRequiredFields = false
  ;

  AssociateDataRequest._() : super();
  factory AssociateDataRequest({
    $core.String? dataId,
    $core.int? manageId,
    $core.String? entityId,
  }) {
    final _result = create();
    if (dataId != null) {
      _result.dataId = dataId;
    }
    if (manageId != null) {
      _result.manageId = manageId;
    }
    if (entityId != null) {
      _result.entityId = entityId;
    }
    return _result;
  }
  factory AssociateDataRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory AssociateDataRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  AssociateDataRequest clone() => AssociateDataRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  AssociateDataRequest copyWith(void Function(AssociateDataRequest) updates) => super.copyWith((message) => updates(message as AssociateDataRequest)) as AssociateDataRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static AssociateDataRequest create() => AssociateDataRequest._();
  AssociateDataRequest createEmptyInstance() => create();
  static $pb.PbList<AssociateDataRequest> createRepeated() => $pb.PbList<AssociateDataRequest>();
  @$core.pragma('dart2js:noInline')
  static AssociateDataRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<AssociateDataRequest>(create);
  static AssociateDataRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get dataId => $_getSZ(0);
  @$pb.TagNumber(1)
  set dataId($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasDataId() => $_has(0);
  @$pb.TagNumber(1)
  void clearDataId() => clearField(1);

  @$pb.TagNumber(2)
  $core.int get manageId => $_getIZ(1);
  @$pb.TagNumber(2)
  set manageId($core.int v) { $_setSignedInt32(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasManageId() => $_has(1);
  @$pb.TagNumber(2)
  void clearManageId() => clearField(2);

  @$pb.TagNumber(3)
  $core.String get entityId => $_getSZ(2);
  @$pb.TagNumber(3)
  set entityId($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasEntityId() => $_has(2);
  @$pb.TagNumber(3)
  void clearEntityId() => clearField(3);
}

class AssociateDataResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'AssociateDataResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'result')
    ..hasRequiredFields = false
  ;

  AssociateDataResponse._() : super();
  factory AssociateDataResponse({
    $core.String? result,
  }) {
    final _result = create();
    if (result != null) {
      _result.result = result;
    }
    return _result;
  }
  factory AssociateDataResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory AssociateDataResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  AssociateDataResponse clone() => AssociateDataResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  AssociateDataResponse copyWith(void Function(AssociateDataResponse) updates) => super.copyWith((message) => updates(message as AssociateDataResponse)) as AssociateDataResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static AssociateDataResponse create() => AssociateDataResponse._();
  AssociateDataResponse createEmptyInstance() => create();
  static $pb.PbList<AssociateDataResponse> createRepeated() => $pb.PbList<AssociateDataResponse>();
  @$core.pragma('dart2js:noInline')
  static AssociateDataResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<AssociateDataResponse>(create);
  static AssociateDataResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get result => $_getSZ(0);
  @$pb.TagNumber(1)
  set result($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasResult() => $_has(0);
  @$pb.TagNumber(1)
  void clearResult() => clearField(1);
}

class DisassociateDataRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'DisassociateDataRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'dataId')
    ..a<$core.int>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'manageId', $pb.PbFieldType.O3)
    ..aOS(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'entityId')
    ..hasRequiredFields = false
  ;

  DisassociateDataRequest._() : super();
  factory DisassociateDataRequest({
    $core.String? dataId,
    $core.int? manageId,
    $core.String? entityId,
  }) {
    final _result = create();
    if (dataId != null) {
      _result.dataId = dataId;
    }
    if (manageId != null) {
      _result.manageId = manageId;
    }
    if (entityId != null) {
      _result.entityId = entityId;
    }
    return _result;
  }
  factory DisassociateDataRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory DisassociateDataRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  DisassociateDataRequest clone() => DisassociateDataRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  DisassociateDataRequest copyWith(void Function(DisassociateDataRequest) updates) => super.copyWith((message) => updates(message as DisassociateDataRequest)) as DisassociateDataRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static DisassociateDataRequest create() => DisassociateDataRequest._();
  DisassociateDataRequest createEmptyInstance() => create();
  static $pb.PbList<DisassociateDataRequest> createRepeated() => $pb.PbList<DisassociateDataRequest>();
  @$core.pragma('dart2js:noInline')
  static DisassociateDataRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<DisassociateDataRequest>(create);
  static DisassociateDataRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get dataId => $_getSZ(0);
  @$pb.TagNumber(1)
  set dataId($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasDataId() => $_has(0);
  @$pb.TagNumber(1)
  void clearDataId() => clearField(1);

  @$pb.TagNumber(2)
  $core.int get manageId => $_getIZ(1);
  @$pb.TagNumber(2)
  set manageId($core.int v) { $_setSignedInt32(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasManageId() => $_has(1);
  @$pb.TagNumber(2)
  void clearManageId() => clearField(2);

  @$pb.TagNumber(3)
  $core.String get entityId => $_getSZ(2);
  @$pb.TagNumber(3)
  set entityId($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasEntityId() => $_has(2);
  @$pb.TagNumber(3)
  void clearEntityId() => clearField(3);
}

class DisassociateDataResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'DisassociateDataResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'result')
    ..hasRequiredFields = false
  ;

  DisassociateDataResponse._() : super();
  factory DisassociateDataResponse({
    $core.String? result,
  }) {
    final _result = create();
    if (result != null) {
      _result.result = result;
    }
    return _result;
  }
  factory DisassociateDataResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory DisassociateDataResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  DisassociateDataResponse clone() => DisassociateDataResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  DisassociateDataResponse copyWith(void Function(DisassociateDataResponse) updates) => super.copyWith((message) => updates(message as DisassociateDataResponse)) as DisassociateDataResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static DisassociateDataResponse create() => DisassociateDataResponse._();
  DisassociateDataResponse createEmptyInstance() => create();
  static $pb.PbList<DisassociateDataResponse> createRepeated() => $pb.PbList<DisassociateDataResponse>();
  @$core.pragma('dart2js:noInline')
  static DisassociateDataResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<DisassociateDataResponse>(create);
  static DisassociateDataResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get result => $_getSZ(0);
  @$pb.TagNumber(1)
  set result($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasResult() => $_has(0);
  @$pb.TagNumber(1)
  void clearResult() => clearField(1);
}

class GetDataInfoRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GetDataInfoRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'dataId')
    ..hasRequiredFields = false
  ;

  GetDataInfoRequest._() : super();
  factory GetDataInfoRequest({
    $core.String? dataId,
  }) {
    final _result = create();
    if (dataId != null) {
      _result.dataId = dataId;
    }
    return _result;
  }
  factory GetDataInfoRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetDataInfoRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetDataInfoRequest clone() => GetDataInfoRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetDataInfoRequest copyWith(void Function(GetDataInfoRequest) updates) => super.copyWith((message) => updates(message as GetDataInfoRequest)) as GetDataInfoRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GetDataInfoRequest create() => GetDataInfoRequest._();
  GetDataInfoRequest createEmptyInstance() => create();
  static $pb.PbList<GetDataInfoRequest> createRepeated() => $pb.PbList<GetDataInfoRequest>();
  @$core.pragma('dart2js:noInline')
  static GetDataInfoRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetDataInfoRequest>(create);
  static GetDataInfoRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get dataId => $_getSZ(0);
  @$pb.TagNumber(1)
  set dataId($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasDataId() => $_has(0);
  @$pb.TagNumber(1)
  void clearDataId() => clearField(1);
}

class GetDataInfoResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GetDataInfoResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'data', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  GetDataInfoResponse._() : super();
  factory GetDataInfoResponse({
    $core.List<$core.int>? data,
  }) {
    final _result = create();
    if (data != null) {
      _result.data = data;
    }
    return _result;
  }
  factory GetDataInfoResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetDataInfoResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetDataInfoResponse clone() => GetDataInfoResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetDataInfoResponse copyWith(void Function(GetDataInfoResponse) updates) => super.copyWith((message) => updates(message as GetDataInfoResponse)) as GetDataInfoResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GetDataInfoResponse create() => GetDataInfoResponse._();
  GetDataInfoResponse createEmptyInstance() => create();
  static $pb.PbList<GetDataInfoResponse> createRepeated() => $pb.PbList<GetDataInfoResponse>();
  @$core.pragma('dart2js:noInline')
  static GetDataInfoResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetDataInfoResponse>(create);
  static GetDataInfoResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get data => $_getN(0);
  @$pb.TagNumber(1)
  set data($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasData() => $_has(0);
  @$pb.TagNumber(1)
  void clearData() => clearField(1);
}

class MarkDataRemovedRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'MarkDataRemovedRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'dataId')
    ..hasRequiredFields = false
  ;

  MarkDataRemovedRequest._() : super();
  factory MarkDataRemovedRequest({
    $core.String? dataId,
  }) {
    final _result = create();
    if (dataId != null) {
      _result.dataId = dataId;
    }
    return _result;
  }
  factory MarkDataRemovedRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory MarkDataRemovedRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  MarkDataRemovedRequest clone() => MarkDataRemovedRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  MarkDataRemovedRequest copyWith(void Function(MarkDataRemovedRequest) updates) => super.copyWith((message) => updates(message as MarkDataRemovedRequest)) as MarkDataRemovedRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static MarkDataRemovedRequest create() => MarkDataRemovedRequest._();
  MarkDataRemovedRequest createEmptyInstance() => create();
  static $pb.PbList<MarkDataRemovedRequest> createRepeated() => $pb.PbList<MarkDataRemovedRequest>();
  @$core.pragma('dart2js:noInline')
  static MarkDataRemovedRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<MarkDataRemovedRequest>(create);
  static MarkDataRemovedRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get dataId => $_getSZ(0);
  @$pb.TagNumber(1)
  set dataId($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasDataId() => $_has(0);
  @$pb.TagNumber(1)
  void clearDataId() => clearField(1);
}

class MarkDataRemovedResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'MarkDataRemovedResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'result')
    ..hasRequiredFields = false
  ;

  MarkDataRemovedResponse._() : super();
  factory MarkDataRemovedResponse({
    $core.String? result,
  }) {
    final _result = create();
    if (result != null) {
      _result.result = result;
    }
    return _result;
  }
  factory MarkDataRemovedResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory MarkDataRemovedResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  MarkDataRemovedResponse clone() => MarkDataRemovedResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  MarkDataRemovedResponse copyWith(void Function(MarkDataRemovedResponse) updates) => super.copyWith((message) => updates(message as MarkDataRemovedResponse)) as MarkDataRemovedResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static MarkDataRemovedResponse create() => MarkDataRemovedResponse._();
  MarkDataRemovedResponse createEmptyInstance() => create();
  static $pb.PbList<MarkDataRemovedResponse> createRepeated() => $pb.PbList<MarkDataRemovedResponse>();
  @$core.pragma('dart2js:noInline')
  static MarkDataRemovedResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<MarkDataRemovedResponse>(create);
  static MarkDataRemovedResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get result => $_getSZ(0);
  @$pb.TagNumber(1)
  set result($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasResult() => $_has(0);
  @$pb.TagNumber(1)
  void clearResult() => clearField(1);
}

