prefix ?= /usr
exec_prefix = $(prefix)
bindir = $(exec_prefix)/bin
includedir = $(prefix)/include
datarootdir = $(prefix)/share
datadir = $(datarootdir)

.PHONY: all install uninstall

BIN=com.github.arshubham.srtnr

SRC=src/* src/*/*
DATA=data/* 

all: target/release/$(BIN)

debug: target/debug/$(BIN)

clean:
	rm -rf target/release/data
	cargo clean

install: all
	install -D -m 0755 "target/release/$(BIN)" "$(DESTDIR)$(bindir)/$(BIN)"
	install -D -m 0644 "target/release/data/images/icons/16/com.github.arshubham.srtnr.svg" "$(DESTDIR)$(datarootdir)/icons/hicolor/16x16/apps/com.github.arshubham.srtnr.svg"
	install -D -m 0644 "target/release/data/images/icons/24/com.github.arshubham.srtnr.svg" "$(DESTDIR)$(datarootdir)/icons/hicolor/24x24/apps/com.github.arshubham.srtnr.svg"
	install -D -m 0644 "target/release/data/images/icons/32/com.github.arshubham.srtnr.svg" "$(DESTDIR)$(datarootdir)/icons/hicolor/32x32/apps/com.github.arshubham.srtnr.svg"
	install -D -m 0644 "target/release/data/images/icons/48/com.github.arshubham.srtnr.svg" "$(DESTDIR)$(datarootdir)/icons/hicolor/48x48/apps/com.github.arshubham.srtnr.svg"
	install -D -m 0644 "target/release/data/images/icons/64/com.github.arshubham.srtnr.svg" "$(DESTDIR)$(datarootdir)/icons/hicolor/64x64/apps/com.github.arshubham.srtnr.svg"
	install -D -m 0644 "target/release/data/images/icons/128/com.github.arshubham.srtnr.svg" "$(DESTDIR)$(datarootdir)/icons/hicolor/128x128/apps/com.github.arshubham.srtnr.svg"
	install -D -m 0644 "target/release/data/images/com.github.arshubham.srtnr.svg" "$(DESTDIR)$(datarootdir)/com.github.arshubham.srtnr/images/com.github.arshubham.srtnr.svg"
	install -D -m 0644 "target/release/data/images/com.github.arshubham.srtnr.png" "$(DESTDIR)$(datarootdir)/pixmaps/com.github.arshubham.srtnr.png"
	install -D -m 0644 "target/release/data/com.github.arshubham.srtnr.appdata.xml" "$(DESTDIR)$(datarootdir)/metainfo/com.github.arshubham.srtnr.appdata.xml"
	install -D -m 0644 "target/release/data/com.github.arshubham.srtnr.desktop" "$(DESTDIR)$(datarootdir)/applications/com.github.arshubham.srtnr.desktop"
	install -D -m 0644 "target/release/data/com.github.arshubham.srtnr.gschema.xml" "$(DESTDIR)$(datarootdir)/glib-2.0/schemas/com.github.arshubham.srtnr.gschema.xml"
	glib-compile-schemas $(DESTDIR)$(datarootdir)/glib-2.0/schemas/

uninstall: 
	rm -f "$(DESTDIR)$(bindir)/$(BIN)"
	rm -f "$(datarootdir)/icons/hicolor/16x16/apps/com.github.arshubham.srtnr.svg"
	rm -f "$(datarootdir)/icons/hicolor/24x24/apps/com.github.arshubham.srtnr.svg"
	rm -f "$(datarootdir)/icons/hicolor/32x32/apps/com.github.arshubham.srtnr.svg"
	rm -f "$(datarootdir)/icons/hicolor/48x48/apps/com.github.arshubham.srtnr.svg"
	rm -f "$(datarootdir)/icons/hicolor/64x64/apps/com.github.arshubham.srtnr.svg"
	rm -f "$(datarootdir)/icons/hicolor/128x128/apps/com.github.arshubham.srtnr.svg"
	rm -rf "$(datarootdir)/com.github.arshubham.srtnr/"
	rm -f "$(datarootdir)/pixmaps/com.github.arshubham.srtnr.png"
	rm -f "$(datarootdir)/metainfo/com.github.arshubham.srtnr.appdata.xml"
	rm -f "$(datarootdir)/applications/com.github.arshubham.srtnr.desktop"
	rm -f "$(datarootdir)/glib-2.0/schemas/com.github.arshubham.srtnr.gschema.xml"
	glib-compile-schemas $(datarootdir)/glib-2.0/schemas/

.cargo/config: vendor_config
	mkdir -p .cargo
	cp $< $@

update:
	cargo update

vendor: .cargo/config
	cargo vendor
	touch vendor

target/release/$(BIN): $(SRC)
	mkdir -p target/release/data
	cp -r $(DATA) target/release/data		
	if [ -d vendor ]; \
	then \
		cargo build --release --frozen -p srtnr && mv target/release/srtnr target/release/$(BIN); \
	else \
		cargo build --release -p srtnr && mv target/release/srtnr target/release/$(BIN); \
	fi 
