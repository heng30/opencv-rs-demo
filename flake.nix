{
  description = "A Nix-flake-based OpenCV development environment";

  inputs = { nixpkgs.url = "github:nixos/nixpkgs/nixos-25.05"; };

  outputs = { self, nixpkgs, ... }:
    let system = "x86_64-linux";
    in {
      devShells."${system}".default =
        let pkgs = import nixpkgs { inherit system; };
        in pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            pkg-config
            rustc
            cargo
            gcc
            gnumake
            clang
            llvm
            cmake
          ];

          buildInputs = with pkgs; [
            gtk2
            gtk3
            libclang
            glib
            pango
            cairo
            gdk-pixbuf
            harfbuzz
            ffmpeg
            libwebp
            libjpeg
            libpng
            libtiff

            (opencv.override {
              enableGtk2 = true;
              enableGtk3 = true;
              enableFfmpeg = true;
              enableVtk = false;
              enableUnfree = false;
              enablePython = false;
              enableContrib = false;
            })
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
            echo "OpenCV build info: $(pkg-config --cflags opencv4 || pkg-config --cflags opencv)"
          '';
        };
    };
}
