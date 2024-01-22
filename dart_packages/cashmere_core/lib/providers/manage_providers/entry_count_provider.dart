// ignore: depend_on_referenced_packages
import 'package:account_module/providers/meta_data_provider.dart';
import 'package:cashmere_core/fetchers/fetch_entry_count.dart';
import 'package:cashmere_core/grpc_call.dart';
import 'package:cashmere_core/protocols/manage.pb.dart';
import 'package:fixnum/fixnum.dart';
import 'package:flutter/foundation.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

class EntryCountProviderArgs {
  final String manageId;
  final GrpcCall<GetManageEntryCountRequest, GetManageEntryCountResponse> stubCall;

  EntryCountProviderArgs({required this.manageId, required this.stubCall});
}

final entryCountFutureProvider = FutureProvider.autoDispose.family<int, EntryCountProviderArgs>((ref, args) async {
  final metaData = await ref.watch(metaDataFutureProvider.future);

  final count = await fetchEntryCount(args.manageId, args.stubCall, metaData);
  debugPrint('entry count: $count');

  return count.toInt();
});
