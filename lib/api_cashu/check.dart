// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.3.0.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

// These functions are ignored because they have generic arguments: `check_proofs_in_database`
// These function are ignored because they are on traits that is not defined in current crate (put an empty `#[frb]` on it to unignore): `add_counter`, `add_mint`, `add_proofs`, `add_transaction`, `as_ref`, `clone`, `delete_counters`, `delete_proofs`, `delete_transactions`, `deref`, `fmt`, `get_all_proofs`, `get_all_transactions`, `get_counters`, `get_mint`, `get_mints`, `get_pending_transactions`, `get_proofs_limit_unit`, `get_proofs`, `get_transaction`, `get_transactions_with_offset`, `get_transactions`, `initialize`, `migrate`
// These functions are ignored (category: IgnoreBecauseOwnerTyShouldIgnore): `open`

class LitePool {
  final LitePool field0;

  const LitePool({
    required this.field0,
  });

  @override
  int get hashCode => field0.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) || other is LitePool && runtimeType == other.runtimeType && field0 == other.field0;
}