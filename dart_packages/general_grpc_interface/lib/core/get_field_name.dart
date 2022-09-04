import 'package:general_grpc_interface/general_name_map.dart';

String getSchemaFieldName(int fieldId, String languageCode) {
  final nameMap = generalSchemaFieldsNameMap[fieldId];
  if (nameMap == null) return "No Lang Name";

  final name = nameMap[languageCode];
  if (name == null) return "No Lang Name";

  return name;
}
