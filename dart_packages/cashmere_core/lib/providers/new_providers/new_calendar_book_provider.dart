import 'package:cashmere_core/new_views/new_calendar_book_view.dart';
import 'package:cashmere_core/protocols/calendar_book.pb.dart';
import 'package:cashmere_core/providers/new_providers/new_entity_state_notifier.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

typedef NewCalenderBookNotifier
    = NewEntityNotifier<NewCalendarBookView, NewCalendarBookRequest, NewCalendarBookResponse>;

final newCalendarBookNotifierProvider =
    StateNotifierProvider.autoDispose<NewCalenderBookNotifier, NewEntityNotifierState>((ref) {
  return NewCalenderBookNotifier();
});
