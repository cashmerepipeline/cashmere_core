import 'package:cashmere_core/grpc_call.dart';
import 'package:cashmere_core/protocols/manage_schema.pb.dart';
import 'package:flutter/foundation.dart';
import 'package:grpc/grpc.dart';

Future<List<SchemaField>> fetchSchemaFields(
  int manageId,
  GrpcCall<GetManageSchemaRequest, GetManageSchemaResponse> stubCall,
  Map<String, String> metaData,
) async {
  final request = GetManageSchemaRequest(manageId: manageId);

  try {
    final response = await stubCall(request, options: CallOptions(metadata: metaData));

    final fields = response.fields;

    // debugPrint("schema fields of $manageId: $fields");

    if (fields.isNotEmpty) {
      return fields;
    } else {
      return Future.error("取得描写列表失败, 不能为空");
    }
  } catch (err) {
    return Future.error("获取模式错误: $err");
  }
}
