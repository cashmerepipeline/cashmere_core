import 'package:cashmere_core/new_entity_calls/view_to_request.dart';
import 'package:fixnum/fixnum.dart';
import 'package:cashmere_core/protocols/gender.pbenum.dart';
import 'package:cashmere_core/protocols/name.pb.dart';
import 'package:cashmere_core/protocols/person.pb.dart';

class NewPersonVew extends ViewToRequest<NewPersonRequest> {
  final Name name;
  final Gender gender;
  final int birthday;
  final List<int> portrait;
  final String description;
  final String address;

  NewPersonVew({
    required this.name,
    required this.gender,
    required this.birthday,
    required this.portrait,
    required this.description,
    required this.address,
  });

  @override
  NewPersonRequest toRequest() {
    return NewPersonRequest(
      name: name,
      gender: gender,
      birthday: Int64(birthday),
      portrait: portrait,
      description: description,
      address: address,
    );
  }
}
