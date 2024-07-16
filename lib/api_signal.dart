// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.1.0.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import 'frb_generated.dart';
import 'lib.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

// These types are ignored because they are not used by any `pub` functions: `RUNTIME`, `STORE`, `SignalStore`
// These function are ignored because they are on traits that is not defined in current crate (put an empty `#[frb]` on it to unignore): `assert_receiver_is_total_eq`, `assert_receiver_is_total_eq`, `assert_receiver_is_total_eq`, `assert_receiver_is_total_eq`, `clone`, `clone`, `clone`, `clone`, `cmp`, `cmp`, `cmp`, `deref`, `deref`, `eq`, `eq`, `eq`, `eq`, `fmt`, `fmt`, `fmt`, `hash`, `hash`, `initialize`, `initialize`, `partial_cmp`, `partial_cmp`, `partial_cmp`

/// init db and KeyChatSignalProtocolStore, this is used for testing
Future<void> init({required String dbPath, required KeychatIdentityKeyPair keyPair, required int regId}) =>
    RustLib.instance.api.crateApiSignalInit(dbPath: dbPath, keyPair: keyPair, regId: regId);

/// init db
Future<void> initSignalDb({required String dbPath}) => RustLib.instance.api.crateApiSignalInitSignalDb(dbPath: dbPath);

/// init KeyChatSignalProtocolStore
Future<void> initKeypair({required KeychatIdentityKeyPair keyPair, required int regId}) =>
    RustLib.instance.api.crateApiSignalInitKeypair(keyPair: keyPair, regId: regId);

Future<(int, Uint8List, Uint8List)> generateSignedKeyApi(
        {required KeychatIdentityKeyPair keyPair, required List<int> signalIdentityPrivateKey}) =>
    RustLib.instance.api
        .crateApiSignalGenerateSignedKeyApi(keyPair: keyPair, signalIdentityPrivateKey: signalIdentityPrivateKey);

Future<(int, Uint8List)> generatePrekeyApi({required KeychatIdentityKeyPair keyPair}) =>
    RustLib.instance.api.crateApiSignalGeneratePrekeyApi(keyPair: keyPair);

Future<void> processPrekeyBundleApi(
        {required KeychatIdentityKeyPair keyPair,
        required KeychatProtocolAddress remoteAddress,
        required int regId,
        required int deviceId,
        required KeychatIdentityKey identityKey,
        required int bobSignedId,
        required List<int> bobSignedPublic,
        required List<int> bobSigedSig,
        required int bobPrekeyId,
        required List<int> bobPrekeyPublic}) =>
    RustLib.instance.api.crateApiSignalProcessPrekeyBundleApi(
        keyPair: keyPair,
        remoteAddress: remoteAddress,
        regId: regId,
        deviceId: deviceId,
        identityKey: identityKey,
        bobSignedId: bobSignedId,
        bobSignedPublic: bobSignedPublic,
        bobSigedSig: bobSigedSig,
        bobPrekeyId: bobPrekeyId,
        bobPrekeyPublic: bobPrekeyPublic);

Future<(Uint8List, String?, String, List<String>?)> encryptSignal(
        {required KeychatIdentityKeyPair keyPair,
        required String ptext,
        required KeychatProtocolAddress remoteAddress}) =>
    RustLib.instance.api.crateApiSignalEncryptSignal(keyPair: keyPair, ptext: ptext, remoteAddress: remoteAddress);

Future<(Uint8List, String, List<String>?)> decryptSignal(
        {required KeychatIdentityKeyPair keyPair,
        required List<int> ciphertext,
        required KeychatProtocolAddress remoteAddress,
        required int roomId,
        required bool isPrekey}) =>
    RustLib.instance.api.crateApiSignalDecryptSignal(
        keyPair: keyPair, ciphertext: ciphertext, remoteAddress: remoteAddress, roomId: roomId, isPrekey: isPrekey);

Future<KeychatSignalSession?> sessionContainAliceAddr(
        {required KeychatIdentityKeyPair keyPair, required String address}) =>
    RustLib.instance.api.crateApiSignalSessionContainAliceAddr(keyPair: keyPair, address: address);

