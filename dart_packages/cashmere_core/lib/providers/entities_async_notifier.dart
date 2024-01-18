import 'dart:async';

import 'package:account_module/protocols/account_status.pb.dart';
import 'package:account_module/providers/account_provider.dart';
import 'package:account_module/providers/meta_data_provider.dart';
import 'package:bson/bson.dart';
import 'package:cashmere_core/fetchers/fetch_entities_page.dart';
import 'package:cashmere_core/ids/general_field_ids.dart';
import 'package:flutter/foundation.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

import 'entities_page_notifier_args.dart';

class EntitiesPageAsyncNotifier
    extends AutoDisposeFamilyAsyncNotifier<List<Map<String, dynamic>>, EntitiesPageNotifierArgs> {
  List<int> fetchedPages = <int>[];

  EntitiesPageAsyncNotifier() : super();

  void refresh(ref) async {
    state = AsyncValue.data([]);
    state = AsyncValue.loading();
    final pages = <int>[];
    pages.addAll(fetchedPages);
    fetchedPages.clear();
    for (int pageIndex in pages) {
      fetchMore(ref, pageIndex);
    }
  }

  Map<String, dynamic>? getEntity(String entityId) {
    return state.value?.firstWhere((element) => element[ID_FIELD_ID.toString()] == entityId);
  }

  void fetchMore(ref, int pageIndex) async {
    debugPrint("fetchMore $pageIndex");
    if (fetchedPages.contains(pageIndex)) {
      return;
    }

    fetchedPages.add(pageIndex);
    debugPrint("objectID ${state.value?.last["_id"]}");

    final startOid = state.value?.last["_id"]! as ObjectId;
    debugPrint("fetchMore $pageIndex ${startOid.oid}");

    final metaData = await ref.watch(metaDataFutureProvider.future);
    try {
      final stream = await fetchEntitiesPage(arg.manageId, 0, startOid.oid, {"_id": -1}, arg.stubCall, metaData);
      stream.listen((entity) {
        final newEntities = state.value ?? [];
        newEntities.add(entity);
        state = AsyncValue.data(newEntities);
      });
    } catch (err) {
      debugPrint("fetchMore error: $err");
    }
  }

  @override
  FutureOr<List<Map<String, dynamic>>> build(EntitiesPageNotifierArgs arg) async {
    // TODO： 判断是否需要登录才能访问
    final account = await ref.watch(accountAsyncNotifierProvider.future);
    if (account.status != LoginStatus.LoggedIn) {
      return Future.error("请先登录");
    }
    final metaData = await ref.watch(metaDataFutureProvider.future);

    try {
      final stream = await fetchEntitiesPage(arg.manageId, 0, "", {"_id": -1}, arg.stubCall, metaData);
      fetchedPages.add(0);
      stream.listen((entity) {
        final newEntities = state.value ?? [];
        newEntities.add(entity);
        state = AsyncValue.data(newEntities);
      });
      return [];
    } catch (err) {
      return [];
    }
  }
}

final entitiesPageNotifierProvider = AsyncNotifierProvider.autoDispose
    .family<EntitiesPageAsyncNotifier, List<Map<String, dynamic>>, EntitiesPageNotifierArgs>(
        () => EntitiesPageAsyncNotifier());
