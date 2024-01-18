import 'package:cashmere_core/grpc_call.dart';
import 'package:cashmere_core/protocols/entity.pb.dart';

class EntitiesPageNotifierArgs {
  final String manageId;
  final ResponseStreamGrpcCall<GetEntitiesPageRequest, GetEntitiesPageResponse> stubCall;

  final GrpcCall<CheckEntitiesUpdateRequest, CheckEntitiesUpdateResponse> checkEntitiesUpdate;

  EntitiesPageNotifierArgs({
    required this.manageId,
    required this.stubCall,
    required this.checkEntitiesUpdate,
  });
}
