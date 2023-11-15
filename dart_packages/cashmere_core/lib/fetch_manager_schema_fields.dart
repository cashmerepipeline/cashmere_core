import 'dart:async';

import 'package:cashmere_core/protocols/manage_schema.pb.dart' as schema_message;
import 'package:cashmere_core/manage/schema_field.dart';

import 'package:grpc/grpc.dart';

Future<List<SchemaField>?> fetchManageSchemaFields(
  dynamic clientStub,
  Map<String, String> metaData,
  int manageId,
) async {
  if (clientStub == null) return null;

  final result = <SchemaField>[];
  final sRequest = schema_message.GetManageSchemaRequest(manageId: manageId);
  try {
    final resp = await clientStub.getManageSchema(sRequest, options: CallOptions(metadata: metaData));

    for (var f in resp.fields) {
      final field = SchemaField.fromMessage(f);
      result.add(field);
    }

    // 排序
    final _rSorted = result.sort(((a, b) {
      return a.fieldId.compareTo(b.fieldId);
    }));

    return result;
  } catch (e) {
    // TODO: 错误日志
    print(e);
    return Future.error(e);
  }
}
