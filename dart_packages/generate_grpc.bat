protoc -Iprotocols -Iaccount_server/protocols --dart_out=grpc:dart_packages/general_grpc_interface/lib/grpc_generated  account_server/protocols/account_service.proto
protoc -Iprotocols -Iaccount_server/protocols --dart_out=dart_packages/general_grpc_interface/lib/grpc_generated  account_server/protocols/account.proto
protoc -Iprotocols -Iaccount_server/protocols --dart_out=dart_packages/general_grpc_interface/lib/grpc_generated  account_server/protocols/login.proto
protoc -Iprotocols -Iaccount_server/protocols --dart_out=dart_packages/general_grpc_interface/lib/grpc_generated  account_server/protocols/password.proto
protoc -Iprotocols -Iaccount_server/protocols --dart_out=dart_packages/general_grpc_interface/lib/grpc_generated  account_server/protocols/status.proto

protoc -Iprotocols  --dart_out=dart_packages/general_grpc_interface/lib/grpc_generated  protocols/manage.proto
protoc -Iprotocols  --dart_out=dart_packages/general_grpc_interface/lib/grpc_generated  protocols/name.proto
protoc -Iprotocols  --dart_out=dart_packages/general_grpc_interface/lib/grpc_generated  protocols/comment.proto
protoc -Iprotocols  --dart_out=dart_packages/general_grpc_interface/lib/grpc_generated  protocols/area.proto
protoc -Iprotocols  --dart_out=dart_packages/general_grpc_interface/lib/grpc_generated  protocols/position.proto
protoc -Iprotocols  --dart_out=dart_packages/general_grpc_interface/lib/grpc_generated  protocols/season.proto
protoc -Iprotocols  --dart_out=dart_packages/general_grpc_interface/lib/grpc_generated  protocols/color.proto

