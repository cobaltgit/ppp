BINARY := "ppp"
TARGET_GLIBC := "2.17"

targets := replace(
    "x86_64-unknown-linux-gnu.GLIBC \
    i686-unknown-linux-gnu.GLIBC \
    aarch64-unknown-linux-gnu.GLIBC \
    armv7-unknown-linux-gnueabihf.GLIBC \
    x86_64-unknown-linux-musl \
    i686-unknown-linux-musl \
    aarch64-unknown-linux-musl \
    armv7-unknown-linux-musleabihf",
    "GLIBC",
    TARGET_GLIBC
)

deps:
    cargo install --locked cargo-zigbuild
    for target in {{targets}}; do \
        rustup target add "${target%.{{TARGET_GLIBC}}}"; \
    done

release:
    for target in {{targets}}; do \
        cargo zigbuild --release --target "$target"; \
    done

dist: release
    mkdir -p dist
    for target in {{targets}}; do \
        target="${target%.{{TARGET_GLIBC}}}"; \
        outfile="dist/{{BINARY}}-$target"; \
        cp target/"$target"/release/{{BINARY}} "$outfile"; \
        xz "$outfile"; \
    done

clean:
    rm -rf dist
    cargo clean
