import 'package:cashmere_core/manage/schema_field.dart';

String getSchemaFieldName(
    List<SchemaField> fields, int fieldId, String languageCode) {
  try {
    final field = fields.firstWhere((i) {
      return i.fieldId == fieldId;
    });

    final name = field.nameMap[languageCode];
    return name ?? "no language name";
  } catch (e) {
    // TODO: 日志
    print(e.toString());
    return "no language name";
  }
}
