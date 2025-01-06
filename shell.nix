{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  # Define the native build inputs needed for your project
  nativeBuildInputs = with pkgs; [
    rustc        # Rust compiler
    cargo        # Rust package manager
    rustfmt      # Rust code formatter
    clippy       # Rust linter
    gcc          # Required for crates needing C compilers
    pkg-config   # Helps locate libraries like OpenSSL
    openssl      # OpenSSL library for crates like openssl-sys
    postgresql   # PostgreSQL for database access and libpq
    docker       # Docker for containerized development
    nodejs       # Node.js for potential frontend tasks
    yarn         # Yarn package manager for React
  ];

  # Set the source path for Rust tooling (e.g., rust-analyzer)
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";

  # Optional shellHook to export additional environment variables
  shellHook = ''
    # Export the PKG_CONFIG_PATH to ensure pkg-config works correctly with OpenSSL
    export PKG_CONFIG_PATH="${pkgs.openssl.dev}/lib/pkgconfig:$PKG_CONFIG_PATH"

    # PostgreSQL setup
    export PATH="${pkgs.postgresql}/bin:$PATH"
    export PGDATA="$HOME/postgres-data"

    # Rust environment
    export RUST_BACKTRACE=1
    export CARGO_HOME=$HOME/.cargo
    export PATH=$CARGO_HOME/bin:$PATH

    echo "Environment setup complete. You are now in the NoteNest development shell!"
  '';
}
