import 'package:account_module/providers/meta_data_provider.dart';
import 'package:cashmere_core/grpc_call.dart';
import 'package:cashmere_core/protocols/name.pb.dart';
import 'package:grpc/grpc.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

import 'package:cashmere_core/providers/edit_providers/edit_entity_notifier_state.dart';
import 'package:cashmere_core/providers/edit_providers/edit_entity_notifier_state_enum.dart';

class RenameNotifier extends StateNotifier<EditEntityNotifierState> {
  RenameNotifier()
      : super(
          EditEntityNotifierState("", EditEntityNotifierStateEnum.preparing),
        );

  Future<void> rename(
    String manageId,
    String entityId,
    Name newName,
    GrpcCall<RenameRequest, RenameResponse> renameCall,
    WidgetRef ref,
    // EntitiesCacheArg cacheArg,
  ) async {
    state = EditEntityNotifierState("", EditEntityNotifierStateEnum.editing);
    final metaData = await ref.watch(metaDataFutureProvider.future);
    // final cache = await ref.watch(cacheProvider(cacheArg).future);

    final request = RenameRequest(
      manageId: manageId,
      entityId: entityId,
      newName: newName,
    );

    try {
      final response = await renameCall(
        request,
        options: CallOptions(metadata: metaData),
      );

      // zh: 更新缓存
      // cache.updateEntity(entityId);

      state = EditEntityNotifierState(response.result, EditEntityNotifierStateEnum.success);
    } catch (e) {
      state = EditEntityNotifierState(e.toString(), EditEntityNotifierStateEnum.error);
    }
  }
}

final renameNotifierProvider =
    StateNotifierProvider.autoDispose<RenameNotifier, EditEntityNotifierState>((ref) => RenameNotifier());
