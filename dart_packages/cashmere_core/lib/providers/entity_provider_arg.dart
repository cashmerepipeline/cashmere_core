import 'package:cashmere_core/cache/entities_cache.dart';

class EntityProviderArg {
  final String manageId;
  final String oid;
  final EntitiesFetchCallsArg fetchCalls;

  EntityProviderArg(
    this.manageId,
    this.oid,
    this.fetchCalls,
  );
}
