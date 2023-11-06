import 'package:grpc/grpc.dart';

import 'package:cashmere_core/grpc_call.dart';
import 'package:cashmere_core/new_entity_calls/view_to_request.dart';

// 通用新建接口，如果需要特别新建形式，则建新的函数
Future<Res> newEntityCall<Req, Res>(
  ViewToRequest newView,
  GrpcCall<Req, Res> stubCall,
  Map<String, String> metaData,
) async {
  final request = newView.toRequest();

  try {
    final response = await stubCall(
      request,
      options: CallOptions(metadata: metaData),
    );
    return response;
  } catch (err) {
    return Future.error(err.toString());
  }
}
