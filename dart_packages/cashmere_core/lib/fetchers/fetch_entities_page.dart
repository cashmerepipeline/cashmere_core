import 'package:bson/bson.dart';
import 'package:grpc/grpc.dart';
import 'package:cashmere_core/protocols/entity.pb.dart';
import 'package:flutter/foundation.dart';
import '../grpc_call.dart';

// 通用新建接口，如果需要特别新建形式，则建新的函数
Future<List<Map<String, dynamic>>> fetchEntitiesPage(
  int manageId,
  int pageIndex,
  GrpcCall<GetEntitiesPageRequest, GetEntitiesPageResponse> stubCall,
  Map<String, String> metaData,
) async {
  debugPrint('fetchEntitiesPage: $manageId, $pageIndex');
  final request = GetEntitiesPageRequest(
    manageId: manageId,
    pageIndex: pageIndex,
  );
  try {
    final response = await stubCall(
      request,
      options: CallOptions(metadata: metaData),
    );

    final bson = BSON();
    final entities = response.entities.map((e) {
      return bson.deserialize(BsonBinary.from(e));
    }).toList();

    debugPrint('fetchEntitiesPage: $manageId, $pageIndex, ${entities.length}');
    return entities;
  } catch (err) {
    return Future.error(err.toString());
  }
}
