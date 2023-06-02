import 'package:grpc/grpc.dart';

// dart不支持直接调用范型类生成实例
typedef ClientCreator<S extends Client> = S Function(ClientChannel channel);

class CashmereClient<T extends Client, U extends Client> {
  final String host;
  final int port;
  final ChannelOptions channelOptions;
  final ClientCreator<T> clientStubCreator;
  final ClientCreator<U> accountStubCreator;

  T? _clientStub;
  U? _accountStub;

  ClientChannel get _channel => ClientChannel(
        host,
        port: port,
        options: channelOptions,
      );

  // 客户端连接，需要被覆写
  T get clientStub => _clientStub ?? clientStubCreator(_channel);

  // 帐号连接，需要被覆写
  U get accountStub => _accountStub ?? accountStubCreator(_channel);

  CashmereClient(
    this.host,
    this.port,
    this.channelOptions,
    this.clientStubCreator,
    this.accountStubCreator,
  );

  void dispose() {
    _channel.shutdown();
  }
}
