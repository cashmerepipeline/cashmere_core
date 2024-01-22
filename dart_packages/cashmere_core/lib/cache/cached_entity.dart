import 'package:bson/bson.dart';
import 'package:isar/isar.dart';

part 'cached_entity.g.dart';

@collection
class CachedEntity {
  @id
  final String oid;
  final String nid;
  List<byte>? data;
  // millisceconds utc
  final DateTime lastModified;
  // final Timestamp lastModified;
  // millisceconds utc
  final DateTime lastChecked;

  CachedEntity({
    required this.oid,
    required this.nid,
    required this.data,
    required this.lastModified,
    required this.lastChecked,
  });
}
