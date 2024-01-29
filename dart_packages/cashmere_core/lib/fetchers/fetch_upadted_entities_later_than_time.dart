import 'dart:async';

import 'package:bson/bson.dart';
import 'package:grpc/grpc.dart';
import 'package:flutter/foundation.dart';
import 'package:cashmere_core/grpc_call.dart';
import 'package:cashmere_core/protocols/entity.pb.dart';

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
  debugPrint('Fetching entities later than time: ${arg.manageId}, ${arg.timestamp}');
  final t = BsonCodec.serialize({"value": arg.timestamp}).byteList;

  final request = CheckUpdatesLaterThenTimeRequest(
    manageId: arg.manageId,
    timestamp: t,
  );

  final response = arg.fetchCall(request, options: CallOptions(metadata: arg.metadata));
  await for (final res in response) {
    final resMaps = res.results.map((e) => BsonCodec.deserialize(BsonBinary.from(e))).toList();

    debugPrint('Received response: $resMaps');

    yield resMaps;
  }
}
