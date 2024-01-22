// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'cached_entity.dart';

// **************************************************************************
// _IsarCollectionGenerator
// **************************************************************************

// coverage:ignore-file
// ignore_for_file: duplicate_ignore, invalid_use_of_protected_member, lines_longer_than_80_chars, constant_identifier_names, avoid_js_rounded_ints, no_leading_underscores_for_local_identifiers, require_trailing_commas, unnecessary_parenthesis, unnecessary_raw_strings, unnecessary_null_in_if_null_operators, library_private_types_in_public_api, prefer_const_constructors
// ignore_for_file: type=lint

extension GetCachedEntityCollection on Isar {
  IsarCollection<String, CachedEntity> get cachedEntitys => this.collection();
}

const CachedEntitySchema = IsarGeneratedSchema(
  schema: IsarSchema(
    name: 'CachedEntity',
    idName: 'oid',
    embedded: false,
    properties: [
      IsarPropertySchema(
        name: 'oid',
        type: IsarType.string,
      ),
      IsarPropertySchema(
        name: 'nid',
        type: IsarType.string,
      ),
      IsarPropertySchema(
        name: 'data',
        type: IsarType.byteList,
      ),
      IsarPropertySchema(
        name: 'lastModified',
        type: IsarType.dateTime,
      ),
      IsarPropertySchema(
        name: 'lastChecked',
        type: IsarType.dateTime,
      ),
    ],
    indexes: [],
  ),
  converter: IsarObjectConverter<String, CachedEntity>(
    serialize: serializeCachedEntity,
    deserialize: deserializeCachedEntity,
    deserializeProperty: deserializeCachedEntityProp,
  ),
  embeddedSchemas: [],
);

@isarProtected
int serializeCachedEntity(IsarWriter writer, CachedEntity object) {
  IsarCore.writeString(writer, 1, object.oid);
  IsarCore.writeString(writer, 2, object.nid);
  {
    final list = object.data;
    if (list == null) {
      IsarCore.writeNull(writer, 3);
    } else {
      final listWriter = IsarCore.beginList(writer, 3, list.length);
      for (var i = 0; i < list.length; i++) {
        IsarCore.writeByte(listWriter, i, list[i]);
      }
      IsarCore.endList(writer, listWriter);
    }
  }
  IsarCore.writeLong(
      writer, 4, object.lastModified.toUtc().microsecondsSinceEpoch);
  IsarCore.writeLong(
      writer, 5, object.lastChecked.toUtc().microsecondsSinceEpoch);
  return Isar.fastHash(object.oid);
}

@isarProtected
CachedEntity deserializeCachedEntity(IsarReader reader) {
  final String _oid;
  _oid = IsarCore.readString(reader, 1) ?? '';
  final String _nid;
  _nid = IsarCore.readString(reader, 2) ?? '';
  final List<int>? _data;
  {
    final length = IsarCore.readList(reader, 3, IsarCore.readerPtrPtr);
    {
      final reader = IsarCore.readerPtr;
      if (reader.isNull) {
        _data = null;
      } else {
        final list = List<int>.filled(length, 0, growable: true);
        for (var i = 0; i < length; i++) {
          list[i] = IsarCore.readByte(reader, i);
        }
        IsarCore.freeReader(reader);
        _data = list;
      }
    }
  }
  final DateTime _lastModified;
  {
    final value = IsarCore.readLong(reader, 4);
    if (value == -9223372036854775808) {
      _lastModified =
          DateTime.fromMillisecondsSinceEpoch(0, isUtc: true).toLocal();
    } else {
      _lastModified =
          DateTime.fromMicrosecondsSinceEpoch(value, isUtc: true).toLocal();
    }
  }
  final DateTime _lastChecked;
  {
    final value = IsarCore.readLong(reader, 5);
    if (value == -9223372036854775808) {
      _lastChecked =
          DateTime.fromMillisecondsSinceEpoch(0, isUtc: true).toLocal();
    } else {
      _lastChecked =
          DateTime.fromMicrosecondsSinceEpoch(value, isUtc: true).toLocal();
    }
  }
  final object = CachedEntity(
    oid: _oid,
    nid: _nid,
    data: _data,
    lastModified: _lastModified,
    lastChecked: _lastChecked,
  );
  return object;
}

