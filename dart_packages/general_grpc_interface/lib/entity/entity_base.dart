import 'package:fixnum/fixnum.dart';

mixin EntityBase {
  late final String id;
  late final Map<String, String> nameMap;
  late final String creator;
  late final Int64 createTimestamp;
  late final String owner;
  late final List<String> groups;
  late final List<String> datas;
  late final List<String> removedDatas;
  late final List<String> comments;
}
