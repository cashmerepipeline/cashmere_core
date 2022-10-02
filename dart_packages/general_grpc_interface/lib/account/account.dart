import 'package:general_grpc_interface/entity/entity_base.dart';
import 'package:general_grpc_interface/grpc_generated/status.pbenum.dart';

/// 账号
class Account with EntityBase {
  String? areaCode;
  String? phone;
  String? password;
  String? email;
  LoginStatus? status;
  String? verificationCode;
  String? jwtToken;
  String? currentRoleGroup;

  Account();
}
