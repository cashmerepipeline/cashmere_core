import 'package:cashmere_core/ids/field_ids.dart';
import 'package:cashmere_core/ids/general_field_ids.dart';
import 'package:cashmere_core/view_models/entity_view_base.dart';
import 'package:cashmere_core/view_models/set_general_fields.dart';

class LanguageCodeView extends EntityViewBase {
  final String code;
  final String native;

  LanguageCodeView(
    String id, {
    required this.code,
    required this.native,
  }) : super(id);

  factory LanguageCodeView.fromMap(Map<String, dynamic> map) {
    final entity = LanguageCodeView(
      map[ID_FIELD_ID.toString()],
      code: map[COUNTRY_CODES_CODE_FIELD_ID.toString()],
      native: map[COUNTRY_CODES_NATIVE_FIELD_ID.toString()],
    );
    final result = setGeneralFields(entity, map);

    return result;
  }
}
