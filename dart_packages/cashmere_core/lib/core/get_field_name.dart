
import 'package:cashmere_core/protocols/manage_schema.pb.dart';

String getSchemaFieldName(
    List<SchemaField> fields, int fieldId, String languageCode) {
  try {
    final field = fields.firstWhere((i) {
      return i.id == fieldId;
    });

    final name = field.nameMap[languageCode];
    return name ?? "no language name";
  } catch (e) {
    // TODO: 日志
    print(e.toString());
    return "no language name";
  }
}
