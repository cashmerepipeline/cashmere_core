import 'package:cashmere_core/new_entity_calls/view_to_request.dart';
import 'package:cashmere_core/protocols/name.pb.dart';
import 'package:cashmere_core/protocols/tag.pb.dart';

class NewTagView extends ViewToRequest<NewTagRequest> {
  final int targetManageId;
  final Name name;
  final String description;

  NewTagView({
    required this.targetManageId,
    required this.name,
    required this.description,
  });

  @override
  NewTagRequest toRequest() {
    return NewTagRequest(
      targetManageId: targetManageId,
      name: name,
      description: description,
    );
  }
}
