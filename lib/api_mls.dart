// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.7.0.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import 'frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

// These types are ignored because they are not used by any `pub` functions: `MlsStore`, `RUNTIME`, `STORE`
// These function are ignored because they are on traits that is not defined in current crate (put an empty `#[frb]` on it to unignore): `deref`, `deref`, `initialize`, `initialize`

Future<void> initMlsDb({required String dbPath, required String nostrId}) =>
    RustLib.instance.api.crateApiMlsInitMlsDb(dbPath: dbPath, nostrId: nostrId);

Future<Uint8List> getExportSecret({required String nostrId, required String groupId}) =>
    RustLib.instance.api.crateApiMlsGetExportSecret(nostrId: nostrId, groupId: groupId);

Future<Uint8List> getTreeHash({required String nostrId, required String groupId}) =>
    RustLib.instance.api.crateApiMlsGetTreeHash(nostrId: nostrId, groupId: groupId);

Future<Uint8List> createKeyPackage({required String nostrId}) =>
    RustLib.instance.api.crateApiMlsCreateKeyPackage(nostrId: nostrId);

Future<void> deleteKeyPackage({required String nostrId, required List<int> keyPackage}) =>
    RustLib.instance.api.crateApiMlsDeleteKeyPackage(nostrId: nostrId, keyPackage: keyPackage);

Future<Uint8List> createGroupConfig() => RustLib.instance.api.crateApiMlsCreateGroupConfig();

Future<Uint8List> getGroupConfig({required String nostrId, required String groupId}) =>
    RustLib.instance.api.crateApiMlsGetGroupConfig(nostrId: nostrId, groupId: groupId);

Future<Map<String, List<Uint8List>>> getMemberExtension({required String nostrId, required String groupId}) =>
    RustLib.instance.api.crateApiMlsGetMemberExtension(nostrId: nostrId, groupId: groupId);

Future<String> getGroupExtension({required String nostrId, required String groupId}) =>
    RustLib.instance.api.crateApiMlsGetGroupExtension(nostrId: nostrId, groupId: groupId);

Future<String> parseWelcomeMessage({required String nostrId, required List<int> welcome}) =>
    RustLib.instance.api.crateApiMlsParseWelcomeMessage(nostrId: nostrId, welcome: welcome);

Future<List<String>> getGroupMembers({required String nostrId, required String groupId}) =>
    RustLib.instance.api.crateApiMlsGetGroupMembers(nostrId: nostrId, groupId: groupId);

Future<Uint8List> createMlsGroup(
        {required String nostrId,
        required String groupId,
        required String description,
        required List<String> adminPubkeysHex,
        required List<String> groupRelays}) =>
    RustLib.instance.api.crateApiMlsCreateMlsGroup(
        nostrId: nostrId,
        groupId: groupId,
        description: description,
        adminPubkeysHex: adminPubkeysHex,
        groupRelays: groupRelays);

Future<String> addMembers({required String nostrId, required String groupId, required List<Uint8List> keyPackages}) =>
    RustLib.instance.api.crateApiMlsAddMembers(nostrId: nostrId, groupId: groupId, keyPackages: keyPackages);

///* PrivateMessage
///    ContentType::Application = 1
///    ContentType::Proposal = 2
///    ContentType::Commit = 3
///* Welcome = 4
///* GroupInfo = 5
///* KeyPackage = 6  Not use
///* PublicMessage = 0
Future<int> parseMlsMsgType({required List<int> data}) => RustLib.instance.api.crateApiMlsParseMlsMsgType(data: data);

Future<void> selfCommit({required String nostrId, required String groupId}) =>
    RustLib.instance.api.crateApiMlsSelfCommit(nostrId: nostrId, groupId: groupId);

Future<void> joinMlsGroup({required String nostrId, required String groupId, required List<int> welcome}) =>
    RustLib.instance.api.crateApiMlsJoinMlsGroup(nostrId: nostrId, groupId: groupId, welcome: welcome);

Future<void> deleteGroup({required String nostrId, required String groupId}) =>
    RustLib.instance.api.crateApiMlsDeleteGroup(nostrId: nostrId, groupId: groupId);

Future<void> othersCommitNormal({required String nostrId, required String groupId, required List<int> queuedMsg}) =>
    RustLib.instance.api.crateApiMlsOthersCommitNormal(nostrId: nostrId, groupId: groupId, queuedMsg: queuedMsg);

Future<String> sendMsg({required String nostrId, required String groupId, required String msg}) =>
    RustLib.instance.api.crateApiMlsSendMsg(nostrId: nostrId, groupId: groupId, msg: msg);

Future<String> decryptMsg({required String nostrId, required String groupId, required List<int> msg}) =>
    RustLib.instance.api.crateApiMlsDecryptMsg(nostrId: nostrId, groupId: groupId, msg: msg);

Future<Uint8List> getLeadNodeIndex(
        {required String nostrIdAdmin, required String nostrIdCommon, required String groupId}) =>
    RustLib.instance.api
        .crateApiMlsGetLeadNodeIndex(nostrIdAdmin: nostrIdAdmin, nostrIdCommon: nostrIdCommon, groupId: groupId);

Future<Uint8List> removeMembers({required String nostrId, required String groupId, required List<Uint8List> members}) =>
    RustLib.instance.api.crateApiMlsRemoveMembers(nostrId: nostrId, groupId: groupId, members: members);

Future<void> othersCommitRemoveMember(
        {required String nostrId, required String groupId, required List<int> queuedMsg}) =>
    RustLib.instance.api.crateApiMlsOthersCommitRemoveMember(nostrId: nostrId, groupId: groupId, queuedMsg: queuedMsg);

Future<Uint8List> selfLeave({required String nostrId, required String groupId}) =>
    RustLib.instance.api.crateApiMlsSelfLeave(nostrId: nostrId, groupId: groupId);

Future<Uint8List> selfUpdate({required String nostrId, required String groupId, required List<int> extensions}) =>
    RustLib.instance.api.crateApiMlsSelfUpdate(nostrId: nostrId, groupId: groupId, extensions: extensions);

Future<void> othersProposalLeave({required String nostrId, required String groupId, required List<int> queuedMsg}) =>
    RustLib.instance.api.crateApiMlsOthersProposalLeave(nostrId: nostrId, groupId: groupId, queuedMsg: queuedMsg);

Future<void> adminCommitLeave({required String nostrId, required String groupId}) =>
    RustLib.instance.api.crateApiMlsAdminCommitLeave(nostrId: nostrId, groupId: groupId);

Future<Uint8List> adminProposalLeave({required String nostrId, required String groupId}) =>
    RustLib.instance.api.crateApiMlsAdminProposalLeave(nostrId: nostrId, groupId: groupId);

Future<void> normalMemberCommitLeave(
        {required String nostrId, required String groupId, required List<int> queuedMsg}) =>
    RustLib.instance.api.crateApiMlsNormalMemberCommitLeave(nostrId: nostrId, groupId: groupId, queuedMsg: queuedMsg);

Future<bool> isAdmin({required String nostrId, required String groupId, required List<int> queuedMsg}) =>
    RustLib.instance.api.crateApiMlsIsAdmin(nostrId: nostrId, groupId: groupId, queuedMsg: queuedMsg);

Future<String> getSender({required String nostrId, required String groupId, required List<int> queuedMsg}) =>
    RustLib.instance.api.crateApiMlsGetSender(nostrId: nostrId, groupId: groupId, queuedMsg: queuedMsg);
