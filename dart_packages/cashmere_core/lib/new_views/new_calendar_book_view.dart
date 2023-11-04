import 'package:cashmere_core/new_entity_calls/view_to_request.dart';
import 'package:cashmere_core/protocols/calendar_book.pb.dart';
import 'package:cashmere_core/protocols/name.pb.dart';

class NewCalendarBookView extends ViewToRequest<NewCalendarBookRequest> {
  final Name name;
  final int manageId;
  final String entityId;
  final String description;

  NewCalendarBookView({required this.name, required this.manageId, required this.entityId, required this.description});

  @override
  NewCalendarBookRequest toRequest() {
    return NewCalendarBookRequest(
      name: name,
      manageId: manageId,
      entityId: entityId,
      description: description,
    );
  }
}
