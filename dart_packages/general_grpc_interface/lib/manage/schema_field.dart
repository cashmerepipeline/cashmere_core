// 取得管理描写表
import 'package:general_grpc_interface/core/name_map_utils.dart';
import 'package:general_grpc_interface/grpc_generated/manage_schema.pb.dart' as s_massge;

class SchemaField {
  final int fieldId;
  final Map<String, String> nameMap;
  final String dataType;
  final bool removed;

  SchemaField(this.fieldId, this.nameMap, this.dataType, this.removed);

  factory SchemaField.fromMessage(s_massge.SchemaField message) {
    final nameMap = nameMapFromBytes(message.nameMap);
    return SchemaField(message.id, nameMap, message.dataType, message.removed);
  }
}
