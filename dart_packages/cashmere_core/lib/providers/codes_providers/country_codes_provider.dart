import 'package:account_module/providers/meta_data_provider.dart';
import 'package:cashmere_core/fetchers/fetch_country_codes.dart';
import 'package:cashmere_core/grpc_call.dart';
import 'package:cashmere_core/protocols/country_code.pb.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

final countryCodesFutureProvider = FutureProvider.autoDispose
    .family<List<Map<String, dynamic>>, GrpcCall<GetCountryCodesRequest, GetCountryCodesResponse>>(
        (ref, stubCall) async {
  final metaData = await ref.watch(metaDataFutureProvider.future);
  final countries = await fetchCountryCodes(stubCall, metaData);
  return countries;
});
