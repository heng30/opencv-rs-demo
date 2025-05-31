{ pkgs ? import <nixpkgs> { } }:

pkgs.mkShell {
  nativeBuildInputs = [
    pkgs.pkg-config
    pkgs.rustc
    pkgs.cargo
    pkgs.gcc
    pkgs.gnumake
    pkgs.clang
    pkgs.llvm
  ];

  buildInputs = [ pkgs.opencv pkgs.libclang ];

  env = {
    LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";

    BINDGEN_EXTRA_CLANG_ARGS =
      "-isystem ${pkgs.llvmPackages.libclang.lib}/lib/clang/${pkgs.llvmPackages.clang.version}/include";

    PKG_CONFIG_PATH = "${pkgs.opencv}/lib/pkgconfig";

    CPLUS_INCLUDE_PATH =
      "${pkgs.gcc-unwrapped}/include/c++/${pkgs.gcc.version}";
  };

  shellHook = ''
    echo "Rust development environment with OpenCV support is ready!"
    echo "Rust version: $(rustc --version)"
    echo "Clang version: ${pkgs.llvmPackages.clang.version}"
    echo "OpenCV version: $(pkg-config --modversion opencv4 || pkg-config --modversion opencv)"
  '';
}