@isarProtected
dynamic deserializeCachedEntityProp(IsarReader reader, int property) {
  switch (property) {
    case 1:
      return IsarCore.readString(reader, 1) ?? '';
    case 2:
      return IsarCore.readString(reader, 2) ?? '';
    case 3:
      {
        final length = IsarCore.readList(reader, 3, IsarCore.readerPtrPtr);
        {
          final reader = IsarCore.readerPtr;
          if (reader.isNull) {
            return null;
          } else {
            final list = List<int>.filled(length, 0, growable: true);
            for (var i = 0; i < length; i++) {
              list[i] = IsarCore.readByte(reader, i);
            }
            IsarCore.freeReader(reader);
            return list;
          }
        }
      }
    case 4:
      {
        final value = IsarCore.readLong(reader, 4);
        if (value == -9223372036854775808) {
          return DateTime.fromMillisecondsSinceEpoch(0, isUtc: true).toLocal();
        } else {
          return DateTime.fromMicrosecondsSinceEpoch(value, isUtc: true)
              .toLocal();
        }
      }
    case 5:
      {
        final value = IsarCore.readLong(reader, 5);
        if (value == -9223372036854775808) {
          return DateTime.fromMillisecondsSinceEpoch(0, isUtc: true).toLocal();
        } else {
          return DateTime.fromMicrosecondsSinceEpoch(value, isUtc: true)
              .toLocal();
        }
      }
    default:
      throw ArgumentError('Unknown property: $property');
  }
}

sealed class _CachedEntityUpdate {
  bool call({
    required String oid,
    String? nid,
    DateTime? lastModified,
    DateTime? lastChecked,
  });
}

class _CachedEntityUpdateImpl implements _CachedEntityUpdate {
  const _CachedEntityUpdateImpl(this.collection);

  final IsarCollection<String, CachedEntity> collection;

  @override
  bool call({
    required String oid,
    Object? nid = ignore,
    Object? lastModified = ignore,
    Object? lastChecked = ignore,
  }) {
    return collection.updateProperties([
          oid
        ], {
          if (nid != ignore) 2: nid as String?,
          if (lastModified != ignore) 4: lastModified as DateTime?,
          if (lastChecked != ignore) 5: lastChecked as DateTime?,
        }) >
        0;
  }
}

sealed class _CachedEntityUpdateAll {
  int call({
    required List<String> oid,
    String? nid,
    DateTime? lastModified,
    DateTime? lastChecked,
  });
}

class _CachedEntityUpdateAllImpl implements _CachedEntityUpdateAll {
  const _CachedEntityUpdateAllImpl(this.collection);

  final IsarCollection<String, CachedEntity> collection;

  @override
  int call({
    required List<String> oid,
    Object? nid = ignore,
    Object? lastModified = ignore,
    Object? lastChecked = ignore,
  }) {
    return collection.updateProperties(oid, {
      if (nid != ignore) 2: nid as String?,
      if (lastModified != ignore) 4: lastModified as DateTime?,
      if (lastChecked != ignore) 5: lastChecked as DateTime?,
    });
  }
}

extension CachedEntityUpdate on IsarCollection<String, CachedEntity> {
  _CachedEntityUpdate get update => _CachedEntityUpdateImpl(this);

  _CachedEntityUpdateAll get updateAll => _CachedEntityUpdateAllImpl(this);
}

sealed class _CachedEntityQueryUpdate {
  int call({
    String? nid,
    DateTime? lastModified,
    DateTime? lastChecked,
  });
}

class _CachedEntityQueryUpdateImpl implements _CachedEntityQueryUpdate {
  const _CachedEntityQueryUpdateImpl(this.query, {this.limit});

  final IsarQuery<CachedEntity> query;
  final int? limit;

  @override
  int call({
    Object? nid = ignore,
    Object? lastModified = ignore,
    Object? lastChecked = ignore,
  }) {
    return query.updateProperties(limit: limit, {
      if (nid != ignore) 2: nid as String?,
      if (lastModified != ignore) 4: lastModified as DateTime?,
      if (lastChecked != ignore) 5: lastChecked as DateTime?,
    });
  }
}

extension CachedEntityQueryUpdate on IsarQuery<CachedEntity> {
  _CachedEntityQueryUpdate get updateFirst =>
      _CachedEntityQueryUpdateImpl(this, limit: 1);

  _CachedEntityQueryUpdate get updateAll => _CachedEntityQueryUpdateImpl(this);
}

