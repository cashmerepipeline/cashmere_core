import 'dart:async';

import 'package:account_module/providers/meta_data_provider.dart';
import 'package:cashmere_core/grpc_call.dart';
import 'package:cashmere_core/providers/device_id_provider.dart';
import 'package:fixnum/fixnum.dart';

import 'package:cashmere_core/protocols/ping.pb.dart';
import 'package:flutter/foundation.dart';
import 'package:grpc/grpc.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

import 'package:cashmere_core/providers/status_enums/connect_status_enum.dart';

final connectStatusProvider =
    StreamProvider.family<ConnectStatusEnum, BiStreamGrpcCall<PingRequest, PingResponse>>((ref, pingCall) async* {
  final metaData = ref.watch(metaDataFutureProvider);
  final deviceId = ref.watch(deviceIdProvider).value;

  final startRequest = PingRequest(
    index: Int64(0),
    deviceId: deviceId,
    time: Int64(0),
  );

  ConnectStatusEnum status = ConnectStatusEnum.connecting;

  // 断线重连
  while (status != ConnectStatusEnum.connected) {
    final pingStreamController = StreamController<PingRequest>();

    pingStreamController.add(startRequest);
    try {
      final pongStream = await pingCall(pingStreamController.stream, options: CallOptions(metadata: metaData.value));

      await for (PingResponse pong in pongStream) {
        // 直接返回
        final nextRequest = PingRequest(
          deviceId: deviceId,
          index: pong.index,
          time: pong.time,
        );
        pingStreamController.add(nextRequest);

        // 如果有更新
        if (status != ConnectStatusEnum.connected) {
          status = ConnectStatusEnum.connected;
          yield status;
        }

        // 切换帐号或者重新登陆
        final rMetaData = await ref.read(metaDataFutureProvider.future);
        if (metaData.value != rMetaData) {
          debugPrint("metaData is reloading");
          // pingStreamController.close();
          await pongStream.cancel();
          break;
        }
        // sleep 5 seconds
        // FIXME: 应该设置为低优先级
        // await Future.delayed(const Duration(seconds: 5));
      }
    } catch (e) {
      debugPrint("$e");
      status = ConnectStatusEnum.error;
      yield status;
    }

    pingStreamController.close();
    status = ConnectStatusEnum.disconnected;
    yield status;

    // 15秒重试
    await Future.delayed(const Duration(seconds: 15));
  }
});
