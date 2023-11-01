import 'package:cashmere_core/protocols/manage_schema.pb.dart';

class NewSchemaFieldView {
  final int manage_id;
  final SchemaField field;

  NewSchemaFieldView({required this.manage_id, required this.field});
}
