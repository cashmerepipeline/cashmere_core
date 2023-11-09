import 'package:account_module/providers/meta_data_provider.dart';
import 'package:cashmere_core/grpc_call.dart';
import 'package:grpc/grpc.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

import 'edit_entity_notifier_state.dart';
import 'edit_entity_notifier_state_enum.dart';

// 编辑单个实体的字段
class EditEntityFieldNotifier<Req, Res> extends StateNotifier<EditEntityNotifierState> {
  EditEntityFieldNotifier()
      : super(
          EditEntityNotifierState("", EditEntityNotifierStateEnum.preparing),
        );

  Future<void> editEntityField(
    Req request,
    GrpcCall<Req, Res> stubCall,
    WidgetRef ref,
  ) async {
    state = EditEntityNotifierState("", EditEntityNotifierStateEnum.editing);
    final metaData = await ref.watch(metaDataFutureProvider.future);

    try {
      final result = await stubCall(
        request,
        options: CallOptions(metadata: metaData),
      );
      state = EditEntityNotifierState(result, EditEntityNotifierStateEnum.success);
    } catch (e) {
      state = EditEntityNotifierState(e.toString(), EditEntityNotifierStateEnum.error);
    }
  }
}
