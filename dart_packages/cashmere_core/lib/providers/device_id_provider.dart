import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:platform_device_id/platform_device_id.dart';

final deviceIdProvider = FutureProvider<String>((ref) async {
  final deviceId = await PlatformDeviceId.getDeviceId;
  return deviceId!;
});
