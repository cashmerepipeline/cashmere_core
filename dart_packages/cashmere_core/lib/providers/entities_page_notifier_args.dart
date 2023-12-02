import 'package:cashmere_core/grpc_call.dart';
import 'package:cashmere_core/protocols/entity.pb.dart';

class EntitiesPageNotifierArgs {
  final int manageId;
  final ResponseStreamGrpcCall<GetEntitiesPageRequest, GetEntitiesPageResponse> stubCall;

  EntitiesPageNotifierArgs({required this.manageId, required this.stubCall});
}
