import 'package:cashmere_core/new_views/new_calendar_book_view.dart';
import 'package:cashmere_core/protocols/calendar_book.pb.dart';
import 'package:cashmere_core/providers/new_entity_state_notifier.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

final newCalendarBookNotifierProvider = StateNotifierProvider.autoDispose<
    NewEntityNotifier<NewCalendarBookView, NewCalendarBookRequest, NewCalendarBookResponse>, NewEntityNotifierState>((ref) {
  return NewEntityNotifier<NewCalendarBookView, NewCalendarBookRequest, NewCalendarBookResponse>(); });
