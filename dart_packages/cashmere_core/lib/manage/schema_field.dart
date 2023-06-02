// 取得管理描写表
import 'package:cashmere_core/core/name_map_utils.dart';
import 'package:cashmere_core/protocols/manage_schema.pb.dart' as s_massge;

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

  toString() {
    return "\{fieldID: $fieldId, dataType: $dataType, nameMap: $nameMap, removed: $removed}";
  }
}
