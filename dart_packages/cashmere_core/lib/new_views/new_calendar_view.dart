import 'package:cashmere_core/new_entity_calls/new_entity_call.dart';
import 'package:cashmere_core/protocols/calendar.pb.dart';
import 'package:cashmere_core/protocols/calendar.pbenum.dart';

class NewCalendarView implements ToNewEntityRequest<NewCalendarRequest> {
  final CalendarType type;
  // {"year"| "month"| "week"}
  final String every;
  // {"day": day, "hour": hour, "minute": minute}
  final Map<String, String> daytime;

  NewCalendarView({required this.type, required this.every, required this.daytime});
  
  @override
  NewCalendarRequest toRequest() {
    return NewCalendarRequest(
      type: type,
      every: every,
      daytime: daytime,
    )
  }
}
