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

class NewEntityNotifier<V, Req, Res> extends StateNotifier<NewEntityNotifierState> {
  NewEntityNotifier()
      : super(
          NewEntityNotifierState(NewEntityNotifierStateEnum.preparing),
        );

  Future<void> newEntity(ViewToRequest newEntityView, GrpcCall<Req, Res> stubcall, WidgetRef ref) async {
    state = NewEntityNotifierState(NewEntityNotifierStateEnum.creating);
    final metaData = await ref.watch(metaDataFutureProvider.future);

    try {
      final result = await newEntityCall(newEntityView, stubcall, metaData);
      final newState = NewEntityNotifierState(NewEntityNotifierStateEnum.success);
      newState.result = result;
      state = newState;
    } catch (e) {
      final errState = NewEntityNotifierState(NewEntityNotifierStateEnum.error);
      state = errState;
    }
  }
}
