import 'package:cashmere_core/new_entity_calls/view_to_request.dart';
import 'package:cashmere_core/protocols/country_code.pb.dart';
import 'package:cashmere_core/protocols/name.pb.dart';

class NewCountryCodeView extends ViewToRequest<NewCountryCodeRequest> {
  final Name name;
  final String code;
  final String native;
  final String phoneAreaCode;
  final List<String> languages;

  NewCountryCodeView(
      {required this.name,
      required this.code,
      required this.native,
      required this.phoneAreaCode,
      required this.languages});

  @override
  NewCountryCodeRequest toRequest() {
    return NewCountryCodeRequest(
      name: name,
      code: code,
      native: native,
      phoneAreaCode: phoneAreaCode,
      languages: languages,
    );
  }
}
