import 'package:cashmere_core/cache/cache_provider.dart';
import 'package:cashmere_core/cache/entities_cache.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

class PageProviderArg {
  final String manageId;
  final int pageIndex;
  final EntitiesFetchCallsArg fetchCallArg;

  PageProviderArg({required this.manageId, required this.pageIndex, required this.fetchCallArg});
}

final entitiesPageProvider =
    StreamProvider.autoDispose.family<List<Map<String, dynamic>>, PageProviderArg>((ref, arg) async* {

  final cache = await ref.watch(cacheProvider(arg.fetchCallArg).future);

  yield* cache
      .watchPage(manageId: arg.manageId, pageIndex: arg.pageIndex)
      .map((event) => event.map((e) => e.data).toList());
});
