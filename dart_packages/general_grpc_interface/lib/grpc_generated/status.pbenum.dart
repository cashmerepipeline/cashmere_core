///
//  Generated code. Do not modify.
//  source: status.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields

// ignore_for_file: UNDEFINED_SHOWN_NAME
import 'dart:core' as $core;
import 'package:protobuf/protobuf.dart' as $pb;

class LoginStatus extends $pb.ProtobufEnum {
  static const LoginStatus LoggedIn = LoginStatus._(0, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'LoggedIn');
  static const LoginStatus LoggedOut = LoginStatus._(1, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'LoggedOut');
  static const LoginStatus LogginFailed = LoginStatus._(2, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'LogginFailed');

  static const $core.List<LoginStatus> values = <LoginStatus> [
    LoggedIn,
    LoggedOut,
    LogginFailed,
  ];

  static final $core.Map<$core.int, LoginStatus> _byValue = $pb.ProtobufEnum.initByValue(values);
  static LoginStatus? valueOf($core.int value) => _byValue[value];

  const LoginStatus._($core.int v, $core.String n) : super(v, n);
}

class AccountStatus extends $pb.ProtobufEnum {
  static const AccountStatus Stopped = AccountStatus._(0, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'Stopped');
  static const AccountStatus Actived = AccountStatus._(1, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'Actived');

  static const $core.List<AccountStatus> values = <AccountStatus> [
    Stopped,
    Actived,
  ];

  static final $core.Map<$core.int, AccountStatus> _byValue = $pb.ProtobufEnum.initByValue(values);
  static AccountStatus? valueOf($core.int value) => _byValue[value];

  const AccountStatus._($core.int v, $core.String n) : super(v, n);
}

