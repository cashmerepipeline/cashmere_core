import 'package:general_grpc_interface/core/name_map_utils.dart';

import 'package:general_grpc_interface/grpc_generated/manage.pb.dart'
    as m_message;

/// 管理定义
class Manage {
  final String manageId;
  final Map<String, String> nameMap;

  Manage(this.manageId, this.nameMap);

  factory Manage.fromMessage(m_message.Manage message) {
    final nameMap = nameMapFromBytes(message.nameMap);

    return Manage(message.manageId.toString(), nameMap);
  }
}
