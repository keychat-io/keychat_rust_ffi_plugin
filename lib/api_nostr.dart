// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.7.0.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import 'frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

// These functions are ignored because they are not marked as `pub`: `create_gift`, `create_unsigned_event`, `get_xonly_pubkey`
// These function are ignored because they are on traits that is not defined in current crate (put an empty `#[frb]` on it to unignore): `clone`, `clone`, `clone`, `eq`, `fmt`, `fmt`, `fmt`

Future<Secp256k1Account> generateSecp256K1() => RustLib.instance.api.crateApiNostrGenerateSecp256K1();

Future<Secp256k1Account> generateFromMnemonic({String? password}) =>
    RustLib.instance.api.crateApiNostrGenerateFromMnemonic(password: password);

Future<Secp256k1SimpleAccount> generateSimple() => RustLib.instance.api.crateApiNostrGenerateSimple();

Future<Secp256k1Account> importKey({required String senderKeys}) =>
    RustLib.instance.api.crateApiNostrImportKey(senderKeys: senderKeys);

Future<Secp256k1Account> importFromPhrase({required String phrase, String? password, int? account}) =>
    RustLib.instance.api.crateApiNostrImportFromPhrase(phrase: phrase, password: password, account: account);

Future<List<Secp256k1Account>> importFromPhraseWith(
        {required String phrase, String? password, required int offset, required int count}) =>
    RustLib.instance.api
        .crateApiNostrImportFromPhraseWith(phrase: phrase, password: password, offset: offset, count: count);

String getHexPubkeyByBech32({required String bech32}) =>
    RustLib.instance.api.crateApiNostrGetHexPubkeyByBech32(bech32: bech32);

String getBech32PubkeyByHex({required String hex}) => RustLib.instance.api.crateApiNostrGetBech32PubkeyByHex(hex: hex);

String getBech32PrikeyByHex({required String hex}) => RustLib.instance.api.crateApiNostrGetBech32PrikeyByHex(hex: hex);

String getHexPrikeyByBech32({required String bech32}) =>
    RustLib.instance.api.crateApiNostrGetHexPrikeyByBech32(bech32: bech32);

String getHexPubkeyByPrikey({required String prikey}) =>
    RustLib.instance.api.crateApiNostrGetHexPubkeyByPrikey(prikey: prikey);

Future<String> createGiftJson(
        {required int kind,
        required String senderKeys,
        required String receiverPubkey,
        required String content,
        String? reply,
        BigInt? expirationTimestamp,
        bool? timestampTweaked}) =>
    RustLib.instance.api.crateApiNostrCreateGiftJson(
        kind: kind,
        senderKeys: senderKeys,
        receiverPubkey: receiverPubkey,
        content: content,
        reply: reply,
        expirationTimestamp: expirationTimestamp,
        timestampTweaked: timestampTweaked);

Future<NostrEvent> decryptGift({required String senderKeys, required String receiver, required String content}) =>
    RustLib.instance.api.crateApiNostrDecryptGift(senderKeys: senderKeys, receiver: receiver, content: content);

Future<String> getEncryptEvent(
        {required String senderKeys, required String receiverPubkey, required String content, String? reply}) =>
    RustLib.instance.api.crateApiNostrGetEncryptEvent(
        senderKeys: senderKeys, receiverPubkey: receiverPubkey, content: content, reply: reply);

Future<String> getUnencryptEvent(
        {required String senderKeys, required List<String> receiverPubkeys, required String content, String? reply}) =>
    RustLib.instance.api.crateApiNostrGetUnencryptEvent(
        senderKeys: senderKeys, receiverPubkeys: receiverPubkeys, content: content, reply: reply);

Future<String> encrypt({required String senderKeys, required String receiverPubkey, required String content}) =>
    RustLib.instance.api.crateApiNostrEncrypt(senderKeys: senderKeys, receiverPubkey: receiverPubkey, content: content);

Future<String> decrypt({required String senderKeys, required String receiverPubkey, required String content}) =>
    RustLib.instance.api.crateApiNostrDecrypt(senderKeys: senderKeys, receiverPubkey: receiverPubkey, content: content);

Future<String> setMetadata({required String senderKeys, required String content}) =>
    RustLib.instance.api.crateApiNostrSetMetadata(senderKeys: senderKeys, content: content);

Future<String> decryptEvent({required String senderKeys, required String json}) =>
    RustLib.instance.api.crateApiNostrDecryptEvent(senderKeys: senderKeys, json: json);

Future<NostrEvent> verifyEvent({required String json}) => RustLib.instance.api.crateApiNostrVerifyEvent(json: json);

