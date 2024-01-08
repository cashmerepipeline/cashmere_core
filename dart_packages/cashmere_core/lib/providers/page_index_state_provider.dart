// page index state provider

import 'package:hooks_riverpod/hooks_riverpod.dart';

final pageIndexStateProvider = StateProvider.autoDispose.family<int, String>((ref, manageId) => 1);