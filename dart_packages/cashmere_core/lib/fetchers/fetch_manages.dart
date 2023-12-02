import 'package:bson/bson.dart';
import 'package:cashmere_core/grpc_call.dart';
import 'package:cashmere_core/protocols/manage.pb.dart';
import 'package:grpc/grpc.dart';

Future<List<Map<String, dynamic>>> fetchManages(
  GrpcCall<GetManagesRequest, GetManagesResponse> stubCall,
  Map<String, String> metaData,
) async {
  final request = GetManagesRequest();

  try {
    final response = await stubCall(request, options: CallOptions(metadata: metaData));

    final items = response.manages.map((e) => BsonCodec.deserialize(BsonBinary.from(e))).toList();

    if (items.isNotEmpty) {
      return items;
    } else {
      return Future.error("取得管理列表失败");
    }
  } catch (err) {
    return Future.error("获取模式错误: $err");
  }
}
