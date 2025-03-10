// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.7.0.

// ignore_for_file: unused_import, unused_element, unnecessary_import, duplicate_ignore, invalid_use_of_internal_member, annotate_overrides, non_constant_identifier_names, curly_braces_in_flow_control_structures, prefer_const_literals_to_create_immutables, unused_field

import 'api_cashu.dart';
import 'api_cashu/types.dart';
import 'api_mls.dart';
import 'api_nostr.dart';
import 'api_signal.dart';
import 'dart:async';
import 'dart:convert';
import 'dart:ffi' as ffi;
import 'frb_generated.dart';
import 'lib.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated_io.dart';

abstract class RustLibApiImplPlatform extends BaseApiImpl<RustLibWire> {
  RustLibApiImplPlatform({
    required super.handler,
    required super.wire,
    required super.generalizedFrbRustBinding,
    required super.portManager,
  });

  @protected
  AnyhowException dco_decode_AnyhowException(dynamic raw);

  @protected
  String dco_decode_String(dynamic raw);

  @protected
  bool dco_decode_bool(dynamic raw);

  @protected
  bool dco_decode_box_autoadd_bool(dynamic raw);

  @protected
  CashuTransaction dco_decode_box_autoadd_cashu_transaction(dynamic raw);

  @protected
  KeychatIdentityKey dco_decode_box_autoadd_keychat_identity_key(dynamic raw);

  @protected
  KeychatIdentityKeyPair dco_decode_box_autoadd_keychat_identity_key_pair(dynamic raw);

  @protected
  KeychatProtocolAddress dco_decode_box_autoadd_keychat_protocol_address(dynamic raw);

  @protected
  KeychatSignalSession dco_decode_box_autoadd_keychat_signal_session(dynamic raw);

  @protected
  LNTransaction dco_decode_box_autoadd_ln_transaction(dynamic raw);

  @protected
  MintInfo dco_decode_box_autoadd_mint_info(dynamic raw);

  @protected
  int dco_decode_box_autoadd_u_32(dynamic raw);

  @protected
  BigInt dco_decode_box_autoadd_u_64(dynamic raw);

  @protected
  CashuTransaction dco_decode_cashu_transaction(dynamic raw);

  @protected
  Contact dco_decode_contact(dynamic raw);

  @protected
  int dco_decode_i_32(dynamic raw);

  @protected
  PlatformInt64 dco_decode_i_64(dynamic raw);

  @protected
  InvoiceInfo dco_decode_invoice_info(dynamic raw);

  @protected
  InvoiceStatus dco_decode_invoice_status(dynamic raw);

  @protected
  KeychatIdentityKey dco_decode_keychat_identity_key(dynamic raw);

  @protected
  KeychatIdentityKeyPair dco_decode_keychat_identity_key_pair(dynamic raw);

  @protected
  KeychatProtocolAddress dco_decode_keychat_protocol_address(dynamic raw);

  @protected
  KeychatSignalSession dco_decode_keychat_signal_session(dynamic raw);

  @protected
  List<String> dco_decode_list_String(dynamic raw);

  @protected
  List<CashuTransaction> dco_decode_list_cashu_transaction(dynamic raw);

  @protected
  List<Contact> dco_decode_list_contact(dynamic raw);

  @protected
  List<List<String>> dco_decode_list_list_String(dynamic raw);

  @protected
  List<Uint8List> dco_decode_list_list_prim_u_8_strict(dynamic raw);

  @protected
  List<LNTransaction> dco_decode_list_ln_transaction(dynamic raw);

  @protected
  List<Mint> dco_decode_list_mint(dynamic raw);

  @protected
  List<PaymentMethod> dco_decode_list_payment_method(dynamic raw);

  @protected
  List<int> dco_decode_list_prim_u_8_loose(dynamic raw);

  @protected
  Uint8List dco_decode_list_prim_u_8_strict(dynamic raw);

  @protected
  List<Secp256k1Account> dco_decode_list_secp_256_k_1_account(dynamic raw);

  @protected
  List<Transaction> dco_decode_list_transaction(dynamic raw);

  @protected
  LNTransaction dco_decode_ln_transaction(dynamic raw);

  @protected
  Mint dco_decode_mint(dynamic raw);

  @protected
  MintInfo dco_decode_mint_info(dynamic raw);

  @protected
  NostrEvent dco_decode_nostr_event(dynamic raw);

