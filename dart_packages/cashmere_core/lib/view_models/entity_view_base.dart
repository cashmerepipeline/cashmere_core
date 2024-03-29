import 'package:bson/bson.dart';

typedef NameMap = Map<String, String>;

abstract class EntityViewBase {
  final String id;
  late final NameMap nameMap;
  late final String creator;
  late final Timestamp createTimestamp;
  late final String modifier;
  late final Timestamp modifyTimestamp;
  late final String owner;
  late final List<String> groups;
  late final List<String>? specs;
  late final List<String>? comments;
  late final bool removed;

  EntityViewBase(this.id);
}
