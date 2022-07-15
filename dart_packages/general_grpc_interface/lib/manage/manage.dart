import 'dart:convert';
import 'package:bson/bson.dart';

import 'package:general_grpc_interface/grpc_generated/manage.pb.dart' as mMessage;

import 'package:general_grpc_interface/name_map_utils.dart';
/// 管理定义
class Manage {
  final String manageId;
  final Map<String, String> nameMap;

  Manage(this.manageId, this.nameMap);

  factory Manage.fromMessage(mMessage.Manage message) {
    final nameMap = nameMapFromBytes(message.nameMap);

    return Manage(message.manageId, nameMap);
  }
}
