import 'package:bson/bson.dart';
import 'package:isar/isar.dart';

part 'cached_entity.g.dart';

@collection
class CachedEntity {
  final String manageId;
  @id
  final String oid;
  final Map<String, dynamic> data;
  // millisceconds utc
  final int lastModified;
  // millisceconds utc
  final DateTime lastChecked;

  CachedEntity({
    required this.manageId,
    required this.oid,
    required this.data,
    required this.lastModified,
    required this.lastChecked,
  });
}
