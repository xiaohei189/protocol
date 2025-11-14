# protocol

Protocol Buffers definitions for multi-language support (Go, TypeScript, Rust, etc.)

## Quick Start

### Go
```shell
go install google.golang.org/protobuf/cmd/protoc-gen-go@latest
go install google.golang.org/grpc/cmd/protoc-gen-go-grpc@latest
mage GenGo  # or: mage go
```

### TypeScript
```shell
npm install ts-proto
mage GenTypeScript  # or: mage ts
```

### Rust
```shell
cargo install protoc-gen-prost
mage GenRust  # or: mage rust
```

More to read [regenerate-grpc-code](https://grpc.io/docs/languages/go/quickstart/#regenerate-grpc-code).

## Documentation

- [Generate Protocol Buffers with Mage](./mage-README.md)
