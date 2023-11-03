import 'package:cashmere_core/new_entity_calls/view_to_request.dart';
import 'package:cashmere_core/protocols/book.pb.dart';
import 'package:cashmere_core/protocols/name.pb.dart';

class NewBookView extends ViewToRequest<NewBookRequest> {
  final Name name;
  final int manageId;
  final String entityId;
  final String description;

  NewBookView({required this.name, required this.manageId, required this.entityId, required this.description});

  @override
  NewBookRequest toRequest() {
    return NewBookRequest(
      name: name,
      manageId: manageId,
      entityId: entityId,
      description: description,
    );
  }
}
