import 'package:cashmere_core/new_entity_calls/view_to_request.dart';
import 'package:cashmere_core/protocols/book.pb.dart';
import 'package:cashmere_core/protocols/calendar.pb.dart';


class AddCalendarView implements ViewToRequest<AddCalendarRequest> {
  final String bookId;
  final Calendar calendar;
  final String description;

  AddCalendarView({
    required this.bookId,
    required this.calendar,
    required this.description,
  });

  @override
  AddCalendarRequest toRequest() {
    return AddCalendarRequest(
      bookId: bookId,
      calendar: calendar,
      description: description,
    );
  }
}
