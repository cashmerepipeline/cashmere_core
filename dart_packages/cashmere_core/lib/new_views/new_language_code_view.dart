import 'package:cashmere_core/protocols/name.pb.dart';

class NewLanguageCodeView {
  final Name name;
  final String code;
  final String native_name;

  NewLanguageCodeView({required this.name, required this.code, required this.native_name});
}
