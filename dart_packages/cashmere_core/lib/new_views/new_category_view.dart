import 'package:cashmere_core/new_entity_calls/view_to_request.dart';
import 'package:cashmere_core/protocols/category.pb.dart';
import 'package:cashmere_core/protocols/name.pb.dart';

class NewCategoryView extends ViewToRequest<NewCategoryRequest> {
  final String manageId;
  final Name name;
  final String description;

  NewCategoryView({required this.manageId, required this.name, required this.description});

  @override
  NewCategoryRequest toRequest() {
    return NewCategoryRequest(
      manageId: manageId,
      name: name,
      description: description,
    );
  }
}
