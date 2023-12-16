import 'package:bson/bson.dart';
import 'package:cashmere_core/grpc_call.dart';
import 'package:cashmere_core/protocols/category.pb.dart';
import 'package:grpc/grpc.dart';

Future<List<Map<String, dynamic>>> fetchCategories(
  GrpcCall<GetCategoriesRequest, GetCategoriesResponse> stubCall,
  Map<String, String> metaData,
  String manageId,
) async {
  final request = GetCategoriesRequest(manageId: manageId);

  final response = await stubCall(
    request,
    options: CallOptions(
      metadata: metaData,
    ),
  );

  final items = response.codes.map(((e) => BsonCodec.deserialize(BsonBinary.from(e))));

  return items.toList();
}
