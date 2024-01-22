import 'package:bson/bson.dart';
import 'package:cashmere_core/cache/cache_provider.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

import 'package:cashmere_core/providers/entity_provider_arg.dart';

final entityProvider = StreamProvider.autoDispose.family<Map<String, dynamic>?, EntityProviderArg>((ref, arg) async* {
  final cache = await ref.read(cacheProvider(arg.fetchCalls).future);
  yield* cache
      .watchEntity(entityOid: arg.oid)
      .map((event) => event.data != null ? BsonCodec.deserialize(BsonBinary.from(event.data!)) : null);
});
