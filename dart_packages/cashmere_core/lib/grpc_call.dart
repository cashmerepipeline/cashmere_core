import 'package:grpc/grpc.dart';

typedef GrpcCall<Req, Res> = Future<Res> Function(
  Req request, {
  CallOptions options,
});
