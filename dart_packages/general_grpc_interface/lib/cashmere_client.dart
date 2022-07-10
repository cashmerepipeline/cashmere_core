import 'package:general_grpc_interface/grpc_generated/account_service.pbgrpc.dart';
import 'package:grpc/grpc.dart';

typedef S ClientCreator<S extends Client>(ClientChannel channel);

class CashmereClient<T extends Client> {
  final String host;
  final int port;
  final ChannelOptions channelOptions;
  final ClientCreator<T> creator;
  T? clientStub;

  ClientChannel get _channel => ClientChannel(
        host,
        port: port,
        options: channelOptions,
      );

  // 账户连接
  AccountGrpcClient get accountStub => AccountGrpcClient(_channel);

  // 客户端连接，需要被覆写
  /* T getClientStub() => Client(_channel) as T; */
  T? getClientStub() {
    if (clientStub != null) {
      return clientStub;
    } else {
      clientStub = creator(_channel);
      return clientStub;
    }
  }

  CashmereClient(this.host, this.port, this.channelOptions, this.creator);

  void dispose() {
    _channel.shutdown();
  }
}
