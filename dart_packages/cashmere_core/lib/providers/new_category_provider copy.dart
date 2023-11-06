import 'package:cashmere_core/new_views/new_category_view.dart';
import 'package:cashmere_core/protocols/category.pb.dart';
import 'package:cashmere_core/providers/new_entity_state_notifier.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

final newCategoryNotifierProvider = StateNotifierProvider.autoDispose<
    NewEntityNotifier<NewCategoryView, NewCategoryRequest, NewCategoryResponse>, NewEntityNotifierState>((ref) {
  return NewEntityNotifier<NewCategoryView, NewCategoryRequest, NewCategoryResponse>();
});
