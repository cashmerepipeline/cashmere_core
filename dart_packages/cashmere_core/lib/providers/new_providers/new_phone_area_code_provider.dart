import 'package:cashmere_core/new_views/new_phone_area_code_view.dart';
import 'package:cashmere_core/protocols/phone_area_code.pb.dart';
import 'package:cashmere_core/providers/new_providers/new_entity_state_notifier.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

final newCategoryNotifierProvider = StateNotifierProvider.autoDispose<
    NewEntityNotifier<NewPhoneAreaCodeView, NewPhoneAreaCodeRequest, NewPhoneAreaCodeResponse>,
    NewEntityNotifierState>((ref) {
  return NewEntityNotifier<NewPhoneAreaCodeView, NewPhoneAreaCodeRequest, NewPhoneAreaCodeResponse>();
});
