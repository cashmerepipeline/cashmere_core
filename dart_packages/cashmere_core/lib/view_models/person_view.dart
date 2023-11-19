// @startuml
// Alice -> Bob: Authentication Request
// Bob --> Alice: Authentication Response
//
// Alice -> Bob: Another authentication Request
// Alice <-- Bob: Another authentication Response
// @enduml

import 'package:cashmere_core/ids/field_ids.dart';
import 'package:cashmere_core/ids/general_field_ids.dart';
import 'package:cashmere_core/view_models/entity_view_base.dart';
import 'package:cashmere_core/view_models/set_general_fields.dart';

class PersonView extends EntityViewBase {
  final String gender;
  final DateTime birthDay;
  final List<int> portrait;

  PersonView(
    String id, {
    required this.gender,
    required this.birthDay,
    required this.portrait,
  }) : super(id);

  factory PersonView.fromMap(Map<String, dynamic> map) {
    final entity = PersonView(
      map[ID_FIELD_ID.toString()],
      gender: map[PERSONS_GENDER_FIELD_ID.toString()],
      birthDay: DateTime.parse(map[PERSONS_BIRTHDAY_FIELD_ID.toString()]),
      portrait: map[PERSONS_PORTRAIT_FIELD_ID],
    );
    final result = setGeneralFields(entity, map);

    return result;
  }
}