Future<String> signSchnorr({required String senderKeys, required String content}) =>
    RustLib.instance.api.crateApiNostrSignSchnorr(senderKeys: senderKeys, content: content);

Future<bool> verifySchnorr(
        {required String pubkey, required String sig, required String content, required bool hash}) =>
    RustLib.instance.api.crateApiNostrVerifySchnorr(pubkey: pubkey, sig: sig, content: content, hash: hash);

Future<(Uint8List, Uint8List)> generateCurve25519Keypair({required String mnemonicWords, String? password, int? pos}) =>
    RustLib.instance.api
        .crateApiNostrGenerateCurve25519Keypair(mnemonicWords: mnemonicWords, password: password, pos: pos);

Future<String> curve25519Sign({required List<int> secretKey, required List<int> message}) =>
    RustLib.instance.api.crateApiNostrCurve25519Sign(secretKey: secretKey, message: message);

Future<String> curve25519GetPubkey({required String prikey}) =>
    RustLib.instance.api.crateApiNostrCurve25519GetPubkey(prikey: prikey);

Future<bool> curve25519Verify({required List<int> publicKey, required List<int> message, required String sig}) =>
    RustLib.instance.api.crateApiNostrCurve25519Verify(publicKey: publicKey, message: message, sig: sig);

Future<String> generateSeedFromRatchetkeyPair({required String seedKey}) =>
    RustLib.instance.api.crateApiNostrGenerateSeedFromRatchetkeyPair(seedKey: seedKey);

Future<String> generateMessageKeyHash({required String seedKey}) =>
    RustLib.instance.api.crateApiNostrGenerateMessageKeyHash(seedKey: seedKey);

Future<String> generateSeedFromKey({required List<int> seedKey}) =>
    RustLib.instance.api.crateApiNostrGenerateSeedFromKey(seedKey: seedKey);

class NostrEvent {
  /// Id
  final String id;

  /// Author
  final String pubkey;

  /// Timestamp (seconds)
  final BigInt createdAt;

  /// Kind
  final BigInt kind;

  /// Vector of [`Tag`]
  final List<List<String>> tags;

  /// Content
  final String content;

  /// Signature
  final String sig;

  const NostrEvent({
    required this.id,
    required this.pubkey,
    required this.createdAt,
    required this.kind,
    required this.tags,
    required this.content,
    required this.sig,
  });

  @override
  int get hashCode =>
      id.hashCode ^
      pubkey.hashCode ^
      createdAt.hashCode ^
      kind.hashCode ^
      tags.hashCode ^
      content.hashCode ^
      sig.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is NostrEvent &&
          runtimeType == other.runtimeType &&
          id == other.id &&
          pubkey == other.pubkey &&
          createdAt == other.createdAt &&
          kind == other.kind &&
          tags == other.tags &&
          content == other.content &&
          sig == other.sig;
}

class Secp256k1Account {
  final String? mnemonic;
  final String pubkey;
  final String prikey;
  final String pubkeyBech32;
  final String prikeyBech32;
  final Uint8List? curve25519Sk;
  final Uint8List? curve25519Pk;
  final String? curve25519SkHex;
  final String? curve25519PkHex;

  const Secp256k1Account({
    this.mnemonic,
    required this.pubkey,
    required this.prikey,
    required this.pubkeyBech32,
    required this.prikeyBech32,
    this.curve25519Sk,
    this.curve25519Pk,
    this.curve25519SkHex,
    this.curve25519PkHex,
  });

  @override
  int get hashCode =>
      mnemonic.hashCode ^
      pubkey.hashCode ^
      prikey.hashCode ^
      pubkeyBech32.hashCode ^
      prikeyBech32.hashCode ^
      curve25519Sk.hashCode ^
      curve25519Pk.hashCode ^
      curve25519SkHex.hashCode ^
      curve25519PkHex.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is Secp256k1Account &&
          runtimeType == other.runtimeType &&
          mnemonic == other.mnemonic &&
          pubkey == other.pubkey &&
          prikey == other.prikey &&
          pubkeyBech32 == other.pubkeyBech32 &&
          prikeyBech32 == other.prikeyBech32 &&
          curve25519Sk == other.curve25519Sk &&
          curve25519Pk == other.curve25519Pk &&
          curve25519SkHex == other.curve25519SkHex &&
          curve25519PkHex == other.curve25519PkHex;
}

class Secp256k1SimpleAccount {
  final String pubkey;
  final String prikey;

  const Secp256k1SimpleAccount({
    required this.pubkey,
    required this.prikey,
  });

  @override
  int get hashCode => pubkey.hashCode ^ prikey.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is Secp256k1SimpleAccount &&
          runtimeType == other.runtimeType &&
          pubkey == other.pubkey &&
          prikey == other.prikey;
}
