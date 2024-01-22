import 'package:cashmere_core/cache/entities_cache.dart';
import 'package:cashmere_core/grpc_call.dart';
import 'package:cashmere_core/protocols/entity.pb.dart';

class EntitiesPageNotifierArgs {
  final String manageId;
  final ResponseStreamGrpcCall<GetEntitiesPageRequest, GetEntitiesPageResponse> getEntitiesPageCall;

  final EntitiesCacheArg fetchCallsArg;

  EntitiesPageNotifierArgs({
    required this.manageId,
    required this.getEntitiesPageCall,
    required this.fetchCallsArg,
  });
}