class _CachedEntityQueryBuilderUpdateImpl implements _CachedEntityQueryUpdate {
  const _CachedEntityQueryBuilderUpdateImpl(this.query, {this.limit});

  final QueryBuilder<CachedEntity, CachedEntity, QOperations> query;
  final int? limit;

  @override
  int call({
    Object? nid = ignore,
    Object? lastModified = ignore,
    Object? lastChecked = ignore,
  }) {
    final q = query.build();
    try {
      return q.updateProperties(limit: limit, {
        if (nid != ignore) 2: nid as String?,
        if (lastModified != ignore) 4: lastModified as DateTime?,
        if (lastChecked != ignore) 5: lastChecked as DateTime?,
      });
    } finally {
      q.close();
    }
  }
}

extension CachedEntityQueryBuilderUpdate
    on QueryBuilder<CachedEntity, CachedEntity, QOperations> {
  _CachedEntityQueryUpdate get updateFirst =>
      _CachedEntityQueryBuilderUpdateImpl(this, limit: 1);

  _CachedEntityQueryUpdate get updateAll =>
      _CachedEntityQueryBuilderUpdateImpl(this);
}

extension CachedEntityQueryFilter
    on QueryBuilder<CachedEntity, CachedEntity, QFilterCondition> {
  QueryBuilder<CachedEntity, CachedEntity, QAfterFilterCondition> oidEqualTo(
    String value, {
    bool caseSensitive = true,
  }) {
    return QueryBuilder.apply(this, (query) {
      return query.addFilterCondition(
        EqualCondition(
          property: 1,
          value: value,
          caseSensitive: caseSensitive,
        ),
      );
    });
  }

  QueryBuilder<CachedEntity, CachedEntity, QAfterFilterCondition>
      oidGreaterThan(
    String value, {
    bool caseSensitive = true,
  }) {
    return QueryBuilder.apply(this, (query) {
      return query.addFilterCondition(
        GreaterCondition(
          property: 1,
          value: value,
          caseSensitive: caseSensitive,
        ),
      );
    });
  }

  QueryBuilder<CachedEntity, CachedEntity, QAfterFilterCondition>
      oidGreaterThanOrEqualTo(
    String value, {
    bool caseSensitive = true,
  }) {
    return QueryBuilder.apply(this, (query) {
      return query.addFilterCondition(
        GreaterOrEqualCondition(
          property: 1,
          value: value,
          caseSensitive: caseSensitive,
        ),
      );
    });
  }

  QueryBuilder<CachedEntity, CachedEntity, QAfterFilterCondition> oidLessThan(
    String value, {
    bool caseSensitive = true,
  }) {
    return QueryBuilder.apply(this, (query) {
      return query.addFilterCondition(
        LessCondition(
          property: 1,
          value: value,
          caseSensitive: caseSensitive,
        ),
      );
    });
  }

  QueryBuilder<CachedEntity, CachedEntity, QAfterFilterCondition>
      oidLessThanOrEqualTo(
    String value, {
    bool caseSensitive = true,
  }) {
    return QueryBuilder.apply(this, (query) {
      return query.addFilterCondition(
        LessOrEqualCondition(
          property: 1,
          value: value,
          caseSensitive: caseSensitive,
        ),
      );
    });
  }

  QueryBuilder<CachedEntity, CachedEntity, QAfterFilterCondition> oidBetween(
    String lower,
    String upper, {
    bool caseSensitive = true,
  }) {
    return QueryBuilder.apply(this, (query) {
      return query.addFilterCondition(
        BetweenCondition(
          property: 1,
          lower: lower,
          upper: upper,
          caseSensitive: caseSensitive,
        ),
      );
    });
  }

  QueryBuilder<CachedEntity, CachedEntity, QAfterFilterCondition> oidStartsWith(
    String value, {
    bool caseSensitive = true,
  }) {
    return QueryBuilder.apply(this, (query) {
      return query.addFilterCondition(
        StartsWithCondition(
          property: 1,
          value: value,
          caseSensitive: caseSensitive,
        ),
      );
    });
  }

  QueryBuilder<CachedEntity, CachedEntity, QAfterFilterCondition> oidEndsWith(
    String value, {
    bool caseSensitive = true,
  }) {
    return QueryBuilder.apply(this, (query) {
      return query.addFilterCondition(
        EndsWithCondition(
          property: 1,
          value: value,
          caseSensitive: caseSensitive,
        ),
      );
    });
  }

  QueryBuilder<CachedEntity, CachedEntity, QAfterFilterCondition> oidContains(
      String value,
      {bool caseSensitive = true}) {
    return QueryBuilder.apply(this, (query) {
      return query.addFilterCondition(
        ContainsCondition(
          property: 1,
          value: value,
          caseSensitive: caseSensitive,
        ),
      );
    });
  }

  QueryBuilder<CachedEntity, CachedEntity, QAfterFilterCondition> oidMatches(
      String pattern,
      {bool caseSensitive = true}) {
    return QueryBuilder.apply(this, (query) {
      return query.addFilterCondition(
        MatchesCondition(
          property: 1,
          wildcard: pattern,
          caseSensitive: caseSensitive,
        ),
      );
    });
  }

  QueryBuilder<CachedEntity, CachedEntity, QAfterFilterCondition> oidIsEmpty() {
    return QueryBuilder.apply(this, (query) {
      return query.addFilterCondition(
        const EqualCondition(
          property: 1,
          value: '',
        ),
      );
    });
  }

  QueryBuilder<CachedEntity, CachedEntity, QAfterFilterCondition>
      oidIsNotEmpty() {
    return QueryBuilder.apply(this, (query) {
      return query.addFilterCondition(
        const GreaterCondition(
          property: 1,
          value: '',
        ),
      );
    });
  }

  QueryBuilder<CachedEntity, CachedEntity, QAfterFilterCondition> nidEqualTo(
    String value, {
    bool caseSensitive = true,
  }) {
    return QueryBuilder.apply(this, (query) {
      return query.addFilterCondition(
        EqualCondition(
          property: 2,
          value: value,
          caseSensitive: caseSensitive,
        ),
      );
    });
  }

  QueryBuilder<CachedEntity, CachedEntity, QAfterFilterCondition>
      nidGreaterThan(
    String value, {
    bool caseSensitive = true,
  }) {
    return QueryBuilder.apply(this, (query) {
      return query.addFilterCondition(
        GreaterCondition(
          property: 2,
          value: value,
          caseSensitive: caseSensitive,
        ),
      );
    });
  }

  QueryBuilder<CachedEntity, CachedEntity, QAfterFilterCondition>
      nidGreaterThanOrEqualTo(
    String value, {
    bool caseSensitive = true,
  }) {
    return QueryBuilder.apply(this, (query) {
      return query.addFilterCondition(
        GreaterOrEqualCondition(
          property: 2,
          value: value,
          caseSensitive: caseSensitive,
        ),
      );
    });
  }

  QueryBuilder<CachedEntity, CachedEntity, QAfterFilterCondition> nidLessThan(
    String value, {
    bool caseSensitive = true,
  }) {
    return QueryBuilder.apply(this, (query) {
      return query.addFilterCondition(
        LessCondition(
          property: 2,
          value: value,
          caseSensitive: caseSensitive,
        ),
      );
    });
  }

  QueryBuilder<CachedEntity, CachedEntity, QAfterFilterCondition>
      nidLessThanOrEqualTo(
    String value, {
    bool caseSensitive = true,
  }) {
    return QueryBuilder.apply(this, (query) {
      return query.addFilterCondition(
        LessOrEqualCondition(
          property: 2,
          value: value,
          caseSensitive: caseSensitive,
        ),
      );
    });
  }

  QueryBuilder<CachedEntity, CachedEntity, QAfterFilterCondition> nidBetween(
    String lower,
    String upper, {
    bool caseSensitive = true,
  }) {
    return QueryBuilder.apply(this, (query) {
      return query.addFilterCondition(
        BetweenCondition(
          property: 2,
          lower: lower,
          upper: upper,
          caseSensitive: caseSensitive,
        ),
      );
    });
  }

  QueryBuilder<CachedEntity, CachedEntity, QAfterFilterCondition> nidStartsWith(
    String value, {
    bool caseSensitive = true,
  }) {
    return QueryBuilder.apply(this, (query) {
      return query.addFilterCondition(
        StartsWithCondition(
          property: 2,
          value: value,
          caseSensitive: caseSensitive,
        ),
      );
    });
  }

  QueryBuilder<CachedEntity, CachedEntity, QAfterFilterCondition> nidEndsWith(
    String value, {
    bool caseSensitive = true,
  }) {
    return QueryBuilder.apply(this, (query) {
      return query.addFilterCondition(
        EndsWithCondition(
          property: 2,
          value: value,
          caseSensitive: caseSensitive,
        ),
      );
    });
  }

  QueryBuilder<CachedEntity, CachedEntity, QAfterFilterCondition> nidContains(
      String value,
      {bool caseSensitive = true}) {
    return QueryBuilder.apply(this, (query) {
      return query.addFilterCondition(
        ContainsCondition(
          property: 2,
          value: value,
          caseSensitive: caseSensitive,
        ),
      );
    });
  }

  QueryBuilder<CachedEntity, CachedEntity, QAfterFilterCondition> nidMatches(
      String pattern,
      {bool caseSensitive = true}) {
    return QueryBuilder.apply(this, (query) {
      return query.addFilterCondition(
        MatchesCondition(
          property: 2,
          wildcard: pattern,
          caseSensitive: caseSensitive,
        ),
      );
    });
  }

  QueryBuilder<CachedEntity, CachedEntity, QAfterFilterCondition> nidIsEmpty() {
    return QueryBuilder.apply(this, (query) {
      return query.addFilterCondition(
        const EqualCondition(
          property: 2,
          value: '',
        ),
      );
    });
  }

  QueryBuilder<CachedEntity, CachedEntity, QAfterFilterCondition>
      nidIsNotEmpty() {
    return QueryBuilder.apply(this, (query) {
      return query.addFilterCondition(
        const GreaterCondition(
          property: 2,
          value: '',
        ),
      );
    });
  }

  QueryBuilder<CachedEntity, CachedEntity, QAfterFilterCondition> dataIsNull() {
    return QueryBuilder.apply(this, (query) {
      return query.addFilterCondition(const IsNullCondition(property: 3));
    });
  }

  QueryBuilder<CachedEntity, CachedEntity, QAfterFilterCondition>
      dataIsNotNull() {
    return QueryBuilder.apply(not(), (query) {
      return query.addFilterCondition(const IsNullCondition(property: 3));
    });
  }

  QueryBuilder<CachedEntity, CachedEntity, QAfterFilterCondition>
      dataElementEqualTo(
    int value,
  ) {
    return QueryBuilder.apply(this, (query) {
      return query.addFilterCondition(
        EqualCondition(
          property: 3,
          value: value,
        ),
      );
    });
  }

  QueryBuilder<CachedEntity, CachedEntity, QAfterFilterCondition>
      dataElementGreaterThan(
    int value,
  ) {
    return QueryBuilder.apply(this, (query) {
      return query.addFilterCondition(
        GreaterCondition(
          property: 3,
          value: value,
        ),
      );
    });
  }

  QueryBuilder<CachedEntity, CachedEntity, QAfterFilterCondition>
      dataElementGreaterThanOrEqualTo(
    int value,
  ) {
    return QueryBuilder.apply(this, (query) {
      return query.addFilterCondition(
        GreaterOrEqualCondition(
          property: 3,
          value: value,
        ),
      );
    });
  }

  QueryBuilder<CachedEntity, CachedEntity, QAfterFilterCondition>
      dataElementLessThan(
    int value,
  ) {
    return QueryBuilder.apply(this, (query) {
      return query.addFilterCondition(
        LessCondition(
          property: 3,
          value: value,
        ),
      );
    });
  }

  QueryBuilder<CachedEntity, CachedEntity, QAfterFilterCondition>
      dataElementLessThanOrEqualTo(
    int value,
  ) {
    return QueryBuilder.apply(this, (query) {
      return query.addFilterCondition(
        LessOrEqualCondition(
          property: 3,
          value: value,
        ),
      );
    });
  }

  QueryBuilder<CachedEntity, CachedEntity, QAfterFilterCondition>
      dataElementBetween(
    int lower,
    int upper,
  ) {
    return QueryBuilder.apply(this, (query) {
      return query.addFilterCondition(
        BetweenCondition(
          property: 3,
          lower: lower,
          upper: upper,
        ),
      );
    });
  }

  QueryBuilder<CachedEntity, CachedEntity, QAfterFilterCondition>
      dataIsEmpty() {
    return not().group(
      (q) => q.dataIsNull().or().dataIsNotEmpty(),
    );
  }

  QueryBuilder<CachedEntity, CachedEntity, QAfterFilterCondition>
      dataIsNotEmpty() {
    return QueryBuilder.apply(this, (query) {
      return query.addFilterCondition(
        const GreaterOrEqualCondition(property: 3, value: null),
      );
    });
  }

  QueryBuilder<CachedEntity, CachedEntity, QAfterFilterCondition>
      lastModifiedEqualTo(
    DateTime value,
  ) {
    return QueryBuilder.apply(this, (query) {
      return query.addFilterCondition(
        EqualCondition(
          property: 4,
          value: value,
        ),
      );
    });
  }

  QueryBuilder<CachedEntity, CachedEntity, QAfterFilterCondition>
      lastModifiedGreaterThan(
    DateTime value,
  ) {
    return QueryBuilder.apply(this, (query) {
      return query.addFilterCondition(
        GreaterCondition(
          property: 4,
          value: value,
        ),
      );
    });
  }

  QueryBuilder<CachedEntity, CachedEntity, QAfterFilterCondition>
      lastModifiedGreaterThanOrEqualTo(
    DateTime value,
  ) {
    return QueryBuilder.apply(this, (query) {
      return query.addFilterCondition(
        GreaterOrEqualCondition(
          property: 4,
          value: value,
        ),
      );
    });
  }

  QueryBuilder<CachedEntity, CachedEntity, QAfterFilterCondition>
      lastModifiedLessThan(
    DateTime value,
  ) {
    return QueryBuilder.apply(this, (query) {
      return query.addFilterCondition(
        LessCondition(
          property: 4,
          value: value,
        ),
      );
    });
  }

  QueryBuilder<CachedEntity, CachedEntity, QAfterFilterCondition>
      lastModifiedLessThanOrEqualTo(
    DateTime value,
  ) {
    return QueryBuilder.apply(this, (query) {
      return query.addFilterCondition(
        LessOrEqualCondition(
          property: 4,
          value: value,
        ),
      );
    });
  }

  QueryBuilder<CachedEntity, CachedEntity, QAfterFilterCondition>
      lastModifiedBetween(
    DateTime lower,
    DateTime upper,
  ) {
    return QueryBuilder.apply(this, (query) {
      return query.addFilterCondition(
        BetweenCondition(
          property: 4,
          lower: lower,
          upper: upper,
        ),
      );
    });
  }

  QueryBuilder<CachedEntity, CachedEntity, QAfterFilterCondition>
      lastCheckedEqualTo(
    DateTime value,
  ) {
    return QueryBuilder.apply(this, (query) {
      return query.addFilterCondition(
        EqualCondition(
          property: 5,
          value: value,
        ),
      );
    });
  }

  QueryBuilder<CachedEntity, CachedEntity, QAfterFilterCondition>
      lastCheckedGreaterThan(
    DateTime value,
  ) {
    return QueryBuilder.apply(this, (query) {
      return query.addFilterCondition(
        GreaterCondition(
          property: 5,
          value: value,
        ),
      );
    });
  }

  QueryBuilder<CachedEntity, CachedEntity, QAfterFilterCondition>
      lastCheckedGreaterThanOrEqualTo(
    DateTime value,
  ) {
    return QueryBuilder.apply(this, (query) {
      return query.addFilterCondition(
        GreaterOrEqualCondition(
          property: 5,
          value: value,
        ),
      );
    });
  }

  QueryBuilder<CachedEntity, CachedEntity, QAfterFilterCondition>
      lastCheckedLessThan(
    DateTime value,
  ) {
    return QueryBuilder.apply(this, (query) {
      return query.addFilterCondition(
        LessCondition(
          property: 5,
          value: value,
        ),
      );
    });
  }

  QueryBuilder<CachedEntity, CachedEntity, QAfterFilterCondition>
      lastCheckedLessThanOrEqualTo(
    DateTime value,
  ) {
    return QueryBuilder.apply(this, (query) {
      return query.addFilterCondition(
        LessOrEqualCondition(
          property: 5,
          value: value,
        ),
      );
    });
  }

  QueryBuilder<CachedEntity, CachedEntity, QAfterFilterCondition>
      lastCheckedBetween(
    DateTime lower,
    DateTime upper,
  ) {
    return QueryBuilder.apply(this, (query) {
      return query.addFilterCondition(
        BetweenCondition(
          property: 5,
          lower: lower,
          upper: upper,
        ),
      );
    });
  }
}

