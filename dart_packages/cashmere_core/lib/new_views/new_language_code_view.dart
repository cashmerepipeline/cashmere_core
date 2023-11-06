import 'package:cashmere_core/new_entity_calls/view_to_request.dart';
import 'package:cashmere_core/protocols/language_code.pb.dart';
import 'package:cashmere_core/protocols/name.pb.dart';

class NewLanguageCodeView extends ViewToRequest<NewLanguageCodeRequest> {
  final Name name;
  final String code;
  final String nativeName;

  NewLanguageCodeView({
    required this.name,
    required this.code,
    required this.nativeName,
  });

  @override
  NewLanguageCodeRequest toRequest() {
    return NewLanguageCodeRequest(
      name: name,
      code: code,
      nativeName: nativeName,
    );
  }
}
