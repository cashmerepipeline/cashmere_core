import 'package:cashmere_core/protocols/name.pb.dart';

class NewTagView {
  final int targetManageId;
  final Name name;
  final String description;

  NewTagView({
    required this.targetManageId,
    required this.name,
    required this.description,
  });
}
