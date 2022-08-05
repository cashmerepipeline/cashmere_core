import 'package:general_grpc_interface/entity/entity_base.dart';

class AreaCode with EntityBase {
  final String name;
  final String code;

  AreaCode({required this.name, required this.code});
}
