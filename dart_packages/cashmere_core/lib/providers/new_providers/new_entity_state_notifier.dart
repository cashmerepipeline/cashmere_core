import 'package:flutter/foundation.dart';
import 'package:account_module/providers/meta_data_provider.dart';
import 'package:cashmere_core/grpc_call.dart';
import 'package:cashmere_core/new_entity_calls/new_entity_call.dart';
import 'package:cashmere_core/new_entity_calls/view_to_request.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

enum NewEntityNotifierStateEnum { preparing, creating, success, error }

class NewEntityNotifierState<Res> {
  late Res? result;
  final NewEntityNotifierStateEnum state;

  NewEntityNotifierState(this.state);
}

class NewEntityNotifier<V, Req, Res> extends StateNotifier<NewEntityNotifierState<Res>> {
  NewEntityNotifier()
      : super(
          NewEntityNotifierState(NewEntityNotifierStateEnum.preparing),
        );

  Future<Res> newEntity(ViewToRequest newEntityView, GrpcCall<Req, Res> stubcall, WidgetRef ref) async {
    state = NewEntityNotifierState<Res>(NewEntityNotifierStateEnum.creating);
    final metaData = await ref.watch(metaDataFutureProvider.future);

    try {
      final response = await newEntityCall(newEntityView, stubcall, metaData);

      final newState = NewEntityNotifierState<Res>(NewEntityNotifierStateEnum.success);
      newState.result = response;
      state = newState;
      return response;
    } catch (e) {
      debugPrint("$e.toString()");
      final errState = NewEntityNotifierState<Res>(NewEntityNotifierStateEnum.error);
      state = errState;
      return Future.error("新建实体错误: ${e.toString()}");
    }
  }
}
