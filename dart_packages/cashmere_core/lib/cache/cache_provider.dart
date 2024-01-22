import 'package:account_module/providers/meta_data_provider.dart';
import 'package:cashmere_core/cache/entities_cache.dart';
import 'package:cashmere_core/cache/isar_provider.dart';
import 'package:flutter/foundation.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

final cacheProvider = FutureProvider.autoDispose.family<EntitiesCache, EntitiesCacheArg>((ref, arg) async {
  final isar = await ref.watch(isarProvider(arg.manageId).future);
  final metadata = await ref.watch(metaDataFutureProvider.future);
  debugPrint('EntitiesCache isar: $isar');

  return EntitiesCache(manageId: arg.manageId, metadata: metadata, isar: isar, args: arg);
});
