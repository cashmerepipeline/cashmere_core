import 'package:account_module/providers/meta_data_provider.dart';
import 'package:bson/bson.dart';
import 'package:cashmere_core/grpc_call.dart';
import 'package:cashmere_core/protocols/calendar_book.pb.dart';
import 'package:grpc/grpc.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

class BookCalendarsArg {
  final String bookId;
  final GrpcCall<ListBookCalendarsRequest, ListBookCalendarsResponse> grpcCall;

  BookCalendarsArg({
    required this.bookId,
    required this.grpcCall,
  });
}

final bookCalendarsFutureProvider = FutureProvider.autoDispose.family<List<Map<String, dynamic>>, BookCalendarsArg>(
  (ref, arg) async {
    final metaData = await ref.watch(metaDataFutureProvider.future);
    final request = ListBookCalendarsRequest(bookId: arg.bookId);

    final response = await arg.grpcCall(request, options: CallOptions(metadata: metaData));
    return response.calendars.map((e) => BsonCodec.deserialize(BsonBinary.from(e))).toList();
  },
);
