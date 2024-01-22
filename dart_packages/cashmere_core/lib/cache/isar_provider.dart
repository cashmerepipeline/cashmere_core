import 'dart:io';

import 'package:cashmere_core/cache/cached_entity.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:isar/isar.dart';
import 'package:path_provider/path_provider.dart';
import 'package:flutter/foundation.dart';

final isarProvider = FutureProvider.autoDispose.family<Isar, String>((ref, manageId) async {
  final dir = await getApplicationCacheDirectory();
  final dbDir = Directory("${dir.path}/$manageId");
  if (!dbDir.existsSync()) {
    dbDir.createSync(recursive: true);
  }
  debugPrint("isar dir: ${dir.path}/$manageId");

  final isar = Isar.open(
    schemas: [CachedEntitySchema],
    name: manageId,
    directory: "${dir.path}/$manageId",
    // engine: IsarEngine.sqlite,
  );

  debugPrint("isar: ${isar.toString()}");

  return isar;
});
