import 'package:cashmere_core/new_views/new_person_view.dart';
import 'package:cashmere_core/protocols/person.pb.dart';
import 'package:cashmere_core/providers/new_providers/new_entity_state_notifier.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

final newCategoryNotifierProvider = StateNotifierProvider.autoDispose<
    NewEntityNotifier<NewPersonVew, NewPersonRequest, NewPersonResponse>, NewEntityNotifierState>((ref) {
  return NewEntityNotifier<NewPersonVew, NewPersonRequest, NewPersonResponse>();
});
