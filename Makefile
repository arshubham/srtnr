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
	rm -r target/release/data
	cargo clean

copydata: 
	mkdir -p target/release/data
	cp -r $(DATA) target/release/data

install: all
	install -D -m 0755 "target/release/$(BIN)" "$(DESTDIR)$(bindir)/$(BIN)"
	install -D -m 0644 "target/release/data/images/icons/16/com.github.arshubham.srtnr.svg" "$(datarootdir)/icons/hicolor/16x16/apps/com.github.arshubham.srtnr.svg"
	install -D -m 0644 "target/release/data/images/icons/24/com.github.arshubham.srtnr.svg" "$(datarootdir)/icons/hicolor/24x24/apps/com.github.arshubham.srtnr.svg"
	install -D -m 0644 "target/release/data/images/icons/32/com.github.arshubham.srtnr.svg" "$(datarootdir)/icons/hicolor/32x32/apps/com.github.arshubham.srtnr.svg"
	install -D -m 0644 "target/release/data/images/icons/48/com.github.arshubham.srtnr.svg" "$(datarootdir)/icons/hicolor/48x48/apps/com.github.arshubham.srtnr.svg"
	install -D -m 0644 "target/release/data/images/icons/64/com.github.arshubham.srtnr.svg" "$(datarootdir)/icons/hicolor/64x64/apps/com.github.arshubham.srtnr.svg"
	install -D -m 0644 "target/release/data/images/icons/128/com.github.arshubham.srtnr.svg" "$(datarootdir)/icons/hicolor/128x128/apps/com.github.arshubham.srtnr.svg"
	install -D -m 0644 "target/release/data/images/com.github.arshubham.srtnr.svg" "$(datarootdir)/com.github.arshubham.srtnr/images/com.github.arshubham.srtnr.svg"
	install -D -m 0644 "target/release/data/images/com.github.arshubham.srtnr.png" "$(datarootdir)/pixmaps/com.github.arshubham.srtnr.png"
	install -D -m 0644 "target/release/data/com.github.arshubham.srtnr.appdata.xml" "$(datarootdir)/metainfo/com.github.arshubham.srtnr.appdata.xml"
	install -D -m 0644 "target/release/data/com.github.arshubham.srtnr.desktop" "$(datarootdir)/applications/com.github.arshubham.srtnr.desktop"

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

target/release/$(BIN): $(SRC)
		cargo build --release -p srtnr && mv target/release/srtnr target/release/$(BIN)
		copydata