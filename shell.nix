let
  pkgs = import <nixpkgs> {};
  rust-toolchain = pkgs.symlinkJoin {
    name = "rust-toolchain";
    paths = [
      pkgs.rustfmt 
      pkgs.rustc 
      pkgs.cargo 
      pkgs.cargo-watch
      pkgs.rustPlatform.rustcSrc 
      pkgs.clippy
    ];
  };
in with pkgs;

mkShell {
  name = "ella";
  buildInputs = [
    clang_14
    lld_14
    cmake
    libiconv
    openssl
    pkg-config
    rust-analyzer
    rust-toolchain
    go
  ] ++ 
  lib.optionals (!stdenv.isDarwin) [
    procps
  ] ++
  lib.optionals (stdenv.isDarwin) [
    darwin.apple_sdk.frameworks.Security
    darwin.apple_sdk.frameworks.SystemConfiguration
    darwin.apple_sdk.frameworks.CoreServices
    darwin.libobjc
  ]
  ;

  NIX_ENFORCE_PURITY = 0;
  RUST_BACKTRACE = "full";
  RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
  CARGO_PROFILE_DEV_BUILD_OVERRIDE_DEBUG = "true";
  #RUSTFLAGS = "-Clinker=clang -Clink-arg=--ld-path=${pkgs.mold}/bin/mold -Clink-arg=-Wl,--warn-unresolved-symbols -Cdebuginfo=1 -Csymbol-mangling-version=v0 --cfg=tokio_usntable";

  RUSTFLAGS = "-Clinker=clang -Clink-arg=-Wl,-undefined,dynamic_lookup -Cdebuginfo=1 -Csymbol-mangling-version=v0";

  shellHook = ''
    export LD_LIBRARY_PATH=${pkgs.stdenv.cc.cc.lib}/lib:$LD_LIBRARY_PATH
    export LIBCLANG_PATH="${llvmPackages_14.libclang.lib}/lib";

    # unset NIX_CC
    export GOPATH=`pwd`/go
    export GO111MODULE=on
    export CGO_ENABLED=1

    mkdir -p $GOPATH/bin
    export PATH=$GOPATH/bin:$PATH
    #go install "github.com/prometheus/prom2json/cmd/prom2json@latest"
    #export PROM2JSON_PATH=$GOPATH/bin/prom2json
  '';

}

