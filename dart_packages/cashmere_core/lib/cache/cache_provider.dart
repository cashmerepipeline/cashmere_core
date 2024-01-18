import 'package:account_module/providers/meta_data_provider.dart';
import 'package:cashmere_core/cache/entities_cache.dart';
import 'package:cashmere_core/cache/isar_provider.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

final cacheProvider = FutureProvider.family<EntitiesCache, EntitiesFetchCallsArg>((ref, arg) async {
  final isar = await ref.watch(isarProvider.future);

  return EntitiesCache(isar: isar, args: arg);
});
