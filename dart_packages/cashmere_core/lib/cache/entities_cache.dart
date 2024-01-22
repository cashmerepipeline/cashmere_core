import 'package:bson/bson.dart';
import 'package:cashmere_core/cache/cached_entity.dart';
import 'package:cashmere_core/cache/timestamp_extension.dart';
import 'package:cashmere_core/fetchers/fetch_upadted_entities_later_than_time.dart';
import 'package:cashmere_core/grpc_call.dart';
import 'package:cashmere_core/ids/general_field_ids.dart';
import 'package:cashmere_core/protocols/entity.pb.dart';
import 'package:flutter/foundation.dart';
import 'package:grpc/grpc.dart';
import 'package:isar/isar.dart';

class EntitiesCacheArg {
  final String manageId;
  final GrpcCall<CheckEntitiesUpdateRequest, CheckEntitiesUpdateResponse> checkEntitiesUpdateCall;
  final ResponseStreamGrpcCall<CheckUpdatesLaterThenTimeRequest, CheckUpdatesLaterThenTimeResponse>
      checkUpdatesLaterThanTimeCall;
  final ResponseStreamGrpcCall<GetEntitiesRequest, GetEntitiesResponse> getEntitiesCall;
  final GrpcCall<GetEntityRequest, GetEntityResponse> getEntityCall;

  EntitiesCacheArg({
    required this.manageId,
    required this.checkEntitiesUpdateCall,
    required this.checkUpdatesLaterThanTimeCall,
    required this.getEntitiesCall,
    required this.getEntityCall,
  });
}

class EntitiesCache {
  final String manageId;
  final Map<String, String> metadata;
  final Isar isar;
  final EntitiesCacheArg args;

  EntitiesCache({
    required this.manageId,
    required this.metadata,
    required this.isar,
    required this.args,
  });

  Stream<CachedEntity> watchEntity({required String entityOid}) async* {
    final query = isar.cachedEntitys.where().oidEqualTo(entityOid).build();

    await for (final results in query.watch(fireImmediately: true)) {
      if (results.isNotEmpty) {
        final cached = results.first;
        if (cached.data == null) {
          final data = await fetchEntity(cached);
          cached.data = data;
          isar.writeAsync((isar) {
            isar.cachedEntitys.put(cached);
          });
        }

        if (cached.lastChecked.difference(DateTime.now()).inMinutes > 5) {
          checkEntityUpdate(cached).then((value) => debugPrint('checkEntityUpdate finished: ${cached.oid}'));
        }

        yield cached;
      }
    }
  }

  Stream<CachedEntity> watchLatestEntity({required String entityId}) async* {
    final query = isar.cachedEntitys.where().sortByLastModifiedDesc().build();

    await for (final results in query.watch(fireImmediately: true)) {
      if (results.isNotEmpty) {
        yield results.first;
      }
    }
  }

  Future<void> checkEntityUpdate(CachedEntity cached) async {
    final timestamp = cached.lastModified.toTimestamp();

    final entitTimestmap = EntityTimestamp(
      entityId: cached.oid,
      timestamp: BsonCodec.serialize(<String, Timestamp>{"value": timestamp}).byteList,
    );

    final request = CheckEntitiesUpdateRequest(
      manageId: manageId,
      entities: [entitTimestmap],
    );

    final response = await args.checkEntitiesUpdateCall(request, options: CallOptions(metadata: metadata));
    if (response.entities.isNotEmpty) {
      final entityBson = response.entities.first;
      final entity = BsonCodec.deserialize(BsonBinary.from(entityBson));
      final timestamp = entity[MODIFY_TIMESTAMP_FIELD_ID.toString()] as Timestamp;
      debugPrint('checkEntityUpdate: $entity');

      final newCached = CachedEntity(
        oid: cached.oid,
        nid: entity[ID_FIELD_ID.toString()] as String,
        lastChecked: DateTime.now(),
        data: entityBson,
        lastModified: timestamp.toDateTime(),
      );

      isar.writeAsync((isar) {
        isar.cachedEntitys.put(newCached);
      });
    }
  }

  /// zh: 取得实体
  Future<List<byte>> fetchEntity(CachedEntity cached) async {
    final request = GetEntityRequest(manageId: manageId, entityId: cached.nid);
    final response = await args.getEntityCall(request, options: CallOptions(metadata: metadata));

    return response.entity;
  }

  /// pageIndex 从0开始
  /// 返回实体oid
  Stream<String> fetchPage(int pageIndex) async* {
    final items =
        await isar.cachedEntitys.where().sortByLastModifiedDesc().findAllAsync(offset: pageIndex * 20, limit: 20);
    for (var cached in items) {
      if (cached.data == null) {
        checkEntityUpdate(cached);
      }
      yield cached.oid;
    }
  }

  /// zh: 取得最近更新, 一次最多1000个
  Future<void> refreshCache() async {
    final lastModified = await getLatesModifiedTimestamp();
    debugPrint("refreshCache lastModified: $lastModified");

    final arg = FetchLaterThanTimeArg(
      manageId: manageId,
      fetchCall: args.checkUpdatesLaterThanTimeCall,
      metadata: metadata,
      timestamp: lastModified,
    );
    await for (final items in fetchEntitiesLaterThanTime(arg)) {
      debugPrint("get new items: ${items.length}");

      for (final newEntity in items) {
        final timestamp = newEntity[MODIFY_TIMESTAMP_FIELD_ID.toString()] as Timestamp;
        final newCached = CachedEntity(
          oid: (newEntity["_id"] as ObjectId).oid,
          nid: newEntity[ID_FIELD_ID.toString()] as String,
          lastChecked: DateTime.now(),
          data: null,
          lastModified: timestamp.toDateTime(),
        );

        isar.writeAsync((isar) {
          isar.cachedEntitys.put(newCached);
        });
      }
    }
  }

  /// 取得上次检查时间，不存在返回1970-01-01
  Future<DateTime> getLastCheckedTime() async {
    final cached = await isar.cachedEntitys.where().sortByLastCheckedDesc().findFirstAsync();
    if (cached != null) {
      return cached.lastChecked;
    }
    return DateTime.utc(1970);
  }

  /// 取得最晚修改时间戳，不存在返回0
  Future<Timestamp> getLatesModifiedTimestamp() async {
    final cached = await isar.cachedEntitys.where().sortByLastModifiedDesc().findFirstAsync();
    if (cached != null) {
      return cached.lastModified.toTimestamp();
    }

    return Timestamp(0, 0);
  }

  /// zh: 更新实体
  Future<void> updateEntity(String entityId) async {
    final request = GetEntityRequest(manageId: manageId, entityId: entityId);
    final response = await args.getEntityCall(request, options: CallOptions(metadata: metadata));

    final entity = BsonCodec.deserialize(BsonBinary.from(response.entity));
    final timestamp = entity[MODIFY_TIMESTAMP_FIELD_ID.toString()] as Timestamp;
    final newCached = CachedEntity(
      oid: (entity["_id"] as ObjectId).oid,
      nid: entity[ID_FIELD_ID.toString()] as String,
      lastChecked: DateTime.now(),
      data: response.entity,
      lastModified: timestamp.toDateTime(),
    );

    isar.writeAsync((isar) {
      isar.cachedEntitys.put(newCached);
    });
  }
}
