///
//  Generated code. Do not modify.
//  source: account_service.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields

import 'dart:async' as $async;

import 'dart:core' as $core;

import 'package:grpc/service_api.dart' as $grpc;
import 'login.pb.dart' as $0;
import 'account.pb.dart' as $1;
export 'account_service.pb.dart';

class AccountGrpcClient extends $grpc.Client {
  static final _$login = $grpc.ClientMethod<$0.LoginRequest, $0.LoginResponse>(
      '/account_service.AccountGrpc/Login',
      ($0.LoginRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $0.LoginResponse.fromBuffer(value));
  static final _$newAccount =
      $grpc.ClientMethod<$1.NewAccountRequest, $1.NewAccountResponse>(
          '/account_service.AccountGrpc/NewAccount',
          ($1.NewAccountRequest value) => value.writeToBuffer(),
          ($core.List<$core.int> value) =>
              $1.NewAccountResponse.fromBuffer(value));
  static final _$addAccountIntoGroup = $grpc.ClientMethod<
          $1.AddAccountIntoGroupRequest, $1.AddAccountIntoGroupResponse>(
      '/account_service.AccountGrpc/AddAccountIntoGroup',
      ($1.AddAccountIntoGroupRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) =>
          $1.AddAccountIntoGroupResponse.fromBuffer(value));

  AccountGrpcClient($grpc.ClientChannel channel,
      {$grpc.CallOptions? options,
      $core.Iterable<$grpc.ClientInterceptor>? interceptors})
      : super(channel, options: options, interceptors: interceptors);

  $grpc.ResponseFuture<$0.LoginResponse> login($0.LoginRequest request,
      {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$login, request, options: options);
  }

  $grpc.ResponseFuture<$1.NewAccountResponse> newAccount(
      $1.NewAccountRequest request,
      {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$newAccount, request, options: options);
  }

  $grpc.ResponseFuture<$1.AddAccountIntoGroupResponse> addAccountIntoGroup(
      $1.AddAccountIntoGroupRequest request,
      {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$addAccountIntoGroup, request, options: options);
  }
}

abstract class AccountGrpcServiceBase extends $grpc.Service {
  $core.String get $name => 'account_service.AccountGrpc';

  AccountGrpcServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.LoginRequest, $0.LoginResponse>(
        'Login',
        login_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.LoginRequest.fromBuffer(value),
        ($0.LoginResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$1.NewAccountRequest, $1.NewAccountResponse>(
        'NewAccount',
        newAccount_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $1.NewAccountRequest.fromBuffer(value),
        ($1.NewAccountResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$1.AddAccountIntoGroupRequest,
            $1.AddAccountIntoGroupResponse>(
        'AddAccountIntoGroup',
        addAccountIntoGroup_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $1.AddAccountIntoGroupRequest.fromBuffer(value),
        ($1.AddAccountIntoGroupResponse value) => value.writeToBuffer()));
  }

  $async.Future<$0.LoginResponse> login_Pre(
      $grpc.ServiceCall call, $async.Future<$0.LoginRequest> request) async {
    return login(call, await request);
  }

  $async.Future<$1.NewAccountResponse> newAccount_Pre($grpc.ServiceCall call,
      $async.Future<$1.NewAccountRequest> request) async {
    return newAccount(call, await request);
  }

  $async.Future<$1.AddAccountIntoGroupResponse> addAccountIntoGroup_Pre(
      $grpc.ServiceCall call,
      $async.Future<$1.AddAccountIntoGroupRequest> request) async {
    return addAccountIntoGroup(call, await request);
  }

  $async.Future<$0.LoginResponse> login(
      $grpc.ServiceCall call, $0.LoginRequest request);
  $async.Future<$1.NewAccountResponse> newAccount(
      $grpc.ServiceCall call, $1.NewAccountRequest request);
  $async.Future<$1.AddAccountIntoGroupResponse> addAccountIntoGroup(
      $grpc.ServiceCall call, $1.AddAccountIntoGroupRequest request);
}
