import 'package:account_module/providers/meta_data_provider.dart';
import 'package:bson/bson.dart';
import 'package:cashmere_core/grpc_call.dart';
import 'package:cashmere_core/protocols/calendar_book.pb.dart';
import 'package:grpc/grpc.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

class ListCalendarBooksArg {
  final String manageId;
  final String entityId;
  final GrpcCall<ListCalendarBooksRequest, ListCalendarBooksResponse> grpcCall;

  ListCalendarBooksArg({
    required this.manageId,
    required this.entityId,
    required this.grpcCall,
  });
}

final calendarBooksFutureProvider =
    FutureProvider.autoDispose.family<List<Map<String, dynamic>>, ListCalendarBooksArg>((ref, arg) async {
  final metaData = await ref.watch(metaDataFutureProvider.future);
  final request = ListCalendarBooksRequest()
    ..manageId = arg.manageId
    ..entityId = arg.entityId;
  final response = await arg.grpcCall(request, options: CallOptions(metadata: metaData));

  return response.books.map((e) => BsonCodec.deserialize(BsonBinary.from(e))).toList();
});
