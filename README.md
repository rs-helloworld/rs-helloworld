# rs-helloworld

There is 3rd-party dependency mbedTLS. Concern: Spectre-safety of parsing of externally provided binary structures, such as PEM-encoded certificate/private key and TLS messages

Dependencies:
* zlib-devel, cmake
* Works with llvm 3.8, doesn't work with llvm 5
