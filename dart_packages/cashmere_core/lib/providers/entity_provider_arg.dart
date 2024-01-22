import 'package:cashmere_core/cache/entities_cache.dart';

class EntityProviderArg {
  final String manageId;
  final String oid;
  final EntitiesCacheArg fetchCalls;

  EntityProviderArg({
    required this.manageId,
    required this.oid,
    required this.fetchCalls,
  });
}
