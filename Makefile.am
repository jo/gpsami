DIST_SUBDIRS=src
EXTRA_PROGRAMS=target/@CARGO_TARGET_DIR@/gpsami

desktop_files = data/gpsami.desktop

desktopdir   = $(datadir)/applications
desktop_DATA = $(desktop_files)

all-local:
	cargo build --release

clean-local:
	-cargo clean

install-exec-local:
	$(MKDIR_P) $(DESTDIR)$(bindir)
	$(INSTALL) -c -m 755 target/@CARGO_TARGET_DIR@/gpsami $(DESTDIR)$(bindir)

EXTRA_DIST = \
	$(desktop_in_files) \
	$(null)

DISTCLEANFILES =                                \
	$(desktop_files)                        \
	$(null)

check:
	desktop-file-validate $(desktop_files)
