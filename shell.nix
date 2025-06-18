{ pkgs ? import <nixpkgs> { } }:

pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    pkg-config
    rustc
    cargo
    gcc
    gnumake
    clang
    llvm
  ];

  buildInputs = with pkgs; [
    (opencv.override {
      enableGtk3 = true;
      enableUnfree = true;
    })
    libclang
  ];

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
