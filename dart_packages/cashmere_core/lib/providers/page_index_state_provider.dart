// page index state provider

import 'package:hooks_riverpod/hooks_riverpod.dart';

final pageIndexStateProvider = StateProvider.autoDispose.family<int, int>((ref, manageId) => 1);