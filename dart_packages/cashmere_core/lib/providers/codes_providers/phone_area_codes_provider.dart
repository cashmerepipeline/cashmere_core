import 'package:account_module/providers/meta_data_provider.dart';
import 'package:bson/bson.dart';
import 'package:cashmere_core/codes_fetchers/fetch_precoded_codes.dart';
import 'package:cashmere_core/grpc_call.dart';
import 'package:cashmere_core/protocols/phone_area_code.pb.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

final phoneAreaCodesFutureProvider =
    FutureProvider.family<List<Map<String, dynamic>>, GrpcCall<GetPhoneAreaCodesRequest, GetPhoneAreaCodesResponse>>(
        (ref, stubCall) async {
  final metaData = await ref.watch(metaDataFutureProvider.future);
  final request = GetPhoneAreaCodesRequest();
  final response =
      await fetchPrecodedCodes<GetPhoneAreaCodesRequest, GetPhoneAreaCodesResponse>(request, stubCall, metaData);

  final bson = BSON();
  final items = response.codes.map((e) => bson.deserialize(BsonBinary.from(e))).toList();

  return items;
});
