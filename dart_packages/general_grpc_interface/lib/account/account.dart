import 'package:imixapp/grpc_generated/account.pbenum.dart';

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
