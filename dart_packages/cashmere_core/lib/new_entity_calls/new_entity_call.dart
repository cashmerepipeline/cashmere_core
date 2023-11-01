import 'package:grpc/grpc.dart';

import '../grpc_call.dart';

abstract class ToNewEntityRequest<Req> {
  Req toRequest();
}

// 通用新建接口，如果需要特别新建形式，则建新的函数
Future<Res> newEntity<Req, Res>(
  ToNewEntityRequest newView,
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
