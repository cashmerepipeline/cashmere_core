import 'package:cashmere_core/protocols/entity.pb.dart';
import 'package:cashmere_core/providers/edit_providers/edit_entity_field_state_notifier.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

import 'edit_entity_notifier_state.dart';

// 编辑单个字段
final editEntityMapFieldRemoveKeyNotifierProvider = StateNotifierProvider<
    EditEntityFieldNotifier<EditEntityMapFieldRemoveKeyRequest, EditEntityMapFieldRemoveKeyResponse>,
    EditEntityNotifierState>(
  (ref) => EditEntityFieldNotifier(),
);
