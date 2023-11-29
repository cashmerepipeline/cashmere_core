import 'dart:async';
import 'package:account_module/providers/account_provider.dart';
import 'package:account_module/providers/meta_data_provider.dart';
import 'package:cashmere_core/fetchers/fetch_entities_page.dart';
import 'package:cashmere_core/ids/general_field_ids.dart';
import 'package:cashmere_core/providers/page_index_state_provider.dart';

import 'package:account_module/protocols/account_status.pb.dart';
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

  void changePage(ref, int pageIndex) {
    ref.read(pageIndexStateProvider(arg.manageId).notifier).state = pageIndex;
    this.refresh(ref);
  }

  void fetchMore(ref, int pageIndex) async {
    if (fetchedPages.contains(pageIndex)) {
      return;
    }

    fetchedPages.add(pageIndex);

    final metaData = await ref.watch(metaDataFutureProvider.future);
    try {
      final entities = await fetchEntitiesPage(arg.manageId, pageIndex , arg.stubCall, metaData) ?? [];
      final newEntities = state.value ?? [];
      newEntities.addAll(entities);
      state = AsyncValue.data(newEntities);
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
      final entities = await fetchEntitiesPage(arg.manageId, 0, arg.stubCall, metaData);
      fetchedPages.add(0);
      return entities;
    } catch (err) {
      return [];
    }
  }
}

final entitiesPageNotifierProvider = AsyncNotifierProvider.autoDispose
    .family<EntitiesPageAsyncNotifier, List<Map<String, dynamic>>, EntitiesPageNotifierArgs>(
        () => EntitiesPageAsyncNotifier());
