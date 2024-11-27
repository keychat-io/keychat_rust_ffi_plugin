rust = ${PWD}/rust
$(info ${rust})

# --full-dep for build ios .h file
g2:
	cd ${rust}; flutter_rust_bridge_codegen -v generate --dart-format-line-length 120 --no-deps-check --no-web\
    --rust-input crate::api_cashu,crate::api_nostr,crate::api_signal,crate::api_mls \
    --rust-root . \
    --rust-output src/gen.rs \
    --dart-output ../lib/

 