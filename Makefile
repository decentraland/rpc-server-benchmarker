PROTOBUF_VERSION = 3.19.1
PROTOC ?= protoc
UNAME := $(shell uname)
PROTO_FILES := $(wildcard src/*.proto)
export PATH := ts-server/node_modules/.bin:/usr/local/include/:protoc3/bin:$(PATH)

ifeq ($(UNAME),Darwin)
PROTOBUF_ZIP = protoc-$(PROTOBUF_VERSION)-osx-x86_64.zip
else
PROTOBUF_ZIP = protoc-$(PROTOBUF_VERSION)-linux-x86_64.zip
endif

# for ts
install_compiler:
	@# remove local folder
	rm -rf protoc3 || true

	@# Make sure you grab the latest version
	curl -OL https://github.com/protocolbuffers/protobuf/releases/download/v$(PROTOBUF_VERSION)/$(PROTOBUF_ZIP)

	@# Unzip
	unzip $(PROTOBUF_ZIP) -d protoc3
	@# delete the files
	rm $(PROTOBUF_ZIP)

	@# move protoc to /usr/local/bin/
	chmod +x protoc3/bin/protoc

build-ts: install_compiler
	@cd ts-server && npm i
	@cd benchmarker && npm i
	@chmod +x ./build-ts.sh
	./build-ts.sh


build-rs:
	@cd rs-server && cargo build

run-rs:
	@cd rs-server && cargo run &
	sleep 8;
	@cd benchmarker-rs && cargo run

ab:
ifdef n
ifdef c
ifdef server
ifeq ($(server), rs)
	@cd rs-server && cargo build && cargo run &
	sleep 8;
	@cd benchmarker-ts && npx ts-node index.ts -n $(n) -c $(c) -r server-$(server)-$(n)-$(c)
	@ps aux | grep "cargo run" | awk '{print $$2}' | tail -1 | xargs kill -9 
else
	@cd ts-server && npx ts-node index.ts &
	sleep 8;
	@cd benchmarker-ts && npx ts-node index.ts -n $(n) -c $(c) -r server-$(server)-$(n)-$(c)
	@ps aux | grep "npx ts-node" | awk '{print $$2}' | tail -1 | xargs kill -9 
endif
else
	@echo "Error: Server to run not defined"
	@exit 1;
endif
else
	@echo "Error: Number of requests not defined"
	@exit 1;
endif
else
	@echo "Error: Concurrency level not defined"
	@exit 1;
endif