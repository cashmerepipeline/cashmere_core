import 'package:cashmere_core/new_views/new_language_name_view.dart';
import 'package:cashmere_core/protocols/name.pb.dart';
import 'package:cashmere_core/providers/new_entity_state_notifier.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

final newCategoryNotifierProvider = StateNotifierProvider.autoDispose<
    NewEntityNotifier<NewLanguageNameView, NewLanguageNameRequest, NewLanguageNameResponse>, NewEntityNotifierState>((ref) {
  return NewEntityNotifier<NewLanguageNameView, NewLanguageNameRequest, NewLanguageNameResponse>();
});
