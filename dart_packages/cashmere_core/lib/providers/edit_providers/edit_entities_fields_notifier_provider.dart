import 'package:cashmere_core/providers/edit_providers/edit_entities_fields_state_notifier.dart';
import 'package:cashmere_core/providers/edit_providers/edit_entity_notifier_state.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

final editEntitiesFieldsNotifierProvider = StateNotifierProvider<EditEntitiesFieldsNotifier, EditEntityNotifierState>(
  (ref) => EditEntitiesFieldsNotifier(),
);
