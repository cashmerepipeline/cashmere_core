import 'package:general_grpc_interface/grpc_generated/account_service.pbgrpc.dart';
import 'package:grpc/grpc.dart';

class CashmereClient<T> {
  final String host;
  final int port;

  ClientChannel get _channel => ClientChannel(
        host,
        port: port,
        options:
            const ChannelOptions(credentials: ChannelCredentials.insecure()),
      );

  // 账户连接
  AccountGrpcClient get accountStub => AccountGrpcClient(_channel);

  // 客户端连接，需要被覆写
  T getClientStub() => Client(_channel) as T;

  CashmereClient(this.host, this.port);

  void dispose() {
    _channel.shutdown();
  }
}
