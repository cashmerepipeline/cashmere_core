import 'dart:async';

import 'package:bson/bson.dart';
import 'package:grpc/grpc.dart';
import 'package:cashmere_core/protocols/entity.pb.dart';
import 'package:flutter/foundation.dart';
import '../grpc_call.dart';

// 通用新建接口，如果需要特别新建形式，则建新的函数
Future<Stream<Map<String, dynamic>>> fetchEntitiesPage(
  String manageId,
  int pageIndex,
  String startOid,
  Map<String, int> sortDoc,
  ResponseStreamGrpcCall<GetEntitiesPageRequest, GetEntitiesPageResponse> stubCall,
  Map<String, String> metaData,
) async {
  debugPrint('fetchEntitiesPage: $manageId, $pageIndex');
  final sort = BsonCodec.serialize(sortDoc).byteList;
  final request = GetEntitiesPageRequest(
    manageId: manageId,
    pageIndex: pageIndex,
    startOid: startOid,
    sortDoc: sort,
  );
  try {
    final response = await stubCall(
      request,
      options: CallOptions(metadata: metaData),
    );

    final entityStream = response.map((resp) {
      return BsonCodec.deserialize(BsonBinary.from(resp.entity));
    });

    return entityStream;
  } catch (err) {
    return Future.error(err.toString());
  }
}
