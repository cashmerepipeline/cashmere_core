import 'package:grpc/grpc.dart';

import '../grpc_call.dart';

// 通用新建接口，如果需要特别新建形式，则建新的函数
Future<Res> fetchPrecodedCodes<Req, Res>(
  Req request,
  GrpcCall<Req, Res> stubCall,
  Map<String, String> metaData,
) async {
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
