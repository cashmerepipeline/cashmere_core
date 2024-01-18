
import 'package:cashmere_core/cache/cached_entity.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:isar/isar.dart';
import 'package:path_provider/path_provider.dart';

final isarProvider = FutureProvider((ref) async {
  final dir = await getApplicationCacheDirectory();
  return Isar.open(schemas: [CachedEntitySchema], directory: dir.path+"/isar");
});