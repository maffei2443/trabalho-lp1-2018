static BUILTINS: phf::Map<&'static str, TargetInfo> = ::phf::Map {
    key: 1897749892740154578,
    disps: ::phf::Slice::Static(&[
        (6, 16),
        (0, 15),
        (0, 0),
        (2, 28),
        (1, 0),
        (1, 2),
        (1, 40),
        (11, 31),
        (6, 21),
        (0, 44),
        (19, 65),
        (0, 38),
        (8, 23),
        (0, 23),
        (28, 67),
    ]),
    entries: ::phf::Slice::Static(&[
        ("x86_64-unknown-redox", TargetInfo { arch: B("x86_64"), os: B("redox"), env: B(""), endian: B("little"), pointer_width: B("64"), switches: B(&[]), other_keys: B(&[]) }),
        ("arm-linux-androideabi", TargetInfo { arch: B("arm"), os: B("android"), env: B(""), endian: B("little"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), ]) }),
        ("i686-unknown-haiku", TargetInfo { arch: B("x86"), os: B("haiku"), env: B(""), endian: B("little"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), ]) }),
        ("armv4t-unknown-linux-gnueabi", TargetInfo { arch: B("arm"), os: B("linux"), env: B("gnu"), endian: B("little"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), ]) }),
        ("thumbv7m-none-eabi", TargetInfo { arch: B("arm"), os: B("none"), env: B(""), endian: B("little"), pointer_width: B("32"), switches: B(&[]), other_keys: B(&[]) }),
        ("aarch64-unknown-fuchsia", TargetInfo { arch: B("aarch64"), os: B("fuchsia"), env: B(""), endian: B("little"), pointer_width: B("64"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), ]) }),
        ("mips-unknown-linux-gnu", TargetInfo { arch: B("mips"), os: B("linux"), env: B("gnu"), endian: B("big"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), ]) }),
        ("x86_64-unknown-dragonfly", TargetInfo { arch: B("x86_64"), os: B("dragonfly"), env: B(""), endian: B("little"), pointer_width: B("64"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), ]) }),
        ("aarch64-linux-android", TargetInfo { arch: B("aarch64"), os: B("android"), env: B(""), endian: B("little"), pointer_width: B("64"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), ]) }),
        ("i686-unknown-netbsd", TargetInfo { arch: B("x86"), os: B("netbsd"), env: B(""), endian: B("little"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), ]) }),
        ("x86_64-unknown-fuchsia", TargetInfo { arch: B("x86_64"), os: B("fuchsia"), env: B(""), endian: B("little"), pointer_width: B("64"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), ]) }),
        ("i586-pc-windows-msvc", TargetInfo { arch: B("x86"), os: B("windows"), env: B("msvc"), endian: B("little"), pointer_width: B("32"), switches: B(&[B("windows"), ]), other_keys: B(&[(B("target_family"), B("windows")), ]) }),
        ("aarch64-unknown-linux-musl", TargetInfo { arch: B("aarch64"), os: B("linux"), env: B("musl"), endian: B("little"), pointer_width: B("64"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_feature"), B("crt-static")), ]) }),
        ("x86_64-pc-windows-msvc", TargetInfo { arch: B("x86_64"), os: B("windows"), env: B("msvc"), endian: B("little"), pointer_width: B("64"), switches: B(&[B("windows"), ]), other_keys: B(&[(B("target_family"), B("windows")), ]) }),
        ("x86_64-linux-android", TargetInfo { arch: B("x86_64"), os: B("android"), env: B(""), endian: B("little"), pointer_width: B("64"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), ]) }),
        ("x86_64-unknown-linux-musl", TargetInfo { arch: B("x86_64"), os: B("linux"), env: B("musl"), endian: B("little"), pointer_width: B("64"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_feature"), B("crt-static")), ]) }),
        ("i686-apple-darwin", TargetInfo { arch: B("x86"), os: B("macos"), env: B(""), endian: B("little"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), ]) }),
        ("aarch64-unknown-cloudabi", TargetInfo { arch: B("aarch64"), os: B("cloudabi"), env: B(""), endian: B("little"), pointer_width: B("64"), switches: B(&[]), other_keys: B(&[]) }),
        ("x86_64-unknown-bitrig", TargetInfo { arch: B("x86_64"), os: B("bitrig"), env: B(""), endian: B("little"), pointer_width: B("64"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), ]) }),
        ("wasm32-unknown-emscripten", TargetInfo { arch: B("wasm32"), os: B("emscripten"), env: B(""), endian: B("little"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), ]) }),
        ("thumbv7em-none-eabihf", TargetInfo { arch: B("arm"), os: B("none"), env: B(""), endian: B("little"), pointer_width: B("32"), switches: B(&[]), other_keys: B(&[]) }),
        ("wasm32-unknown-unknown", TargetInfo { arch: B("wasm32"), os: B("unknown"), env: B(""), endian: B("little"), pointer_width: B("32"), switches: B(&[]), other_keys: B(&[]) }),
        ("i686-unknown-linux-gnu", TargetInfo { arch: B("x86"), os: B("linux"), env: B("gnu"), endian: B("little"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), ]) }),
        ("s390x-unknown-linux-gnu", TargetInfo { arch: B("s390x"), os: B("linux"), env: B("gnu"), endian: B("big"), pointer_width: B("64"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), ]) }),
        ("x86_64-unknown-linux-gnu", TargetInfo { arch: B("x86_64"), os: B("linux"), env: B("gnu"), endian: B("little"), pointer_width: B("64"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), ]) }),
        ("powerpc-unknown-linux-gnu", TargetInfo { arch: B("powerpc"), os: B("linux"), env: B("gnu"), endian: B("big"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), ]) }),
        ("i686-unknown-dragonfly", TargetInfo { arch: B("x86"), os: B("dragonfly"), env: B(""), endian: B("little"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), ]) }),
        ("aarch64-unknown-freebsd", TargetInfo { arch: B("aarch64"), os: B("freebsd"), env: B(""), endian: B("little"), pointer_width: B("64"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), ]) }),
        ("i686-unknown-linux-musl", TargetInfo { arch: B("x86"), os: B("linux"), env: B("musl"), endian: B("little"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_feature"), B("crt-static")), ]) }),
        ("x86_64-apple-darwin", TargetInfo { arch: B("x86_64"), os: B("macos"), env: B(""), endian: B("little"), pointer_width: B("64"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), ]) }),
        ("armv7-linux-androideabi", TargetInfo { arch: B("arm"), os: B("android"), env: B(""), endian: B("little"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), ]) }),
        ("asmjs-unknown-emscripten", TargetInfo { arch: B("asmjs"), os: B("emscripten"), env: B(""), endian: B("little"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), ]) }),
        ("x86_64-unknown-openbsd", TargetInfo { arch: B("x86_64"), os: B("openbsd"), env: B(""), endian: B("little"), pointer_width: B("64"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), ]) }),
        ("x86_64-unknown-haiku", TargetInfo { arch: B("x86_64"), os: B("haiku"), env: B(""), endian: B("little"), pointer_width: B("64"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), ]) }),
        ("x86_64-unknown-cloudabi", TargetInfo { arch: B("x86_64"), os: B("cloudabi"), env: B(""), endian: B("little"), pointer_width: B("64"), switches: B(&[]), other_keys: B(&[]) }),
        ("mipsel-unknown-linux-gnu", TargetInfo { arch: B("mips"), os: B("linux"), env: B("gnu"), endian: B("little"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), ]) }),
        ("sparc64-unknown-linux-gnu", TargetInfo { arch: B("sparc64"), os: B("linux"), env: B("gnu"), endian: B("big"), pointer_width: B("64"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), ]) }),
        ("x86_64-rumprun-netbsd", TargetInfo { arch: B("x86_64"), os: B("netbsd"), env: B(""), endian: B("little"), pointer_width: B("64"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), ]) }),
        ("powerpc64le-unknown-linux-gnu", TargetInfo { arch: B("powerpc64"), os: B("linux"), env: B("gnu"), endian: B("little"), pointer_width: B("64"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), ]) }),
        ("wasm32-experimental-emscripten", TargetInfo { arch: B("wasm32"), os: B("emscripten"), env: B(""), endian: B("little"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), ]) }),
        ("powerpc64-unknown-linux-gnu", TargetInfo { arch: B("powerpc64"), os: B("linux"), env: B("gnu"), endian: B("big"), pointer_width: B("64"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), ]) }),
        ("mipsel-unknown-linux-uclibc", TargetInfo { arch: B("mips"), os: B("linux"), env: B("uclibc"), endian: B("little"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), ]) }),
        ("arm-unknown-linux-gnueabihf", TargetInfo { arch: B("arm"), os: B("linux"), env: B("gnu"), endian: B("little"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), ]) }),
        ("mips64-unknown-linux-gnuabi64", TargetInfo { arch: B("mips64"), os: B("linux"), env: B("gnu"), endian: B("big"), pointer_width: B("64"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), ]) }),
        ("mips64el-unknown-linux-gnuabi64", TargetInfo { arch: B("mips64"), os: B("linux"), env: B("gnu"), endian: B("little"), pointer_width: B("64"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), ]) }),
        ("i686-unknown-openbsd", TargetInfo { arch: B("x86"), os: B("openbsd"), env: B(""), endian: B("little"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), ]) }),
        ("arm-unknown-linux-musleabihf", TargetInfo { arch: B("arm"), os: B("linux"), env: B("musl"), endian: B("little"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_feature"), B("crt-static")), ]) }),
        ("i586-unknown-linux-musl", TargetInfo { arch: B("x86"), os: B("linux"), env: B("musl"), endian: B("little"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_feature"), B("crt-static")), ]) }),
        ("aarch64-unknown-linux-gnu", TargetInfo { arch: B("aarch64"), os: B("linux"), env: B("gnu"), endian: B("little"), pointer_width: B("64"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), ]) }),
        ("armv7-unknown-cloudabi-eabihf", TargetInfo { arch: B("arm"), os: B("cloudabi"), env: B(""), endian: B("little"), pointer_width: B("32"), switches: B(&[]), other_keys: B(&[]) }),
        ("mipsel-unknown-linux-musl", TargetInfo { arch: B("mips"), os: B("linux"), env: B("musl"), endian: B("little"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), ]) }),
        ("mips-unknown-linux-musl", TargetInfo { arch: B("mips"), os: B("linux"), env: B("musl"), endian: B("big"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), ]) }),
        ("x86_64-unknown-linux-gnux32", TargetInfo { arch: B("x86_64"), os: B("linux"), env: B("gnu"), endian: B("little"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), ]) }),
        ("thumbv7em-none-eabi", TargetInfo { arch: B("arm"), os: B("none"), env: B(""), endian: B("little"), pointer_width: B("32"), switches: B(&[]), other_keys: B(&[]) }),
        ("thumbv6m-none-eabi", TargetInfo { arch: B("arm"), os: B("none"), env: B(""), endian: B("little"), pointer_width: B("32"), switches: B(&[]), other_keys: B(&[]) }),
        ("arm-unknown-linux-musleabi", TargetInfo { arch: B("arm"), os: B("linux"), env: B("musl"), endian: B("little"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_feature"), B("crt-static")), ]) }),
        ("i586-unknown-linux-gnu", TargetInfo { arch: B("x86"), os: B("linux"), env: B("gnu"), endian: B("little"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), ]) }),
        ("x86_64-unknown-netbsd", TargetInfo { arch: B("x86_64"), os: B("netbsd"), env: B(""), endian: B("little"), pointer_width: B("64"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), ]) }),
        ("i686-pc-windows-gnu", TargetInfo { arch: B("x86"), os: B("windows"), env: B("gnu"), endian: B("little"), pointer_width: B("32"), switches: B(&[B("windows"), ]), other_keys: B(&[(B("target_family"), B("windows")), ]) }),
        ("mips-unknown-linux-uclibc", TargetInfo { arch: B("mips"), os: B("linux"), env: B("uclibc"), endian: B("big"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), ]) }),
        ("x86_64-pc-windows-gnu", TargetInfo { arch: B("x86_64"), os: B("windows"), env: B("gnu"), endian: B("little"), pointer_width: B("64"), switches: B(&[B("windows"), ]), other_keys: B(&[(B("target_family"), B("windows")), ]) }),
        ("i686-pc-windows-msvc", TargetInfo { arch: B("x86"), os: B("windows"), env: B("msvc"), endian: B("little"), pointer_width: B("32"), switches: B(&[B("windows"), ]), other_keys: B(&[(B("target_family"), B("windows")), ]) }),
        ("x86_64-unknown-freebsd", TargetInfo { arch: B("x86_64"), os: B("freebsd"), env: B(""), endian: B("little"), pointer_width: B("64"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), ]) }),
        ("armv5te-unknown-linux-gnueabi", TargetInfo { arch: B("arm"), os: B("linux"), env: B("gnu"), endian: B("little"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), ]) }),
        ("x86_64-sun-solaris", TargetInfo { arch: B("x86_64"), os: B("solaris"), env: B(""), endian: B("little"), pointer_width: B("64"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), ]) }),
        ("sparc64-unknown-netbsd", TargetInfo { arch: B("sparc64"), os: B("netbsd"), env: B(""), endian: B("big"), pointer_width: B("64"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), ]) }),
        ("armv7-unknown-linux-gnueabihf", TargetInfo { arch: B("arm"), os: B("linux"), env: B("gnu"), endian: B("little"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), ]) }),
        ("sparcv9-sun-solaris", TargetInfo { arch: B("sparc64"), os: B("solaris"), env: B(""), endian: B("big"), pointer_width: B("64"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), ]) }),
        ("arm-unknown-linux-gnueabi", TargetInfo { arch: B("arm"), os: B("linux"), env: B("gnu"), endian: B("little"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), ]) }),
        ("msp430-none-elf", TargetInfo { arch: B("msp430"), os: B("none"), env: B(""), endian: B("little"), pointer_width: B("16"), switches: B(&[]), other_keys: B(&[]) }),
        ("i686-unknown-freebsd", TargetInfo { arch: B("x86"), os: B("freebsd"), env: B(""), endian: B("little"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), ]) }),
        ("armv7-unknown-linux-musleabihf", TargetInfo { arch: B("arm"), os: B("linux"), env: B("musl"), endian: B("little"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_feature"), B("crt-static")), ]) }),
        ("i686-unknown-cloudabi", TargetInfo { arch: B("x86"), os: B("cloudabi"), env: B(""), endian: B("little"), pointer_width: B("32"), switches: B(&[]), other_keys: B(&[]) }),
        ("i686-linux-android", TargetInfo { arch: B("x86"), os: B("android"), env: B(""), endian: B("little"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), ]) }),
    ]),
};