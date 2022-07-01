import 'package:fixnum/fixnum.dart';
import 'package:general_grpc_interface/core/name.dart';

mixin EntityBase {
  late final String id;
  late final List<Name> name;
  late final String creator;
  late final Int64 createTimestamp;
  late final String owner;
  late final List<String> groups;
  late final List<String> datas;
  late final List<String> removed_datas;
  late final List<String> comments;
}