  @protected
  NutSupported dco_decode_nut_supported(dynamic raw);

  @protected
  Nuts dco_decode_nuts(dynamic raw);

  @protected
  String? dco_decode_opt_String(dynamic raw);

  @protected
  bool? dco_decode_opt_box_autoadd_bool(dynamic raw);

  @protected
  KeychatIdentityKey? dco_decode_opt_box_autoadd_keychat_identity_key(dynamic raw);

  @protected
  KeychatSignalSession? dco_decode_opt_box_autoadd_keychat_signal_session(dynamic raw);

  @protected
  MintInfo? dco_decode_opt_box_autoadd_mint_info(dynamic raw);

  @protected
  int? dco_decode_opt_box_autoadd_u_32(dynamic raw);

  @protected
  BigInt? dco_decode_opt_box_autoadd_u_64(dynamic raw);

  @protected
  List<String>? dco_decode_opt_list_String(dynamic raw);

  @protected
  Uint8List? dco_decode_opt_list_prim_u_8_strict(dynamic raw);

  @protected
  PaymentMethod dco_decode_payment_method(dynamic raw);

  @protected
  PaymentMethodSettings dco_decode_payment_method_settings(dynamic raw);

  @protected
  (Uint8List, Uint8List) dco_decode_record_list_prim_u_8_strict_list_prim_u_8_strict(dynamic raw);

  @protected
  (Uint8List, Uint8List, List<Uint8List>, List<Uint8List>)
      dco_decode_record_list_prim_u_8_strict_list_prim_u_8_strict_list_list_prim_u_8_strict_list_list_prim_u_8_strict(
          dynamic raw);

  @protected
  (Uint8List, Uint8List?) dco_decode_record_list_prim_u_8_strict_opt_list_prim_u_8_strict(dynamic raw);

  @protected
  (Uint8List, String?, String, List<String>?) dco_decode_record_list_prim_u_8_strict_opt_string_string_opt_list_string(
      dynamic raw);

  @protected
  (Uint8List, String, List<String>?) dco_decode_record_list_prim_u_8_strict_string_opt_list_string(dynamic raw);

  @protected
  (String, String, Uint8List?) dco_decode_record_string_string_opt_list_prim_u_8_strict(dynamic raw);

  @protected
  (String, int) dco_decode_record_string_u_32(dynamic raw);

  @protected
  (int, Uint8List, Uint8List) dco_decode_record_u_32_list_prim_u_8_strict_list_prim_u_8_strict(dynamic raw);

  @protected
  (int, Uint8List, Uint8List, Uint8List)
      dco_decode_record_u_32_list_prim_u_8_strict_list_prim_u_8_strict_list_prim_u_8_strict(dynamic raw);

  @protected
  (BigInt, BigInt) dco_decode_record_u_64_usize(dynamic raw);

  @protected
  (BigInt, BigInt) dco_decode_record_usize_usize(dynamic raw);

  @protected
  (BigInt, BigInt, BigInt) dco_decode_record_usize_usize_usize(dynamic raw);

  @protected
  Secp256k1Account dco_decode_secp_256_k_1_account(dynamic raw);

  @protected
  Secp256k1SimpleAccount dco_decode_secp_256_k_1_simple_account(dynamic raw);

  @protected
  TokenInfo dco_decode_token_info(dynamic raw);

  @protected
  Transaction dco_decode_transaction(dynamic raw);

  @protected
  TransactionDirection dco_decode_transaction_direction(dynamic raw);

  @protected
  TransactionStatus dco_decode_transaction_status(dynamic raw);

  @protected
  int dco_decode_u_16(dynamic raw);

  @protected
  int dco_decode_u_32(dynamic raw);

  @protected
  BigInt dco_decode_u_64(dynamic raw);

  @protected
  int dco_decode_u_8(dynamic raw);

  @protected
  U8Array32 dco_decode_u_8_array_32(dynamic raw);

  @protected
  U8Array33 dco_decode_u_8_array_33(dynamic raw);

  @protected
  void dco_decode_unit(dynamic raw);

  @protected
  BigInt dco_decode_usize(dynamic raw);

  @protected
  AnyhowException sse_decode_AnyhowException(SseDeserializer deserializer);

  @protected
  String sse_decode_String(SseDeserializer deserializer);

  @protected
  bool sse_decode_bool(SseDeserializer deserializer);

  @protected
  bool sse_decode_box_autoadd_bool(SseDeserializer deserializer);