extension CachedEntityQueryObject
    on QueryBuilder<CachedEntity, CachedEntity, QFilterCondition> {}

extension CachedEntityQuerySortBy
    on QueryBuilder<CachedEntity, CachedEntity, QSortBy> {
  QueryBuilder<CachedEntity, CachedEntity, QAfterSortBy> sortByOid(
      {bool caseSensitive = true}) {
    return QueryBuilder.apply(this, (query) {
      return query.addSortBy(
        1,
        caseSensitive: caseSensitive,
      );
    });
  }

  QueryBuilder<CachedEntity, CachedEntity, QAfterSortBy> sortByOidDesc(
      {bool caseSensitive = true}) {
    return QueryBuilder.apply(this, (query) {
      return query.addSortBy(
        1,
        sort: Sort.desc,
        caseSensitive: caseSensitive,
      );
    });
  }

  QueryBuilder<CachedEntity, CachedEntity, QAfterSortBy> sortByNid(
      {bool caseSensitive = true}) {
    return QueryBuilder.apply(this, (query) {
      return query.addSortBy(
        2,
        caseSensitive: caseSensitive,
      );
    });
  }

  QueryBuilder<CachedEntity, CachedEntity, QAfterSortBy> sortByNidDesc(
      {bool caseSensitive = true}) {
    return QueryBuilder.apply(this, (query) {
      return query.addSortBy(
        2,
        sort: Sort.desc,
        caseSensitive: caseSensitive,
      );
    });
  }

  QueryBuilder<CachedEntity, CachedEntity, QAfterSortBy> sortByLastModified() {
    return QueryBuilder.apply(this, (query) {
      return query.addSortBy(4);
    });
  }

  QueryBuilder<CachedEntity, CachedEntity, QAfterSortBy>
      sortByLastModifiedDesc() {
    return QueryBuilder.apply(this, (query) {
      return query.addSortBy(4, sort: Sort.desc);
    });
  }

  QueryBuilder<CachedEntity, CachedEntity, QAfterSortBy> sortByLastChecked() {
    return QueryBuilder.apply(this, (query) {
      return query.addSortBy(5);
    });
  }

  QueryBuilder<CachedEntity, CachedEntity, QAfterSortBy>
      sortByLastCheckedDesc() {
    return QueryBuilder.apply(this, (query) {
      return query.addSortBy(5, sort: Sort.desc);
    });
  }
}

