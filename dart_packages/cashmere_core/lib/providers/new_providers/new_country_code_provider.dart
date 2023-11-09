import 'package:cashmere_core/new_views/new_country_code_view.dart';
import 'package:cashmere_core/protocols/country_code.pb.dart';
import 'package:cashmere_core/providers/new_providers/new_entity_state_notifier.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

final newCategoryNotifierProvider = StateNotifierProvider.autoDispose<
    NewEntityNotifier<NewCountryCodeView, NewCountryCodeRequest, NewCountryCodeResponse>, NewEntityNotifierState>((ref) {
  return NewEntityNotifier<NewCountryCodeView, NewCountryCodeRequest, NewCountryCodeResponse>();
});