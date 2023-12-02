import 'package:bson/bson.dart';
import 'package:cashmere_core/grpc_call.dart';
import 'package:cashmere_core/protocols/country_code.pb.dart';
import 'package:grpc/grpc.dart';

Future<List<Map<String, dynamic>>> fetchCountryCodes(
  GrpcCall<GetCountryCodesRequest, GetCountryCodesResponse> stubCall,
  Map<String, String> metaData,
) async {
  final request = GetCountryCodesRequest();

  try {
    final response = await stubCall(request, options: CallOptions(metadata: metaData));

    return response.codes.map((e) {
      return BsonCodec.deserialize(BsonBinary.from(e));
    }).toList();
  } catch (err) {
    return Future.error("获取国家编码错误: $err");
  }
}
