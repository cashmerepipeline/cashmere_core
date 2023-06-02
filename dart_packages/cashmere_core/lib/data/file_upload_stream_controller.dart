import 'dart:async';
import 'dart:io';
import 'dart:typed_data';

import 'package:crypto/crypto.dart';
import 'package:fixnum/fixnum.dart';
import 'package:imixapp/client/protocols/file_info.pb.dart';
import 'package:imixapp/client/protocols/file_transfer.pb.dart';

const ChuckSize = 1024 * 128 * 1;

class UploadFileStreamController {
  final String dataId;
  final String filePath;

  late StreamController<UploadFileRequest> uploadStreamController;
  late UploadFileRequest firstRequest;
  late FileInfo fileInfo;
  late Uint8List bytes;
  late int totalChuncks;
  get stream => uploadStreamController.stream;

  UploadFileStreamController(this.dataId, this.filePath) {
    uploadStreamController = StreamController<UploadFileRequest>();

    final file = File(filePath);
    bytes = file.readAsBytesSync();
    totalChuncks = (bytes.length / ChuckSize).ceil();
    final fileDigest = md5.convert(bytes);
    final modifyTime = file.lastModifiedSync();
    final fileName = file.path.split('/').last;

    fileInfo = FileInfo(
      fileName: fileName,
      md5: fileDigest.toString(),
      size: Int64(bytes.length),
      lastModifiedTime: Int64(modifyTime.microsecondsSinceEpoch),
    );

    firstRequest = UploadFileRequest(
        dataId: dataId,
        chunk: [0, 0, 0],
        currentChunkIndex: Int64(0),
        totalChunks: Int64(totalChuncks),
        fileInfo: fileInfo,
        chunkMd5: "no_md");
  }

  // 第1个包由客户端主动发出
  sendFirst() {
    uploadStreamController.add(firstRequest);
  }

  // 就收到服务端应答后发送下一个包
  // 发送从0开始，到0结束
  sendNext(Int64 transferIndex) {
    // 实际数据编号比包编号小1
    final i = (transferIndex - 1).toInt();
    final start = i * ChuckSize;
    var end = (i + 1) * ChuckSize;
    if (end > bytes.length) {
      end = bytes.length;
    }

    final chunk = bytes.sublist(start, end);
    print("chunk ${i}: ${chunk.length}/${totalChuncks}");

    uploadStreamController.add(UploadFileRequest(
      dataId: dataId,
      chunk: chunk,
      currentChunkIndex: transferIndex,
      totalChunks: Int64(totalChuncks),
      fileInfo: fileInfo,
    ));
  }

  // 到0结束
  sendLast() {
    sendFirst();
  }

  close() {
    uploadStreamController.close();
  }
}
