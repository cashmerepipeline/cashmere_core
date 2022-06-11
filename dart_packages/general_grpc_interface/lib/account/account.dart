import 'package:general_grpc_interface/grpc_generated/status.pbenum.dart';

/// 账号
class Account {
  String? areaCode;
  String? phone;
  String? password;
  String? email;
  LoginStatus? status;
  String? verificationCode;
  String? jwtToken;

  Account();
}
