import 'package:cashmere_core/new_entity_calls/view_to_request.dart';
import 'package:cashmere_core/protocols/name.pb.dart';

class NewLanguageNameView extends ViewToRequest<NewLanguageNameRequest> {
  final int manageId;
  final String entityId;
  final Name newName;

  NewLanguageNameView({
    required this.manageId,
    required this.entityId,
    required this.newName,
  });

  @override
  NewLanguageNameRequest toRequest() {
    return NewLanguageNameRequest(
      manageId: manageId,
      entityId: entityId,
      newName: newName,
    );
  }
}