  @protected
  CashuTransaction sse_decode_box_autoadd_cashu_transaction(SseDeserializer deserializer);

  @protected
  KeychatIdentityKey sse_decode_box_autoadd_keychat_identity_key(SseDeserializer deserializer);

  @protected
  KeychatIdentityKeyPair sse_decode_box_autoadd_keychat_identity_key_pair(SseDeserializer deserializer);

  @protected
  KeychatProtocolAddress sse_decode_box_autoadd_keychat_protocol_address(SseDeserializer deserializer);

  @protected
  KeychatSignalSession sse_decode_box_autoadd_keychat_signal_session(SseDeserializer deserializer);

  @protected
  LNTransaction sse_decode_box_autoadd_ln_transaction(SseDeserializer deserializer);

  @protected
  MintInfo sse_decode_box_autoadd_mint_info(SseDeserializer deserializer);

  @protected
  int sse_decode_box_autoadd_u_32(SseDeserializer deserializer);

  @protected
  BigInt sse_decode_box_autoadd_u_64(SseDeserializer deserializer);

  @protected
  CashuTransaction sse_decode_cashu_transaction(SseDeserializer deserializer);

  @protected
  Contact sse_decode_contact(SseDeserializer deserializer);

  @protected
  int sse_decode_i_32(SseDeserializer deserializer);

  @protected
  PlatformInt64 sse_decode_i_64(SseDeserializer deserializer);

  @protected
  InvoiceInfo sse_decode_invoice_info(SseDeserializer deserializer);

  @protected
  InvoiceStatus sse_decode_invoice_status(SseDeserializer deserializer);

  @protected
  KeychatIdentityKey sse_decode_keychat_identity_key(SseDeserializer deserializer);

  @protected
  KeychatIdentityKeyPair sse_decode_keychat_identity_key_pair(SseDeserializer deserializer);

  @protected
  KeychatProtocolAddress sse_decode_keychat_protocol_address(SseDeserializer deserializer);

  @protected
  KeychatSignalSession sse_decode_keychat_signal_session(SseDeserializer deserializer);

  @protected
  List<String> sse_decode_list_String(SseDeserializer deserializer);

  @protected
  List<CashuTransaction> sse_decode_list_cashu_transaction(SseDeserializer deserializer);

  @protected
  List<Contact> sse_decode_list_contact(SseDeserializer deserializer);

  @protected
  List<List<String>> sse_decode_list_list_String(SseDeserializer deserializer);

  @protected
  List<Uint8List> sse_decode_list_list_prim_u_8_strict(SseDeserializer deserializer);

  @protected
  List<LNTransaction> sse_decode_list_ln_transaction(SseDeserializer deserializer);

  @protected
  List<Mint> sse_decode_list_mint(SseDeserializer deserializer);

  @protected
  List<PaymentMethod> sse_decode_list_payment_method(SseDeserializer deserializer);

  @protected
  List<int> sse_decode_list_prim_u_8_loose(SseDeserializer deserializer);

  @protected
  Uint8List sse_decode_list_prim_u_8_strict(SseDeserializer deserializer);

  @protected
  List<Secp256k1Account> sse_decode_list_secp_256_k_1_account(SseDeserializer deserializer);

  @protected
  List<Transaction> sse_decode_list_transaction(SseDeserializer deserializer);

  @protected
  LNTransaction sse_decode_ln_transaction(SseDeserializer deserializer);

  @protected
  Mint sse_decode_mint(SseDeserializer deserializer);

  @protected
  MintInfo sse_decode_mint_info(SseDeserializer deserializer);

  @protected
  NostrEvent sse_decode_nostr_event(SseDeserializer deserializer);

  @protected
  NutSupported sse_decode_nut_supported(SseDeserializer deserializer);

  @protected
  Nuts sse_decode_nuts(SseDeserializer deserializer);

  @protected
  String? sse_decode_opt_String(SseDeserializer deserializer);

  @protected
  bool? sse_decode_opt_box_autoadd_bool(SseDeserializer deserializer);

  @protected
  KeychatIdentityKey? sse_decode_opt_box_autoadd_keychat_identity_key(SseDeserializer deserializer);

  @protected
  KeychatSignalSession? sse_decode_opt_box_autoadd_keychat_signal_session(SseDeserializer deserializer);

  @protected
  MintInfo? sse_decode_opt_box_autoadd_mint_info(SseDeserializer deserializer);

