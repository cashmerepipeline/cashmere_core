import 'package:cashmere_core/protocols/name.pb.dart';

class NewPersonVew {
  final Name name;
  final String gender;
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
}