extension CachedEntityQuerySortThenBy
    on QueryBuilder<CachedEntity, CachedEntity, QSortThenBy> {
  QueryBuilder<CachedEntity, CachedEntity, QAfterSortBy> thenByOid(
      {bool caseSensitive = true}) {
    return QueryBuilder.apply(this, (query) {
      return query.addSortBy(1, caseSensitive: caseSensitive);
    });
  }

  QueryBuilder<CachedEntity, CachedEntity, QAfterSortBy> thenByOidDesc(
      {bool caseSensitive = true}) {
    return QueryBuilder.apply(this, (query) {
      return query.addSortBy(1, sort: Sort.desc, caseSensitive: caseSensitive);
    });
  }

  QueryBuilder<CachedEntity, CachedEntity, QAfterSortBy> thenByNid(
      {bool caseSensitive = true}) {
    return QueryBuilder.apply(this, (query) {
      return query.addSortBy(2, caseSensitive: caseSensitive);
    });
  }

  QueryBuilder<CachedEntity, CachedEntity, QAfterSortBy> thenByNidDesc(
      {bool caseSensitive = true}) {
    return QueryBuilder.apply(this, (query) {
      return query.addSortBy(2, sort: Sort.desc, caseSensitive: caseSensitive);
    });
  }

  QueryBuilder<CachedEntity, CachedEntity, QAfterSortBy> thenByLastModified() {
    return QueryBuilder.apply(this, (query) {
      return query.addSortBy(4);
    });
  }

  QueryBuilder<CachedEntity, CachedEntity, QAfterSortBy>
      thenByLastModifiedDesc() {
    return QueryBuilder.apply(this, (query) {
      return query.addSortBy(4, sort: Sort.desc);
    });
  }

  QueryBuilder<CachedEntity, CachedEntity, QAfterSortBy> thenByLastChecked() {
    return QueryBuilder.apply(this, (query) {
      return query.addSortBy(5);
    });
  }

  QueryBuilder<CachedEntity, CachedEntity, QAfterSortBy>
      thenByLastCheckedDesc() {
    return QueryBuilder.apply(this, (query) {
      return query.addSortBy(5, sort: Sort.desc);
    });
  }
}

