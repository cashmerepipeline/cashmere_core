import 'package:cashmere_core/new_entity_calls/view_to_request.dart';
import 'package:cashmere_core/protocols/calendar.pb.dart';


class NewCalendarView implements ViewToRequest<NewCalendarRequest> {
  final String bookId;
  final Calendar calendar;
  final String description;

  NewCalendarView({
    required this.bookId,
    required this.calendar,
    required this.description,
  });

  @override
  NewCalendarRequest toRequest() {
    return NewCalendarRequest(
      bookId: bookId,
      calendar: calendar,
      description: description,
    );
  }
}
