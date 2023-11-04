import 'package:cashmere_core/new_views/new_group_view.dart';
import 'package:cashmere_core/protocols/group.pb.dart';
import 'package:cashmere_core/providers/new_entity_state_notifier.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

final newCategoryNotifierProvider = StateNotifierProvider.autoDispose<
    NewEntityNotifier<NewGroupView, NewGroupRequest, NewGroupResponse>, NewEntityNotifierState>((ref) {
  return NewEntityNotifier<NewGroupView, NewGroupRequest, NewGroupResponse>();
});
