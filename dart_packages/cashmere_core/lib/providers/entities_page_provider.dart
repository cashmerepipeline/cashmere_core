import 'package:bson/bson.dart';
import 'package:cashmere_core/cache/cache_provider.dart';
import 'package:cashmere_core/cache/cached_entity.dart';
import 'package:cashmere_core/cache/entities_cache.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:isar/isar.dart';

class PageProviderArg {
  final int pageIndex;
  final EntitiesCacheArg fetchCallArg;

  PageProviderArg({required this.pageIndex, required this.fetchCallArg});
}

class QueryArg {}

final entitiesPageProvider =
    StreamProvider.autoDispose.family<List<Map<String, dynamic>?>, PageProviderArg>((ref, arg) async* {
  final cache = await ref.watch(cacheProvider(arg.fetchCallArg).future);

  cache.refreshCache();
  yield* cache.isar.cachedEntitys
      .where()
      .sortByLastModifiedDesc()
      .watch(fireImmediately: true, limit: 20, offset: (arg.pageIndex - 1) * 20)
      .map((event) =>
          event.map((e) => e.data != null ? BsonCodec.deserialize(BsonBinary.from(e.data!)) : null).toList());
});