  @protected
  int? sse_decode_opt_box_autoadd_u_32(SseDeserializer deserializer);

  @protected
  BigInt? sse_decode_opt_box_autoadd_u_64(SseDeserializer deserializer);

  @protected
  List<String>? sse_decode_opt_list_String(SseDeserializer deserializer);

  @protected
  Uint8List? sse_decode_opt_list_prim_u_8_strict(SseDeserializer deserializer);

  @protected
  PaymentMethod sse_decode_payment_method(SseDeserializer deserializer);

  @protected
  PaymentMethodSettings sse_decode_payment_method_settings(SseDeserializer deserializer);

  @protected
  (Uint8List, Uint8List) sse_decode_record_list_prim_u_8_strict_list_prim_u_8_strict(SseDeserializer deserializer);

  @protected
  (Uint8List, Uint8List, List<Uint8List>, List<Uint8List>)
      sse_decode_record_list_prim_u_8_strict_list_prim_u_8_strict_list_list_prim_u_8_strict_list_list_prim_u_8_strict(
          SseDeserializer deserializer);

  @protected
  (Uint8List, Uint8List?) sse_decode_record_list_prim_u_8_strict_opt_list_prim_u_8_strict(SseDeserializer deserializer);

  @protected
  (Uint8List, String?, String, List<String>?) sse_decode_record_list_prim_u_8_strict_opt_string_string_opt_list_string(
      SseDeserializer deserializer);

  @protected
  (Uint8List, String, List<String>?) sse_decode_record_list_prim_u_8_strict_string_opt_list_string(
      SseDeserializer deserializer);

  @protected
  (String, String, Uint8List?) sse_decode_record_string_string_opt_list_prim_u_8_strict(SseDeserializer deserializer);

  @protected
  (String, int) sse_decode_record_string_u_32(SseDeserializer deserializer);

  @protected
  (int, Uint8List, Uint8List) sse_decode_record_u_32_list_prim_u_8_strict_list_prim_u_8_strict(
      SseDeserializer deserializer);

  @protected
  (
    int,
    Uint8List,
    Uint8List,
    Uint8List
  ) sse_decode_record_u_32_list_prim_u_8_strict_list_prim_u_8_strict_list_prim_u_8_strict(SseDeserializer deserializer);

  @protected
  (BigInt, BigInt) sse_decode_record_u_64_usize(SseDeserializer deserializer);

  @protected
  (BigInt, BigInt) sse_decode_record_usize_usize(SseDeserializer deserializer);

  @protected
  (BigInt, BigInt, BigInt) sse_decode_record_usize_usize_usize(SseDeserializer deserializer);

  @protected
  Secp256k1Account sse_decode_secp_256_k_1_account(SseDeserializer deserializer);

  @protected
  Secp256k1SimpleAccount sse_decode_secp_256_k_1_simple_account(SseDeserializer deserializer);

  @protected
  TokenInfo sse_decode_token_info(SseDeserializer deserializer);

  @protected
  Transaction sse_decode_transaction(SseDeserializer deserializer);

  @protected
  TransactionDirection sse_decode_transaction_direction(SseDeserializer deserializer);

  @protected
  TransactionStatus sse_decode_transaction_status(SseDeserializer deserializer);

  @protected
  int sse_decode_u_16(SseDeserializer deserializer);

  @protected
  int sse_decode_u_32(SseDeserializer deserializer);

  @protected
  BigInt sse_decode_u_64(SseDeserializer deserializer);

  @protected
  int sse_decode_u_8(SseDeserializer deserializer);

  @protected
  U8Array32 sse_decode_u_8_array_32(SseDeserializer deserializer);

  @protected
  U8Array33 sse_decode_u_8_array_33(SseDeserializer deserializer);

  @protected
  void sse_decode_unit(SseDeserializer deserializer);

  @protected
  BigInt sse_decode_usize(SseDeserializer deserializer);

  @protected
  void sse_encode_AnyhowException(AnyhowException self, SseSerializer serializer);

  @protected
  void sse_encode_String(String self, SseSerializer serializer);

  @protected
  void sse_encode_bool(bool self, SseSerializer serializer);

  @protected
  void sse_encode_box_autoadd_bool(bool self, SseSerializer serializer);

  @protected
  void sse_encode_box_autoadd_cashu_transaction(CashuTransaction self, SseSerializer serializer);

