import 'package:cashmere_core/new_entity_calls/to_new_entity_request.dart';
import 'package:cashmere_core/protocols/name.pb.dart';
import 'package:cashmere_core/protocols/phone_area_code.pb.dart';

class NewPhoneAreaCodeView extends ViewToRequest<NewPhoneAreaCodeRequest> {
  final Name name;
  final String code;
  final List<String> areas;

  NewPhoneAreaCodeView({
    required this.name,
    required this.code,
    required this.areas,
  });

  @override
  NewPhoneAreaCodeRequest toRequest() {
    return NewPhoneAreaCodeRequest(
      name: name,
      code: code,
      areas: areas,
    );
  }
}
