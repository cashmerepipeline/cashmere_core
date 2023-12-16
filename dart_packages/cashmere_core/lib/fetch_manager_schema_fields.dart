import 'dart:async';
import 'dart:developer';

import 'package:cashmere_core/grpc_call.dart';
import 'package:cashmere_core/protocols/manage_schema.pb.dart';

import 'package:grpc/grpc.dart';

Future<List<SchemaField>?> fetchManageSchemaFields(
  GrpcCall<GetManageSchemaRequest, GetManageSchemaResponse> stubCall,
  Map<String, String> metaData,
  String manageId,
) async {
  final sRequest = GetManageSchemaRequest(manageId: manageId);
  try {
    final resp = await stubCall(sRequest, options: CallOptions(metadata: metaData));
    return resp.fields;
  } catch (e) {
    log(e.toString());
    return Future.error(e);
  }
}