  @protected
  void sse_encode_box_autoadd_keychat_identity_key(KeychatIdentityKey self, SseSerializer serializer);

  @protected
  void sse_encode_box_autoadd_keychat_identity_key_pair(KeychatIdentityKeyPair self, SseSerializer serializer);

  @protected
  void sse_encode_box_autoadd_keychat_protocol_address(KeychatProtocolAddress self, SseSerializer serializer);

  @protected
  void sse_encode_box_autoadd_keychat_signal_session(KeychatSignalSession self, SseSerializer serializer);

  @protected
  void sse_encode_box_autoadd_ln_transaction(LNTransaction self, SseSerializer serializer);

  @protected
  void sse_encode_box_autoadd_mint_info(MintInfo self, SseSerializer serializer);

  @protected
  void sse_encode_box_autoadd_u_32(int self, SseSerializer serializer);

  @protected
  void sse_encode_box_autoadd_u_64(BigInt self, SseSerializer serializer);

  @protected
  void sse_encode_cashu_transaction(CashuTransaction self, SseSerializer serializer);

  @protected
  void sse_encode_contact(Contact self, SseSerializer serializer);

  @protected
  void sse_encode_i_32(int self, SseSerializer serializer);

  @protected
  void sse_encode_i_64(PlatformInt64 self, SseSerializer serializer);

  @protected
  void sse_encode_invoice_info(InvoiceInfo self, SseSerializer serializer);

  @protected
  void sse_encode_invoice_status(InvoiceStatus self, SseSerializer serializer);

  @protected
  void sse_encode_keychat_identity_key(KeychatIdentityKey self, SseSerializer serializer);

  @protected
  void sse_encode_keychat_identity_key_pair(KeychatIdentityKeyPair self, SseSerializer serializer);

  @protected
  void sse_encode_keychat_protocol_address(KeychatProtocolAddress self, SseSerializer serializer);

  @protected
  void sse_encode_keychat_signal_session(KeychatSignalSession self, SseSerializer serializer);

  @protected
  void sse_encode_list_String(List<String> self, SseSerializer serializer);

  @protected
  void sse_encode_list_cashu_transaction(List<CashuTransaction> self, SseSerializer serializer);

  @protected
  void sse_encode_list_contact(List<Contact> self, SseSerializer serializer);

  @protected
  void sse_encode_list_list_String(List<List<String>> self, SseSerializer serializer);

  @protected
  void sse_encode_list_list_prim_u_8_strict(List<Uint8List> self, SseSerializer serializer);

  @protected
  void sse_encode_list_ln_transaction(List<LNTransaction> self, SseSerializer serializer);

  @protected
  void sse_encode_list_mint(List<Mint> self, SseSerializer serializer);

  @protected
  void sse_encode_list_payment_method(List<PaymentMethod> self, SseSerializer serializer);

  @protected
  void sse_encode_list_prim_u_8_loose(List<int> self, SseSerializer serializer);

  @protected
  void sse_encode_list_prim_u_8_strict(Uint8List self, SseSerializer serializer);

  @protected
  void sse_encode_list_secp_256_k_1_account(List<Secp256k1Account> self, SseSerializer serializer);

  @protected
  void sse_encode_list_transaction(List<Transaction> self, SseSerializer serializer);

  @protected
  void sse_encode_ln_transaction(LNTransaction self, SseSerializer serializer);

  @protected
  void sse_encode_mint(Mint self, SseSerializer serializer);

  @protected
  void sse_encode_mint_info(MintInfo self, SseSerializer serializer);

  @protected
  void sse_encode_nostr_event(NostrEvent self, SseSerializer serializer);

  @protected
  void sse_encode_nut_supported(NutSupported self, SseSerializer serializer);

  @protected
  void sse_encode_nuts(Nuts self, SseSerializer serializer);

  @protected
  void sse_encode_opt_String(String? self, SseSerializer serializer);

  @protected
  void sse_encode_opt_box_autoadd_bool(bool? self, SseSerializer serializer);

  @protected
  void sse_encode_opt_box_autoadd_keychat_identity_key(KeychatIdentityKey? self, SseSerializer serializer);

  @protected
  void sse_encode_opt_box_autoadd_keychat_signal_session(KeychatSignalSession? self, SseSerializer serializer);

  @protected
  void sse_encode_opt_box_autoadd_mint_info(MintInfo? self, SseSerializer serializer);

  @protected
  void sse_encode_opt_box_autoadd_u_32(int? self, SseSerializer serializer);

