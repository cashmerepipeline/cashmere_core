import 'package:grpc/grpc.dart';

typedef GrpcCall<Req, Res> = ResponseFuture<Res> Function(
  Req request, {
  CallOptions options,
});

typedef StreamGrpcCall<Req, Res> = ResponseStream<Res> Function(
  Stream<Req> request, {
  CallOptions options,
});
