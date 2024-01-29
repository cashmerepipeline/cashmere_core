import 'package:cashmere_core/cache_schemas/cache_entity_from_map.dart';
import 'package:cashmere_core/grpc_call.dart';
import 'package:cashmere_core/protocols/entity.pb.dart';

class EntitiesCacheArg<Bt> {
  final String manageId;
  final GrpcCall<CheckEntitiesUpdateRequest, CheckEntitiesUpdateResponse> checkEntitiesUpdateCall;
  final ResponseStreamGrpcCall<CheckUpdatesLaterThenTimeRequest, CheckUpdatesLaterThenTimeResponse>
      checkUpdatesLaterThanTimeCall;
  final ResponseStreamGrpcCall<GetEntitiesRequest, GetEntitiesResponse> getEntitiesCall;
  final GrpcCall<GetEntityRequest, GetEntityResponse> getEntityCall;
  final CacheEntityFromMap<Bt> cacheEntityFromMap; 
  final GetLastModifiedTimestamp<Bt> getLastModifiedTimestamp; 

  EntitiesCacheArg({
    required this.manageId,
    required this.checkEntitiesUpdateCall,
    required this.checkUpdatesLaterThanTimeCall,
    required this.getEntitiesCall,
    required this.getEntityCall,
    required this.cacheEntityFromMap,
    required this.getLastModifiedTimestamp,
  });
}