  @protected
  void sse_encode_opt_box_autoadd_u_64(BigInt? self, SseSerializer serializer);

  @protected
  void sse_encode_opt_list_String(List<String>? self, SseSerializer serializer);

  @protected
  void sse_encode_opt_list_prim_u_8_strict(Uint8List? self, SseSerializer serializer);

  @protected
  void sse_encode_payment_method(PaymentMethod self, SseSerializer serializer);

  @protected
  void sse_encode_payment_method_settings(PaymentMethodSettings self, SseSerializer serializer);

  @protected
  void sse_encode_record_list_prim_u_8_strict_list_prim_u_8_strict(
      (Uint8List, Uint8List) self, SseSerializer serializer);

  @protected
  void sse_encode_record_list_prim_u_8_strict_list_prim_u_8_strict_list_list_prim_u_8_strict_list_list_prim_u_8_strict(
      (Uint8List, Uint8List, List<Uint8List>, List<Uint8List>) self, SseSerializer serializer);

  @protected
  void sse_encode_record_list_prim_u_8_strict_opt_list_prim_u_8_strict(
      (Uint8List, Uint8List?) self, SseSerializer serializer);

  @protected
  void sse_encode_record_list_prim_u_8_strict_opt_string_string_opt_list_string(
      (Uint8List, String?, String, List<String>?) self, SseSerializer serializer);

  @protected
  void sse_encode_record_list_prim_u_8_strict_string_opt_list_string(
      (Uint8List, String, List<String>?) self, SseSerializer serializer);

  @protected
  void sse_encode_record_string_string_opt_list_prim_u_8_strict(
      (String, String, Uint8List?) self, SseSerializer serializer);

  @protected
  void sse_encode_record_string_u_32((String, int) self, SseSerializer serializer);

  @protected
  void sse_encode_record_u_32_list_prim_u_8_strict_list_prim_u_8_strict(
      (int, Uint8List, Uint8List) self, SseSerializer serializer);

  @protected
  void sse_encode_record_u_32_list_prim_u_8_strict_list_prim_u_8_strict_list_prim_u_8_strict(
      (int, Uint8List, Uint8List, Uint8List) self, SseSerializer serializer);

  @protected
  void sse_encode_record_u_64_usize((BigInt, BigInt) self, SseSerializer serializer);

  @protected
  void sse_encode_record_usize_usize((BigInt, BigInt) self, SseSerializer serializer);

  @protected
  void sse_encode_record_usize_usize_usize((BigInt, BigInt, BigInt) self, SseSerializer serializer);

  @protected
  void sse_encode_secp_256_k_1_account(Secp256k1Account self, SseSerializer serializer);

  @protected
  void sse_encode_secp_256_k_1_simple_account(Secp256k1SimpleAccount self, SseSerializer serializer);

  @protected
  void sse_encode_token_info(TokenInfo self, SseSerializer serializer);

  @protected
  void sse_encode_transaction(Transaction self, SseSerializer serializer);

  @protected
  void sse_encode_transaction_direction(TransactionDirection self, SseSerializer serializer);

  @protected
  void sse_encode_transaction_status(TransactionStatus self, SseSerializer serializer);

  @protected
  void sse_encode_u_16(int self, SseSerializer serializer);

  @protected
  void sse_encode_u_32(int self, SseSerializer serializer);

  @protected
  void sse_encode_u_64(BigInt self, SseSerializer serializer);

  @protected
  void sse_encode_u_8(int self, SseSerializer serializer);

  @protected
  void sse_encode_u_8_array_32(U8Array32 self, SseSerializer serializer);

  @protected
  void sse_encode_u_8_array_33(U8Array33 self, SseSerializer serializer);

  @protected
  void sse_encode_unit(void self, SseSerializer serializer);

  @protected
  void sse_encode_usize(BigInt self, SseSerializer serializer);
}

// Section: wire_class

class RustLibWire implements BaseWire {
  factory RustLibWire.fromExternalLibrary(ExternalLibrary lib) => RustLibWire(lib.ffiDynamicLibrary);

  /// Holds the symbol lookup function.
  final ffi.Pointer<T> Function<T extends ffi.NativeType>(String symbolName) _lookup;

  /// The symbols are looked up in [dynamicLibrary].
  RustLibWire(ffi.DynamicLibrary dynamicLibrary) : _lookup = dynamicLibrary.lookup;
}
