import 'dart:developer';
import 'dart:io';
import 'package:flutter_test/flutter_test.dart';
import 'package:general_grpc_interface/grpc_generated/login.pb.dart';
import 'package:imix_grpc_interface/client.dart';
import 'package:flutter_dotenv/flutter_dotenv.dart';
import 'package:bson/bson.dart';
import 'package:imixapp/inject_configure.dart';

Future<void> main() async {
  const host = 'localhost';
  const port = 8800;

  final LoginRequest request = LoginRequest(countryCode: "86", phone: "10000000000", password: "root");

  dotenv.testLoad(fileInput: File('configs/develop.env').readAsStringSync());

  configureDependencies();
  // final serverConfigs = getIt.get<ServerConfiguration>();
  final client = getIt.get<IMixClient>();

  final bson = BSON();

  try {
    final LoginResponse response = await client.accountStub.login(request);

    log(response.toString());
    log(bson.deserialize(BsonBinary.from(response.person)).toString());

    test("测试初始值", () {
      expect(client.host, host);
      expect(client.port, port);
    });

    test("测试登录", () {
      expect(response.token, isNotNull);
    });
  } catch (e) {
    log(e.toString());
  }
}
