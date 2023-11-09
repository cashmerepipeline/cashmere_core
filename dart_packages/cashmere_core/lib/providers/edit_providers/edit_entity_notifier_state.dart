

import 'package:cashmere_core/providers/edit_providers/edit_entity_notifier_state_enum.dart';

class EditEntityNotifierState<Res> {
  late Res? result;
  final EditEntityNotifierStateEnum state;

  EditEntityNotifierState(this.result, this.state);
}
