import 'package:bson/bson.dart';
import 'package:cashmere_core/grpc_call.dart';
import 'package:cashmere_core/protocols/phone_area_code.pb.dart';
import 'package:grpc/grpc.dart';

Future<List<Map<String, dynamic>>> fetchPhoneAreaCodes(
  GrpcCall<GetPhoneAreaCodesRequest, GetPhoneAreaCodesResponse> stubCall,
  Map<String, String> metaData,
) async {
  final request = GetPhoneAreaCodesRequest();

  final response = await stubCall(
    request,
    options: CallOptions(
      metadata: metaData,
    ),
  );

  final bson = BSON();
  final items = response.codes.map((e) => bson.deserialize(BsonBinary.from(e))).toList();

  return items;
}
