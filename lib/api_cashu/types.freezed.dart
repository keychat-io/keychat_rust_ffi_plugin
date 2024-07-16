// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'types.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#adding-getters-and-methods-to-our-models');

/// @nodoc
mixin _$Transaction {
  Object get field0 => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(CashuTransaction field0) cashu,
    required TResult Function(LNTransaction field0) ln,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(CashuTransaction field0)? cashu,
    TResult? Function(LNTransaction field0)? ln,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(CashuTransaction field0)? cashu,
    TResult Function(LNTransaction field0)? ln,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Transaction_Cashu value) cashu,
    required TResult Function(Transaction_LN value) ln,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Transaction_Cashu value)? cashu,
    TResult? Function(Transaction_LN value)? ln,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Transaction_Cashu value)? cashu,
    TResult Function(Transaction_LN value)? ln,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $TransactionCopyWith<$Res> {
  factory $TransactionCopyWith(
          Transaction value, $Res Function(Transaction) then) =
      _$TransactionCopyWithImpl<$Res, Transaction>;
}

/// @nodoc
class _$TransactionCopyWithImpl<$Res, $Val extends Transaction>
    implements $TransactionCopyWith<$Res> {
  _$TransactionCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$Transaction_CashuImplCopyWith<$Res> {
  factory _$$Transaction_CashuImplCopyWith(_$Transaction_CashuImpl value,
          $Res Function(_$Transaction_CashuImpl) then) =
      __$$Transaction_CashuImplCopyWithImpl<$Res>;
  @useResult
  $Res call({CashuTransaction field0});
}

/// @nodoc
class __$$Transaction_CashuImplCopyWithImpl<$Res>
    extends _$TransactionCopyWithImpl<$Res, _$Transaction_CashuImpl>
    implements _$$Transaction_CashuImplCopyWith<$Res> {
  __$$Transaction_CashuImplCopyWithImpl(_$Transaction_CashuImpl _value,
      $Res Function(_$Transaction_CashuImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$Transaction_CashuImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as CashuTransaction,
    ));
  }
}

/// @nodoc

class _$Transaction_CashuImpl extends Transaction_Cashu {
  const _$Transaction_CashuImpl(this.field0) : super._();

  @override
  final CashuTransaction field0;

  @override
  String toString() {
    return 'Transaction.cashu(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Transaction_CashuImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$Transaction_CashuImplCopyWith<_$Transaction_CashuImpl> get copyWith =>
      __$$Transaction_CashuImplCopyWithImpl<_$Transaction_CashuImpl>(
          this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(CashuTransaction field0) cashu,
    required TResult Function(LNTransaction field0) ln,
  }) {
    return cashu(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(CashuTransaction field0)? cashu,
    TResult? Function(LNTransaction field0)? ln,
  }) {
    return cashu?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(CashuTransaction field0)? cashu,
    TResult Function(LNTransaction field0)? ln,
    required TResult orElse(),
  }) {
    if (cashu != null) {
      return cashu(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Transaction_Cashu value) cashu,
    required TResult Function(Transaction_LN value) ln,
  }) {
    return cashu(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Transaction_Cashu value)? cashu,
    TResult? Function(Transaction_LN value)? ln,
  }) {
    return cashu?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Transaction_Cashu value)? cashu,
    TResult Function(Transaction_LN value)? ln,
    required TResult orElse(),
  }) {
    if (cashu != null) {
      return cashu(this);
    }
    return orElse();
  }
}

abstract class Transaction_Cashu extends Transaction {
  const factory Transaction_Cashu(final CashuTransaction field0) =
      _$Transaction_CashuImpl;
  const Transaction_Cashu._() : super._();

  @override
  CashuTransaction get field0;
  @JsonKey(ignore: true)
  _$$Transaction_CashuImplCopyWith<_$Transaction_CashuImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$Transaction_LNImplCopyWith<$Res> {
  factory _$$Transaction_LNImplCopyWith(_$Transaction_LNImpl value,
          $Res Function(_$Transaction_LNImpl) then) =
      __$$Transaction_LNImplCopyWithImpl<$Res>;
  @useResult
  $Res call({LNTransaction field0});
}

/// @nodoc
class __$$Transaction_LNImplCopyWithImpl<$Res>
    extends _$TransactionCopyWithImpl<$Res, _$Transaction_LNImpl>
    implements _$$Transaction_LNImplCopyWith<$Res> {
  __$$Transaction_LNImplCopyWithImpl(
      _$Transaction_LNImpl _value, $Res Function(_$Transaction_LNImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$Transaction_LNImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as LNTransaction,
    ));
  }
}

/// @nodoc

class _$Transaction_LNImpl extends Transaction_LN {
  const _$Transaction_LNImpl(this.field0) : super._();

  @override
  final LNTransaction field0;

  @override
  String toString() {
    return 'Transaction.ln(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Transaction_LNImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$Transaction_LNImplCopyWith<_$Transaction_LNImpl> get copyWith =>
      __$$Transaction_LNImplCopyWithImpl<_$Transaction_LNImpl>(
          this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(CashuTransaction field0) cashu,
    required TResult Function(LNTransaction field0) ln,
  }) {
    return ln(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(CashuTransaction field0)? cashu,
    TResult? Function(LNTransaction field0)? ln,
  }) {
    return ln?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(CashuTransaction field0)? cashu,
    TResult Function(LNTransaction field0)? ln,
    required TResult orElse(),
  }) {
    if (ln != null) {
      return ln(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Transaction_Cashu value) cashu,
    required TResult Function(Transaction_LN value) ln,
  }) {
    return ln(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Transaction_Cashu value)? cashu,
    TResult? Function(Transaction_LN value)? ln,
  }) {
    return ln?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Transaction_Cashu value)? cashu,
    TResult Function(Transaction_LN value)? ln,
    required TResult orElse(),
  }) {
    if (ln != null) {
      return ln(this);
    }
    return orElse();
  }
}

abstract class Transaction_LN extends Transaction {
  const factory Transaction_LN(final LNTransaction field0) =
      _$Transaction_LNImpl;
  const Transaction_LN._() : super._();

  @override
  LNTransaction get field0;
  @JsonKey(ignore: true)
  _$$Transaction_LNImplCopyWith<_$Transaction_LNImpl> get copyWith =>
      throw _privateConstructorUsedError;
}
