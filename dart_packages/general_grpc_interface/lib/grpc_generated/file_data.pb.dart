///
//  Generated code. Do not modify.
//  source: file_data.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

import 'file_info.pb.dart' as $0;

class FileDataUploadFileRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'FileDataUploadFileRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'dataId')
    ..a<$fixnum.Int64>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'totalChunks', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'currentChunkIndex', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$core.List<$core.int>>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'chunk', $pb.PbFieldType.OY)
    ..aOM<$0.FileInfo>(5, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'fileInfo', subBuilder: $0.FileInfo.create)
    ..aOS(6, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'chunkMd5')
    ..hasRequiredFields = false
  ;

  FileDataUploadFileRequest._() : super();
  factory FileDataUploadFileRequest({
    $core.String? dataId,
    $fixnum.Int64? totalChunks,
    $fixnum.Int64? currentChunkIndex,
    $core.List<$core.int>? chunk,
    $0.FileInfo? fileInfo,
    $core.String? chunkMd5,
  }) {
    final _result = create();
    if (dataId != null) {
      _result.dataId = dataId;
    }
    if (totalChunks != null) {
      _result.totalChunks = totalChunks;
    }
    if (currentChunkIndex != null) {
      _result.currentChunkIndex = currentChunkIndex;
    }
    if (chunk != null) {
      _result.chunk = chunk;
    }
    if (fileInfo != null) {
      _result.fileInfo = fileInfo;
    }
    if (chunkMd5 != null) {
      _result.chunkMd5 = chunkMd5;
    }
    return _result;
  }
  factory FileDataUploadFileRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory FileDataUploadFileRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  FileDataUploadFileRequest clone() => FileDataUploadFileRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  FileDataUploadFileRequest copyWith(void Function(FileDataUploadFileRequest) updates) => super.copyWith((message) => updates(message as FileDataUploadFileRequest)) as FileDataUploadFileRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static FileDataUploadFileRequest create() => FileDataUploadFileRequest._();
  FileDataUploadFileRequest createEmptyInstance() => create();
  static $pb.PbList<FileDataUploadFileRequest> createRepeated() => $pb.PbList<FileDataUploadFileRequest>();
  @$core.pragma('dart2js:noInline')
  static FileDataUploadFileRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<FileDataUploadFileRequest>(create);
  static FileDataUploadFileRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get dataId => $_getSZ(0);
  @$pb.TagNumber(1)
  set dataId($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasDataId() => $_has(0);
  @$pb.TagNumber(1)
  void clearDataId() => clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get totalChunks => $_getI64(1);
  @$pb.TagNumber(2)
  set totalChunks($fixnum.Int64 v) { $_setInt64(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasTotalChunks() => $_has(1);
  @$pb.TagNumber(2)
  void clearTotalChunks() => clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get currentChunkIndex => $_getI64(2);
  @$pb.TagNumber(3)
  set currentChunkIndex($fixnum.Int64 v) { $_setInt64(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasCurrentChunkIndex() => $_has(2);
  @$pb.TagNumber(3)
  void clearCurrentChunkIndex() => clearField(3);

  @$pb.TagNumber(4)
  $core.List<$core.int> get chunk => $_getN(3);
  @$pb.TagNumber(4)
  set chunk($core.List<$core.int> v) { $_setBytes(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasChunk() => $_has(3);
  @$pb.TagNumber(4)
  void clearChunk() => clearField(4);

  @$pb.TagNumber(5)
  $0.FileInfo get fileInfo => $_getN(4);
  @$pb.TagNumber(5)
  set fileInfo($0.FileInfo v) { setField(5, v); }
  @$pb.TagNumber(5)
  $core.bool hasFileInfo() => $_has(4);
  @$pb.TagNumber(5)
  void clearFileInfo() => clearField(5);
  @$pb.TagNumber(5)
  $0.FileInfo ensureFileInfo() => $_ensure(4);

  @$pb.TagNumber(6)
  $core.String get chunkMd5 => $_getSZ(5);
  @$pb.TagNumber(6)
  set chunkMd5($core.String v) { $_setString(5, v); }
  @$pb.TagNumber(6)
  $core.bool hasChunkMd5() => $_has(5);
  @$pb.TagNumber(6)
  void clearChunkMd5() => clearField(6);
}

class FileDataUploadFileResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'FileDataUploadFileResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..a<$fixnum.Int64>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'nextChunkIndex', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false
  ;

  FileDataUploadFileResponse._() : super();
  factory FileDataUploadFileResponse({
    $fixnum.Int64? nextChunkIndex,
  }) {
    final _result = create();
    if (nextChunkIndex != null) {
      _result.nextChunkIndex = nextChunkIndex;
    }
    return _result;
  }
  factory FileDataUploadFileResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory FileDataUploadFileResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  FileDataUploadFileResponse clone() => FileDataUploadFileResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  FileDataUploadFileResponse copyWith(void Function(FileDataUploadFileResponse) updates) => super.copyWith((message) => updates(message as FileDataUploadFileResponse)) as FileDataUploadFileResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static FileDataUploadFileResponse create() => FileDataUploadFileResponse._();
  FileDataUploadFileResponse createEmptyInstance() => create();
  static $pb.PbList<FileDataUploadFileResponse> createRepeated() => $pb.PbList<FileDataUploadFileResponse>();
  @$core.pragma('dart2js:noInline')
  static FileDataUploadFileResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<FileDataUploadFileResponse>(create);
  static FileDataUploadFileResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get nextChunkIndex => $_getI64(0);
  @$pb.TagNumber(1)
  set nextChunkIndex($fixnum.Int64 v) { $_setInt64(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasNextChunkIndex() => $_has(0);
  @$pb.TagNumber(1)
  void clearNextChunkIndex() => clearField(1);
}

class FileDataDownloadFileRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'FileDataDownloadFileRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'dataId')
    ..hasRequiredFields = false
  ;

  FileDataDownloadFileRequest._() : super();
  factory FileDataDownloadFileRequest({
    $core.String? dataId,
  }) {
    final _result = create();
    if (dataId != null) {
      _result.dataId = dataId;
    }
    return _result;
  }
  factory FileDataDownloadFileRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory FileDataDownloadFileRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  FileDataDownloadFileRequest clone() => FileDataDownloadFileRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  FileDataDownloadFileRequest copyWith(void Function(FileDataDownloadFileRequest) updates) => super.copyWith((message) => updates(message as FileDataDownloadFileRequest)) as FileDataDownloadFileRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static FileDataDownloadFileRequest create() => FileDataDownloadFileRequest._();
  FileDataDownloadFileRequest createEmptyInstance() => create();
  static $pb.PbList<FileDataDownloadFileRequest> createRepeated() => $pb.PbList<FileDataDownloadFileRequest>();
  @$core.pragma('dart2js:noInline')
  static FileDataDownloadFileRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<FileDataDownloadFileRequest>(create);
  static FileDataDownloadFileRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get dataId => $_getSZ(0);
  @$pb.TagNumber(1)
  set dataId($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasDataId() => $_has(0);
  @$pb.TagNumber(1)
  void clearDataId() => clearField(1);
}

class FileDataDownloadFileResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'FileDataDownloadFileResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'cashmere'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'dataId')
    ..a<$fixnum.Int64>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'totalChunks', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'currentChunk', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$core.List<$core.int>>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'chunk', $pb.PbFieldType.OY)
    ..aOS(5, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'fileName')
    ..hasRequiredFields = false
  ;

  FileDataDownloadFileResponse._() : super();
  factory FileDataDownloadFileResponse({
    $core.String? dataId,
    $fixnum.Int64? totalChunks,
    $fixnum.Int64? currentChunk,
    $core.List<$core.int>? chunk,
    $core.String? fileName,
  }) {
    final _result = create();
    if (dataId != null) {
      _result.dataId = dataId;
    }
    if (totalChunks != null) {
      _result.totalChunks = totalChunks;
    }
    if (currentChunk != null) {
      _result.currentChunk = currentChunk;
    }
    if (chunk != null) {
      _result.chunk = chunk;
    }
    if (fileName != null) {
      _result.fileName = fileName;
    }
    return _result;
  }
  factory FileDataDownloadFileResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory FileDataDownloadFileResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  FileDataDownloadFileResponse clone() => FileDataDownloadFileResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  FileDataDownloadFileResponse copyWith(void Function(FileDataDownloadFileResponse) updates) => super.copyWith((message) => updates(message as FileDataDownloadFileResponse)) as FileDataDownloadFileResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static FileDataDownloadFileResponse create() => FileDataDownloadFileResponse._();
  FileDataDownloadFileResponse createEmptyInstance() => create();
  static $pb.PbList<FileDataDownloadFileResponse> createRepeated() => $pb.PbList<FileDataDownloadFileResponse>();
  @$core.pragma('dart2js:noInline')
  static FileDataDownloadFileResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<FileDataDownloadFileResponse>(create);
  static FileDataDownloadFileResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get dataId => $_getSZ(0);
  @$pb.TagNumber(1)
  set dataId($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasDataId() => $_has(0);
  @$pb.TagNumber(1)
  void clearDataId() => clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get totalChunks => $_getI64(1);
  @$pb.TagNumber(2)
  set totalChunks($fixnum.Int64 v) { $_setInt64(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasTotalChunks() => $_has(1);
  @$pb.TagNumber(2)
  void clearTotalChunks() => clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get currentChunk => $_getI64(2);
  @$pb.TagNumber(3)
  set currentChunk($fixnum.Int64 v) { $_setInt64(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasCurrentChunk() => $_has(2);
  @$pb.TagNumber(3)
  void clearCurrentChunk() => clearField(3);

  @$pb.TagNumber(4)
  $core.List<$core.int> get chunk => $_getN(3);
  @$pb.TagNumber(4)
  set chunk($core.List<$core.int> v) { $_setBytes(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasChunk() => $_has(3);
  @$pb.TagNumber(4)
  void clearChunk() => clearField(4);

  @$pb.TagNumber(5)
  $core.String get fileName => $_getSZ(4);
  @$pb.TagNumber(5)
  set fileName($core.String v) { $_setString(4, v); }
  @$pb.TagNumber(5)
  $core.bool hasFileName() => $_has(4);
  @$pb.TagNumber(5)
  void clearFileName() => clearField(5);
}

