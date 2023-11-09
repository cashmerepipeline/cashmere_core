import 'package:account_module/providers/meta_data_provider.dart';
import 'package:bson/bson.dart';
import 'package:cashmere_core/fetchers/fetch_precoded_codes.dart';
import 'package:cashmere_core/grpc_call.dart';
import 'package:cashmere_core/protocols/language_code.pb.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

final countryCodesFutureProvider =
    FutureProvider.family<List<Map<String, dynamic>>, GrpcCall<GetLanguageCodesRequest, GetLanguageCodesResponse>>(
        (ref, stubCall) async {
  final metaData = await ref.watch(metaDataFutureProvider.future);
  final request = GetLanguageCodesRequest();
  final response =
      await fetchPrecodedCodes<GetLanguageCodesRequest, GetLanguageCodesResponse>(request, stubCall, metaData);

  final bson = BSON();
  final items = response.codes.map((e) => bson.deserialize(BsonBinary.from(e))).toList();

  return items;
});
