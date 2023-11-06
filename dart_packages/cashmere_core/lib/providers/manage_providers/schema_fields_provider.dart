import 'package:account_module/providers/meta_data_provider.dart';
import 'package:cashmere_core/codes_fetchers/fetch_schema_fields.dart';
import 'package:cashmere_core/grpc_call.dart';
import 'package:cashmere_core/protocols/manage_schema.pb.dart';
import 'package:cashmere_core/manage/schema_field.dart' as model_schema;
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:flutter/foundation.dart';

class SchemaFieldsNotifier extends StateNotifier<List<model_schema.SchemaField>> {
  final int manageId;
  SchemaFieldsNotifier(this.manageId) : super([]);
}

class SchemaFieldsProviderArgs {
  final int manageId;
  final GrpcCall<GetManageSchemaRequest, GetManageSchemaResponse> stubCall;

  SchemaFieldsProviderArgs({required this.manageId, required this.stubCall});
}

final schemaFieldsFutureProvider =
    FutureProvider.autoDispose.family<List<model_schema.SchemaField>, SchemaFieldsProviderArgs>((ref, args) async {
  final metaData = await ref.watch(metaDataFutureProvider.future);
  final fields = await fetchSchemaFields(args.manageId, args.stubCall, metaData);

  debugPrint("schema fields of : $fields");
  return fields;
});

final schemaFieldsStateProvider = StateNotifierProvider.autoDispose
    .family<SchemaFieldsNotifier, List<model_schema.SchemaField>, SchemaFieldsProviderArgs>((ref, args) {
  final fields = ref.watch(schemaFieldsFutureProvider(args));
  final notifier = SchemaFieldsNotifier(args.manageId);

  fields.when(data: (data) {
    notifier.state = data;
  }, error: (err, stack) {
    notifier.state = [];
  }, loading: () {
    notifier.state = [];
  });
  return notifier;
});