extension CachedEntityQueryWhereDistinct
    on QueryBuilder<CachedEntity, CachedEntity, QDistinct> {
  QueryBuilder<CachedEntity, CachedEntity, QAfterDistinct> distinctByNid(
      {bool caseSensitive = true}) {
    return QueryBuilder.apply(this, (query) {
      return query.addDistinctBy(2, caseSensitive: caseSensitive);
    });
  }

  QueryBuilder<CachedEntity, CachedEntity, QAfterDistinct> distinctByData() {
    return QueryBuilder.apply(this, (query) {
      return query.addDistinctBy(3);
    });
  }

  QueryBuilder<CachedEntity, CachedEntity, QAfterDistinct>
      distinctByLastModified() {
    return QueryBuilder.apply(this, (query) {
      return query.addDistinctBy(4);
    });
  }

  QueryBuilder<CachedEntity, CachedEntity, QAfterDistinct>
      distinctByLastChecked() {
    return QueryBuilder.apply(this, (query) {
      return query.addDistinctBy(5);
    });
  }
}

extension CachedEntityQueryProperty1
    on QueryBuilder<CachedEntity, CachedEntity, QProperty> {
  QueryBuilder<CachedEntity, String, QAfterProperty> oidProperty() {
    return QueryBuilder.apply(this, (query) {
      return query.addProperty(1);
    });
  }

  QueryBuilder<CachedEntity, String, QAfterProperty> nidProperty() {
    return QueryBuilder.apply(this, (query) {
      return query.addProperty(2);
    });
  }

  QueryBuilder<CachedEntity, List<int>?, QAfterProperty> dataProperty() {
    return QueryBuilder.apply(this, (query) {
      return query.addProperty(3);
    });
  }

  QueryBuilder<CachedEntity, DateTime, QAfterProperty> lastModifiedProperty() {
    return QueryBuilder.apply(this, (query) {
      return query.addProperty(4);
    });
  }

  QueryBuilder<CachedEntity, DateTime, QAfterProperty> lastCheckedProperty() {
    return QueryBuilder.apply(this, (query) {
      return query.addProperty(5);
    });
  }
}

