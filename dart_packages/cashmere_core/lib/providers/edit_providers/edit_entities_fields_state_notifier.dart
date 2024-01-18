import 'package:account_module/providers/meta_data_provider.dart';
import 'package:cashmere_core/grpc_call.dart';
import 'package:cashmere_core/protocols/entity.pb.dart';
import 'package:grpc/grpc.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

import 'edit_entity_notifier_state.dart';
import 'edit_entity_notifier_state_enum.dart';

class EditEntitiesFieldsNotifier extends StateNotifier<EditEntityNotifierState> {
  EditEntitiesFieldsNotifier()
      : super(
          EditEntityNotifierState("", EditEntityNotifierStateEnum.preparing),
        );

  Future<void> editEntitiesFields(
    List<EntityFieldEdit> edits,
    GrpcCall<EditMultiEntityFieldsRequest, EditMultiEntityFieldsResponse> stubCall,
    WidgetRef ref,
  ) async {
    state = EditEntityNotifierState("", EditEntityNotifierStateEnum.editing);
    final metaData = await ref.watch(metaDataFutureProvider.future);

    try {
    final request = EditMultiEntityFieldsRequest(
      edits: edits,
    );
    final response = await stubCall(
      request,
      options: CallOptions(metadata: metaData),
    );

    state = EditEntityNotifierState(response.result, EditEntityNotifierStateEnum.success);
    } catch (e) {
      state = EditEntityNotifierState(e.toString(), EditEntityNotifierStateEnum.error);
    }
  }
}
