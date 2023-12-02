import 'package:grpc/grpc.dart';

typedef GrpcCall<Req, Res> = ResponseFuture<Res> Function(
  Req request, {
  CallOptions options,
});

typedef ResponseStreamGrpcCall<Req, Res> = ResponseStream<Res> Function(
  Req request, {
  CallOptions options,
});

typedef BiStreamGrpcCall<Req, Res> = ResponseStream<Res> Function(
  Stream<Req> request, {
  CallOptions options,
});
