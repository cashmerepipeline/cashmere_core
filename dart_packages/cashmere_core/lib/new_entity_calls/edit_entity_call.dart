import 'package:cashmere_core/protocols/entity.pb.dart';
import 'package:grpc/grpc.dart';

import 'package:cashmere_core/grpc_call.dart';

// 通用新建接口，如果需要特别新建形式，则建新的函数
Future<EditEntitiesFielsdsResponse> editEntityCall(
  List<EntityFieldEdit> edits,
  GrpcCall<EditEntitiesFielsdsRequest, EditEntitiesFielsdsResponse> stubCall,
  Map<String, String> metaData,
) async {
  try {
    final request = EditEntitiesFielsdsRequest(
      edits: edits,
    );
    final response = await stubCall(
      request,
      options: CallOptions(metadata: metaData),
    );
    return response;
  } catch (err) {
    return Future.error(err.toString());
  }
}
