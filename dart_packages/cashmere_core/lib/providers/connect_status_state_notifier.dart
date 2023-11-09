import 'package:cashmere_core/providers/status_enums/connect_status_enum.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

class ConnectStatusStateNotifier extends StateNotifier<ConnectStatusEnum> {
  ConnectStatusStateNotifier() : super(ConnectStatusEnum.disconnected);

  void update(ConnectStatusEnum status) {
    state = status;
  }
}
