import 'package:bson/bson.dart';
import 'package:cashmere_core/grpc_call.dart';
import 'package:cashmere_core/protocols/country_code.pb.dart';
import 'package:cashmere_core/protocols/language_code.pb.dart';
import 'package:grpc/grpc.dart';

Future<List<Map<String, dynamic>>> fetchLanguageCodes(
  GrpcCall<GetLanguageCodesRequest, GetCountryCodesResponse> stubCall,
  Map<String, String> metaData,
) async {
  final request = GetLanguageCodesRequest();

  final response = await stubCall(
    request,
    options: CallOptions(
      metadata: metaData,
    ),
  );

  final items = response.codes.map((e) => BsonCodec.deserialize(BsonBinary.from(e))).toList();

  return items;
}
