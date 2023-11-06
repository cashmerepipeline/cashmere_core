import 'package:cashmere_core/new_entity_calls/view_to_request.dart';
import 'package:cashmere_core/protocols/member.pb.dart';
import 'package:cashmere_core/protocols/name.pb.dart';


class AddMemberView implements ViewToRequest<AddMemberRequest> {
  final Name name;
  final int ownerManageId;
  final String ownerEntityId;
  final int selfManageId;
  final String selfEntityId;
  final String description;

  AddMemberView({
    required this.name,
    required this.ownerManageId,
    required this.ownerEntityId,
    required this.selfManageId,
    required this.selfEntityId,
    required this.description,
  });

  @override
  AddMemberRequest toRequest() {
    return AddMemberRequest(
      name: name,
      ownerManageId: ownerManageId,
      ownerEntityId: ownerEntityId,
      selfManageId: selfManageId,
      selfEntityId: selfEntityId,
      description: description,
    );
  }
}
