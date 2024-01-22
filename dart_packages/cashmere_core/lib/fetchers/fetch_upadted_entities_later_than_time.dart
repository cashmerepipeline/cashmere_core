import 'dart:async';

import 'package:bson/bson.dart';
import 'package:cashmere_core/grpc_call.dart';
import 'package:cashmere_core/protocols/entity.pb.dart';
import 'package:grpc/grpc.dart';

class FetchLaterThanTimeArg {
  final String manageId;
  final Timestamp timestamp;
  final ResponseStreamGrpcCall<CheckUpdatesLaterThenTimeRequest, CheckUpdatesLaterThenTimeResponse> fetchCall;
  final Map<String, String> metadata;

  FetchLaterThanTimeArg({
    required this.manageId,
    required this.timestamp,
    required this.fetchCall,
    required this.metadata,
  });
}

Stream<List<Map<String, dynamic>>> fetchEntitiesLaterThanTime(FetchLaterThanTimeArg arg) async* {
  final t = BsonCodec.serialize({"value": arg.timestamp}).byteList;

  final request = CheckUpdatesLaterThenTimeRequest(
    manageId: arg.manageId,
    timestamp: t,
  );

  final response = arg.fetchCall(request, options: CallOptions(metadata: arg.metadata));
  await for (final res in response) {
    yield res.entityIds.map((e) => BsonCodec.deserialize(BsonBinary.from(e))).toList();
  }
}
