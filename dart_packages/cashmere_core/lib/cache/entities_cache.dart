import 'package:bson/bson.dart';
import 'package:cashmere_core/cache/cached_entity.dart';
import 'package:cashmere_core/fetchers/fetch_upadted_entities_later_than_time.dart';
import 'package:cashmere_core/grpc_call.dart';
import 'package:cashmere_core/ids/general_field_ids.dart';
import 'package:cashmere_core/protocols/entity.pb.dart';
import 'package:flutter/foundation.dart';
import 'package:grpc/grpc.dart';
import 'package:isar/isar.dart';

class EntitiesFetchCallsArg {
  final Map<String, String> metadata;
  final GrpcCall<CheckEntitiesUpdateRequest, CheckEntitiesUpdateResponse> checkEntitiesUpdateCall;
  final ResponseStreamGrpcCall<CheckUpdatesLaterThenTimeRequest, CheckUpdatesLaterThenTimeResponse>
      checkUpdatesLaterThanTimeCall;
  final ResponseStreamGrpcCall<GetEntitiesRequest, GetEntitiesResponse> getEntitiesCall;

  EntitiesFetchCallsArg({
    required this.metadata,
    required this.checkEntitiesUpdateCall,
    required this.checkUpdatesLaterThanTimeCall,
    required this.getEntitiesCall,
  });
}

class EntitiesCache {
  final Isar isar;
  final EntitiesFetchCallsArg args;

  EntitiesCache({
    required this.isar,
    required this.args,
  });

  Stream<CachedEntity> watchEntity({required String manageId, required String entityOid}) async* {
    final query = isar.cachedEntitys.where().manageIdEqualTo(manageId).oidEqualTo(entityOid).build();

    await for (final results in query.watch(fireImmediately: true)) {
      if (results.isNotEmpty) {
        final cached = results.first;
        checkEntityUpdate(cached).then((value) => debugPrint('checkEntityUpdate finished: ${cached.oid}'));
        yield results.first;
      }
    }
  }

  Stream<CachedEntity> watchLatestEntity({required String manageId, required String entityId}) async* {
    final query = isar.cachedEntitys.where().manageIdEqualTo(manageId).sortByLastModifiedDesc().build();

    await for (final results in query.watch(fireImmediately: true)) {
      if (results.isNotEmpty) {
        yield results.first;
      }
    }
  }

  Stream<List<CachedEntity>> watchPage({
    required String manageId,
    required int pageIndex,
  }) async* {
    final latestTimestamp = await getLastModifiedTimestamp(manageId);
    compute(
        fetchEntitiesLaterThanTime,
        FetchLaterThanTimeArg(
          manageId: manageId,
          timestamp: latestTimestamp,
          fetchCall: args.checkUpdatesLaterThanTimeCall,
          metadata: args.metadata,
        )).then((ids) async {
      if (ids.isNotEmpty) {
        final request = GetEntitiesRequest(manageId: manageId, entityIds: ids);
        final response = args.getEntitiesCall(request, options: CallOptions(metadata: args.metadata));
        await for (final resp in response) {
          final entity = BsonCodec.deserialize(BsonBinary.from(resp.entity));
          final timestamp = entity[MODIFY_TIMESTAMP_FIELD_ID.toString()] as Timestamp;
          final newCached = CachedEntity(
            manageId: manageId,
            oid: (entity["_id"] as ObjectId).oid,
            data: entity,
            lastChecked: DateTime.now(),
            lastModified: timestamp.seconds * 1000 + timestamp.increment,
          );

          isar.writeAsync((isar) {
            isar.cachedEntitys.put(newCached);
          });
        }
      }
    });

    final query = isar.cachedEntitys.where().manageIdEqualTo(manageId).sortByLastModifiedDesc().build();

    await for (final results in query.watch(fireImmediately: true, offset: (pageIndex - 1) * 20, limit: 20)) {
      yield results;
    }
  }

  Future<void> checkEntityUpdate(CachedEntity cached) async {
    final timestamp = cached.data[MODIFY_TIMESTAMP_FIELD_ID.toString()] as Timestamp;
    final entitTimestmap = EntityTimestamp(
      entityId: cached.oid,
      timestamp: BsonCodec.serialize(<String, Timestamp>{"value": timestamp}).byteList,
    );

    final request = CheckEntitiesUpdateRequest(
      manageId: cached.manageId,
      entities: [entitTimestmap],
    );

    final response = await args.checkEntitiesUpdateCall(request, options: CallOptions(metadata: args.metadata));
    if (response.entities.isNotEmpty) {
      final entityBson = response.entities.first;
      final entity = BsonCodec.deserialize(BsonBinary.from(entityBson));
      final timestamp = entity[MODIFY_TIMESTAMP_FIELD_ID.toString()] as Timestamp;

      final newCached = CachedEntity(
        oid: cached.oid,
        manageId: cached.manageId,
        lastChecked: DateTime.now(),
        data: entity,
        lastModified: timestamp.seconds * 1000 + timestamp.increment,
      );

      isar.writeAsync((isar) {
        isar.cachedEntitys.put(newCached);
      });
    }
  }

  Future<DateTime> getLastChecked(String manageId) async {
    final cached = await isar.cachedEntitys.where().manageIdEqualTo(manageId).sortByLastCheckedDesc().findFirstAsync();
    if (cached != null) {
      return cached.lastChecked;
    }
    return DateTime.utc(1970);
  }

  /// 取得最晚修改时间戳，不存在返回0
  Future<Timestamp> getLastModifiedTimestamp(String manageId) async {
    final cached = await isar.cachedEntitys.where().manageIdEqualTo(manageId).sortByLastModifiedDesc().findFirstAsync();
    if (cached != null) {
      final timestamp = cached.data[MODIFY_TIMESTAMP_FIELD_ID.toString()] as Timestamp;
      return timestamp;
    }

    return Timestamp(0, 0);
  }
}
