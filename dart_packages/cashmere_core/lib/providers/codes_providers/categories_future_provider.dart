import 'package:account_module/providers/meta_data_provider.dart';
import 'package:cashmere_core/fetchers/fetch_categories.dart';
import 'package:cashmere_core/grpc_call.dart';
import 'package:cashmere_core/protocols/category.pb.dart';
import 'package:flutter/foundation.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

class CategoriesProviderArg {
  final GrpcCall<GetCategoriesRequest, GetCategoriesResponse> stubCall;
  final int manageId;

  CategoriesProviderArg({required this.stubCall, required this.manageId});
}

final categoriesFutureProvider =
    FutureProvider.family<List<Map<String, dynamic>>, CategoriesProviderArg>((ref, arg) async {
  final metaData = await ref.watch(metaDataFutureProvider.future);
  final categories = await fetchCategories(arg.stubCall, metaData, arg.manageId);
  debugPrint("$categories");

  return categories;
});
