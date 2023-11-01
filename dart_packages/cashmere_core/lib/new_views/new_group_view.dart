import 'package:cashmere_core/new_entity_calls/to_new_entity_request.dart';
import 'package:cashmere_core/protocols/group.pb.dart';
import 'package:cashmere_core/protocols/name.pb.dart';

class NewGroupView extends ViewToRequest<NewGroupRequest> {
  final Name name;
  final String newGroupId;
  final String description;

  NewGroupView({
    required this.name,
    required this.newGroupId,
    required this.description,
  });

  @override
  NewGroupRequest toRequest() {
    return NewGroupRequest(
      name: name,
      newGroupId: newGroupId,
      description: description,
    );
  }
}
