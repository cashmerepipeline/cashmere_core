import 'package:cashmere_core/grpc_call.dart';
import 'package:cashmere_core/protocols/entity.pb.dart';

class FetchEntityArg {
  final String manageId;
  final String entityId;
  final GrpcCall<GetEntityRequest, GetEntitiesResponse> getEntityCall;
  final Map<String, String> metadata;

  var checkEntitiesUpdate;

  FetchEntityArg({
    required this.manageId,
    required this.entityId,
    required this.getEntityCall,
    required this.metadata,
  });
}
