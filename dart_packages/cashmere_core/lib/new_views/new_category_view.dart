import 'package:cashmere_core/protocols/name.pb.dart';

class NewCategoryView {
  final int manageId;
  final Name name;
  final String description;

  NewCategoryView({required this.manageId, required this.name, required this.description});
}
