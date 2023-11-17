import 'package:cashmere_core/core/name_map_from_bytes.dart';

import 'package:cashmere_core/protocols/manage.pb.dart' as m_message;

/// 管理定义
class Manage {
  final int manageId;
  final Map<String, String> nameMap;

  Manage(this.manageId, this.nameMap);

  factory Manage.fromMessage(m_message.Manage message) {
    final nameMap = nameMapFromBytes(message.nameMap);

    return Manage(message.manageId, nameMap);
  }
}
