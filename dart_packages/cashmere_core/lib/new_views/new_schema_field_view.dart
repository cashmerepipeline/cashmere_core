import 'package:cashmere_core/new_entity_calls/view_to_request.dart';
import 'package:cashmere_core/protocols/manage_schema.pb.dart';

class NewSchemaFieldView extends ViewToRequest<NewSchemaFieldRequest> {
  final String manageId;
  final SchemaField newField;

  NewSchemaFieldView({required this.manageId, required this.newField});

  @override
  NewSchemaFieldRequest toRequest() {
    return NewSchemaFieldRequest(manageId: manageId, newField: newField);
  }
}
