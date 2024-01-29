import 'package:cashmere_core/ids/field_ids.dart';
import 'package:cashmere_core/ids/general_field_ids.dart';
import 'package:cashmere_core/view_models/entity_view_base.dart';
import 'package:cashmere_core/view_models/set_general_fields.dart';

class CategoryView extends EntityViewBase {
  final String manageId;
  CategoryView(super.id, {required this.manageId});

  factory CategoryView.fromMap(Map<String, dynamic> map) {
    final CategoryView categoryView = CategoryView(
      map[ID_FIELD_ID.toString()],
      manageId: map[CATEGORIES_MANAGE_ID_FIELD_ID.toString()],
    );
    final result = setGeneralFields(categoryView, map);
    return result;
  }
}
