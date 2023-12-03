import 'package:cashmere_core/view_models/entity_view_base.dart';
import 'package:cashmere_core/view_models/set_general_fields.dart';

class GroupView extends EntityViewBase {
  GroupView(super.id);

  factory GroupView.fromMap(Map<String, dynamic> m) {
    GroupView v = GroupView(m['id']);
    
    v = setGeneralFields<GroupView>(v, m);

    return v;
  }
}
