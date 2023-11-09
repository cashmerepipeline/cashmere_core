import 'dart:ffi';

// ignore: depend_on_referenced_packages
import 'package:cashmere_core/grpc_call.dart';
import 'package:fixnum/fixnum.dart';
import 'package:cashmere_core/protocols/manage.pb.dart';
import 'package:grpc/grpc.dart';

Future<Int64> fetchEntryCount(
  int manageId,
  GrpcCall<GetManageEntryCountRequest, GetManageEntryCountResponse> stubCall,
  Map<String, String> metaData,
) async {
  final request = GetManageEntryCountRequest(manageId: manageId);

  try {
    final response = await stubCall(request, options: CallOptions(metadata: metaData));

    return response.count;
  } catch (e) {
    return Future.error("取得实体页错误：$e");
  }
}
