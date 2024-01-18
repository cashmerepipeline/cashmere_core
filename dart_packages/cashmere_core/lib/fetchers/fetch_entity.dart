import 'dart:async';
import 'package:bson/bson.dart';
import 'package:cashmere_core/fetchers/fetch_entity_arg.dart';
import 'package:cashmere_core/protocols/entity.pb.dart';
import 'package:grpc/grpc.dart';

Future<Map<String, dynamic>> fetchEntity(FetchEntityArg arg) async {
    final request = GetEntityRequest()..entityId = arg.entityId;
    request.manageId = arg.manageId;
    
    final response = await arg.getEntityCall(request, options: CallOptions(metadata: arg.metadata));
    
    final entity = BsonCodec.deserialize(BsonBinary.from(response.entity));
    return entity;
  }