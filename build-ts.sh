./protoc3/bin/protoc \
		--plugin=./ts-server/node_modules/.bin/protoc-gen-ts_proto \
		--ts_proto_opt=esModuleInterop=true,returnObservable=false,outputServices=generic-definitions \
		--ts_proto_out="$(pwd)/ts-server" -I="$(pwd)" \
		"$(pwd)/api.proto"

cp $(pwd)/ts-server/api.ts $(pwd)/benchmarker-ts/api.ts