Future<bool> updateAliceAddr(
        {required KeychatIdentityKeyPair keyPair,
        required String address,
        required String deviceId,
        required String aliceAddr}) =>
    RustLib.instance.api
        .crateApiSignalUpdateAliceAddr(keyPair: keyPair, address: address, deviceId: deviceId, aliceAddr: aliceAddr);

Future<bool> containsSession({required KeychatIdentityKeyPair keyPair, required KeychatProtocolAddress address}) =>
    RustLib.instance.api.crateApiSignalContainsSession(keyPair: keyPair, address: address);

Future<bool> deleteSessionByDeviceId({required KeychatIdentityKeyPair keyPair, required int deviceId}) =>
    RustLib.instance.api.crateApiSignalDeleteSessionByDeviceId(keyPair: keyPair, deviceId: deviceId);

Future<void> deleteSession({required KeychatIdentityKeyPair keyPair, required KeychatProtocolAddress address}) =>
    RustLib.instance.api.crateApiSignalDeleteSession(keyPair: keyPair, address: address);

Future<List<String>> getAllAliceAddrs({required KeychatIdentityKeyPair keyPair}) =>
    RustLib.instance.api.crateApiSignalGetAllAliceAddrs(keyPair: keyPair);

Future<KeychatSignalSession?> getSession(
        {required KeychatIdentityKeyPair keyPair, required String address, required String deviceId}) =>
    RustLib.instance.api.crateApiSignalGetSession(keyPair: keyPair, address: address, deviceId: deviceId);

/// * IdentityStore function
///
Future<bool> deleteIdentity({required KeychatIdentityKeyPair keyPair, required String address}) =>
    RustLib.instance.api.crateApiSignalDeleteIdentity(keyPair: keyPair, address: address);

Future<KeychatIdentityKey?> getIdentity(
        {required KeychatIdentityKeyPair keyPair, required KeychatProtocolAddress address}) =>
    RustLib.instance.api.crateApiSignalGetIdentity(keyPair: keyPair, address: address);

class KeychatIdentityKey {
  final U8Array33 publicKey;

  const KeychatIdentityKey({
    required this.publicKey,
  });

  @override
  int get hashCode => publicKey.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is KeychatIdentityKey && runtimeType == other.runtimeType && publicKey == other.publicKey;
}

class KeychatIdentityKeyPair {
  final U8Array33 identityKey;
  final U8Array32 privateKey;

  const KeychatIdentityKeyPair({
    required this.identityKey,
    required this.privateKey,
  });

  @override
  int get hashCode => identityKey.hashCode ^ privateKey.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is KeychatIdentityKeyPair &&
          runtimeType == other.runtimeType &&
          identityKey == other.identityKey &&
          privateKey == other.privateKey;
}

class KeychatProtocolAddress {
  final String name;
  final int deviceId;

  const KeychatProtocolAddress({
    required this.name,
    required this.deviceId,
  });

  @override
  int get hashCode => name.hashCode ^ deviceId.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is KeychatProtocolAddress &&
          runtimeType == other.runtimeType &&
          name == other.name &&
          deviceId == other.deviceId;
}

class KeychatSignalSession {
  final String? aliceSenderRatchetKey;
  final String address;
  final int device;
  final String? bobSenderRatchetKey;
  final String record;
  final String? bobAddress;
  final String? aliceAddresses;

  const KeychatSignalSession({
    this.aliceSenderRatchetKey,
    required this.address,
    required this.device,
    this.bobSenderRatchetKey,
    required this.record,
    this.bobAddress,
    this.aliceAddresses,
  });

  @override
  int get hashCode =>
      aliceSenderRatchetKey.hashCode ^
      address.hashCode ^
      device.hashCode ^
      bobSenderRatchetKey.hashCode ^
      record.hashCode ^
      bobAddress.hashCode ^
      aliceAddresses.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is KeychatSignalSession &&
          runtimeType == other.runtimeType &&
          aliceSenderRatchetKey == other.aliceSenderRatchetKey &&
          address == other.address &&
          device == other.device &&
          bobSenderRatchetKey == other.bobSenderRatchetKey &&
          record == other.record &&
          bobAddress == other.bobAddress &&
          aliceAddresses == other.aliceAddresses;
}
