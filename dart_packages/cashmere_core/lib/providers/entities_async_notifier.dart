import 'dart:async';

import 'package:account_module/protocols/account_status.pb.dart';
import 'package:account_module/providers/account_provider.dart';
import 'package:cashmere_core/cache/cache_provider.dart';
import 'package:flutter/foundation.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

import 'entities_page_notifier_args.dart';

/// zh: 只存储实体的id
class EntitiesPageAsyncNotifier extends AutoDisposeFamilyAsyncNotifier<List<String>, EntitiesPageNotifierArgs> {
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

  void fetchMore(ref, int pageIndex) async {
    debugPrint("fetchMore $pageIndex");
    if (fetchedPages.contains(pageIndex)) {
      return;
    }

    fetchedPages.add(pageIndex);

    final cache = await ref.watch(cacheProvider(arg.fetchCallsArg).future);
    final stream = cache.fetchPage(pageIndex);
    stream.listen((id) {
      final ids = state.value ?? [];
      ids.add(id);
      state = AsyncValue.data(ids);
    });

    // debugPrint("objectID ${state.value?.last["_id"]}");
    //
    // final startOid = state.value?.last["_id"]! as ObjectId;
    // debugPrint("fetchMore $pageIndex ${startOid.oid}");
    //
    // final metaData = await ref.watch(metaDataFutureProvider.future);
    // try {
    //   final stream = await fetchEntitiesPage(arg.manageId, 0, startOid.oid, {"_id": -1}, arg.stubCall, metaData);
    //   stream.listen((entity) {
    //     final newEntities = state.value ?? [];
    //     newEntities.add(entity);
    //     state = AsyncValue.data(newEntities);
    //   });
    // } catch (err) {
    //   debugPrint("fetchMore error: $err");
    // }
  }

  @override
  FutureOr<List<String>> build(EntitiesPageNotifierArgs arg) async {
    final account = await ref.watch(accountAsyncNotifierProvider.future);
    if (account.status != LoginStatus.LoggedIn) {
      return Future.error("请先登录");
    }

    // final metaData = await ref.watch(metaDataFutureProvider.future);

    final cache = await ref.watch(cacheProvider(arg.fetchCallsArg).future);
    cache.refreshCache();

    try {
      final stream = cache.fetchPage(0);
      stream.listen((id) {
        final ids = state.value ?? [];
        ids.add(id);
        state = AsyncValue.data(ids);
      });
      // final stream = await fetchEntitiesPage(arg.manageId, 0, "", {"_id": -1}, arg.stubCall, metaData);
      fetchedPages.add(0);
      // stream.listen((entity) {
      //   final newEntities = state.value ?? [];
      //   newEntities.add(entity);
      //   state = AsyncValue.data(newEntities);
      // });
      return [];
    } catch (err) {
      return [];
    }
  }
}

final entitiesPageNotifierProvider = AsyncNotifierProvider.autoDispose
    .family<EntitiesPageAsyncNotifier, List<String>, EntitiesPageNotifierArgs>(() => EntitiesPageAsyncNotifier());
