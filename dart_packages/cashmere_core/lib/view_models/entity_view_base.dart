import 'package:bson/bson.dart';

abstract class EntityViewBase {
  final String id;
  late final Map<String, String> nameMap;
  late final String creator;
  late final DateTime createTimestamp;
  late final String modifier;
  late final DateTime modifyTimestamp;
  late final String owner;
  late final List<String> groups;
  late final List<String>? specs;
  late final List<String>? comments;
  late final List<String>? tags;
  late final String? description;
  late final bool removed;

  EntityViewBase(this.id);
}
