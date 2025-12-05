# Space-separated list of all targets
BINARY := "ppp"
TARGETS := "x86_64-unknown-linux-gnu.2.17 \
            i686-unknown-linux-gnu.2.17 \
            aarch64-unknown-linux-gnu.2.17 \
            armv7-unknown-linux-gnueabihf.2.17 \
            x86_64-unknown-linux-musl \
            i686-unknown-linux-musl \
            aarch64-unknown-linux-musl \
            armv7-unknown-linux-musleabihf"

deps:
    cargo install --locked cargo-zigbuild
    for target in {{TARGETS}}; do \
        rustup target add "${target%.2.17}"; \
    done

release:
    for target in {{TARGETS}}; do \
        cargo zigbuild --release --target "$target"; \
    done

dist: release
    mkdir -p dist
    for target in {{TARGETS}}; do \
        target="${target%.2.17}"; \
        outfile="dist/{{BINARY}}-$target"; \
        cp target/"$target"/release/{{BINARY}} "$outfile"; \
        xz "$outfile"; \
    done

clean:
    rm -rf dist
    cargo clean
