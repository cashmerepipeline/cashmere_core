import 'package:cashmere_core/protocols/entity.pb.dart';
import 'package:cashmere_core/providers/edit_providers/edit_entity_field_state_notifier.dart';
import 'package:cashmere_core/providers/edit_providers/edit_entity_notifier_state.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

final editEntityArrayFieldAddItemsNotifierProvider = StateNotifierProvider<
    EditEntityFieldNotifier<EditEntityArrayFieldAddItemsRequest, EditEntityArrayFieldAddItemsResponse>,
    EditEntityNotifierState>(
  (ref) => EditEntityFieldNotifier(),
);