extension CachedEntityQueryProperty2<R>
    on QueryBuilder<CachedEntity, R, QAfterProperty> {
  QueryBuilder<CachedEntity, (R, String), QAfterProperty> oidProperty() {
    return QueryBuilder.apply(this, (query) {
      return query.addProperty(1);
    });
  }

  QueryBuilder<CachedEntity, (R, String), QAfterProperty> nidProperty() {
    return QueryBuilder.apply(this, (query) {
      return query.addProperty(2);
    });
  }

  QueryBuilder<CachedEntity, (R, List<int>?), QAfterProperty> dataProperty() {
    return QueryBuilder.apply(this, (query) {
      return query.addProperty(3);
    });
  }

  QueryBuilder<CachedEntity, (R, DateTime), QAfterProperty>
      lastModifiedProperty() {
    return QueryBuilder.apply(this, (query) {
      return query.addProperty(4);
    });
  }

  QueryBuilder<CachedEntity, (R, DateTime), QAfterProperty>
      lastCheckedProperty() {
    return QueryBuilder.apply(this, (query) {
      return query.addProperty(5);
    });
  }
}

extension CachedEntityQueryProperty3<R1, R2>
    on QueryBuilder<CachedEntity, (R1, R2), QAfterProperty> {
  QueryBuilder<CachedEntity, (R1, R2, String), QOperations> oidProperty() {
    return QueryBuilder.apply(this, (query) {
      return query.addProperty(1);
    });
  }

  QueryBuilder<CachedEntity, (R1, R2, String), QOperations> nidProperty() {
    return QueryBuilder.apply(this, (query) {
      return query.addProperty(2);
    });
  }

  QueryBuilder<CachedEntity, (R1, R2, List<int>?), QOperations> dataProperty() {
    return QueryBuilder.apply(this, (query) {
      return query.addProperty(3);
    });
  }

  QueryBuilder<CachedEntity, (R1, R2, DateTime), QOperations>
      lastModifiedProperty() {
    return QueryBuilder.apply(this, (query) {
      return query.addProperty(4);
    });
  }

  QueryBuilder<CachedEntity, (R1, R2, DateTime), QOperations>
      lastCheckedProperty() {
    return QueryBuilder.apply(this, (query) {
      return query.addProperty(5);
    });
  }
}
