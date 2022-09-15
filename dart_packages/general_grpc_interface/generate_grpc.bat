protoc -I../../protocols    -I../../account_server/protocols --dart_out=grpc:lib/grpc_generated  ../../account_server/protocols/account_service.proto
protoc -I../../protocols    -I../../account_server/protocols --dart_out=lib/grpc_generated       ../../account_server/protocols/account.proto
protoc -I../../protocols    -I../../account_server/protocols --dart_out=lib/grpc_generated       ../../account_server/protocols/login.proto
protoc -I../../protocols    -I../../account_server/protocols --dart_out=lib/grpc_generated       ../../account_server/protocols/password.proto
protoc -I../../protocols    -I../../account_server/protocols --dart_out=lib/grpc_generated       ../../account_server/protocols/status.proto

protoc -I../../protocols  --dart_out=lib/grpc_generated  ../../protocols/language_code.proto
protoc -I../../protocols  --dart_out=lib/grpc_generated  ../../protocols/manage.proto
protoc -I../../protocols  --dart_out=lib/grpc_generated  ../../protocols/manage_schema.proto
protoc -I../../protocols  --dart_out=lib/grpc_generated  ../../protocols/group.proto
protoc -I../../protocols  --dart_out=lib/grpc_generated  ../../protocols/name.proto
protoc -I../../protocols  --dart_out=lib/grpc_generated  ../../protocols/comment.proto
protoc -I../../protocols  --dart_out=lib/grpc_generated  ../../protocols/area.proto
protoc -I../../protocols  --dart_out=lib/grpc_generated  ../../protocols/position.proto
protoc -I../../protocols  --dart_out=lib/grpc_generated  ../../protocols/season.proto
protoc -I../../protocols  --dart_out=lib/grpc_generated  ../../protocols/color.proto
protoc -I../../protocols  --dart_out=lib/grpc_generated  ../../protocols/data.proto
protoc -I../../protocols  --dart_out=lib/grpc_generated  ../../protocols/file_data.proto
protoc -I../../protocols  --dart_out=lib/grpc_generated  ../../protocols/file_info.proto
protoc -I../../protocols  --dart_out=lib/grpc_generated  ../../protocols/sequence_data.proto
protoc -I../../protocols  --dart_out=lib/grpc_generated  ../../protocols/set_data.proto
protoc -I../../protocols  --dart_out=lib/grpc_generated  ../../protocols/set_data_info.proto
protoc -I../../protocols  --dart_out=lib/grpc_generated  ../../protocols/entity.proto
protoc -I../../protocols  --dart_out=lib/grpc_generated  ../../protocols/entity_template.proto

