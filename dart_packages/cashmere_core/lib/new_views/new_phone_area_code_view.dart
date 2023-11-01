import 'package:cashmere_core/protocols/name.pb.dart';

class NewPhoneAreaCodeView {
  final Name name;
  final String code;
  final List<String> areas;

  NewPhoneAreaCodeView({
    required this.name,
    required this.code,
    required this.areas,
  });
}
