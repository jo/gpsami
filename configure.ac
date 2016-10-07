AC_INIT([gpsami], m4_esyscmd([grep ^version Cargo.toml | awk '{print $3}' | tr -d '"' | tr -d "\n"]), [hub@figuiere.net])
AM_INIT_AUTOMAKE([1.11 foreign no-dependencies no-dist-gzip dist-xz subdir-objects])

VERSION=$(grep ^version Cargo.toml | awk '{print $3}' | tr -d '"' | tr -d "\n")

AC_CHECK_PROG(CARGO, [cargo], [yes], [no])
AC_CHECK_PROG(RUSTC, [rustc], [yes], [no])

AC_SUBST(CARGO)
AC_SUBST(RUSTC)

CARGO_BUILD_MODE=
dnl Release build we do.
CARGO_TARGET_DIR=release
AC_SUBST(CARGO_TARGET_DIR)

AC_SUBST(VERSION)

AC_CONFIG_FILES([Makefile
        data/gpsami.desktop])

AC_OUTPUT


echo "
You can now build using 'make'

Alternatively you can use 'cargo build'
"
