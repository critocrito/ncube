.PHONY: all workspace ui ncube ncubed \
pkg-bin pkg-deb pkg-deb-ncubed pkg-web-ext clean clean-dist clean-build clean-pkgs \
verify verify-ui verify-backend verify-web-ext test test-ui test-backend fmt-backend

target_dir = target
ui_dir = $(target_dir)/ui
webext_dir = $(target_dir)/web-ext
workspace_dir = $(target_dir)/workspace
backend_dir = $(target_dir)/release
pkg_build_dir = $(target_dir)/pkgs
pkg_build_macos = $(pkg_build_dir)/Ncube.app
workspace_archive = $(workspace_dir)/workspace.tar.gz
pkgs_release_dir = pkgs
node_deps_dir = node_modules

all: ncube

ui: $(ui_dir)

ncube: $(backend_dir)/ncube

workspace: $(workspace_archive)

ncubed: $(backend_dir)/ncubed $(backend_dir)/ncubectl

$(node_deps_dir):
	yarn install

$(ui_dir): $(node_deps_dir)
	yarn compile:ui

$(webext_dir): $(node_deps_dir)
	yarn compile:web-ext

$(workspace_archive):
	@mkdir -p $(workspace_dir)
	./scripts/build_workspace.sh $(workspace_dir) $(workspace_archive)

$(backend_dir)/ncube: $(ui_dir) $(workspace_archive)
	@mkdir -p $(backend_dir)
	cargo build --bin ncube --release

$(backend_dir)/ncubed: $(ui_dir) $(workspace_archive)
	@mkdir -p $(backend_dir)
	cargo build --bin ncubed --release

$(backend_dir)/ncubectl: $(ui_dir) $(workspace_archive)
	@mkdir -p $(backend_dir)
	cargo build --bin ncubectl --release

$(pkg_build_macos): $(backend_dir)/ncube
	@mkdir -p $(pkg_build_macos)/Contents/{MacOS,Resources}
	cp -r resources/Info.plist $(pkg_build_macos)/Contents
	cp -r resources/icon.icns $(pkg_build_macos)/Contents/Resources/ncube.icns
	cp -r $(backend_dir)/ncube $(pkg_build_macos)/Contents/MacOS/ncube-bin
	cp -r scripts/macos_launch.sh $(pkg_build_macos)/Contents/MacOS/Ncube
	chmod +x $(pkg_build_macos)/Contents/MacOS/ncube-bin
	chmod +x $(pkg_build_macos)/Contents/MacOS/Ncube

# TODO: If I don't provide a signing code the create-dmg command returns an
# error. To make make not choke up on that I force a good return.
pkg-dmg: $(pkg_build_macos)
	@mkdir -p  $(pkgs_release_dir)
	npx create-dmg --overwrite $(pkg_build_macos) $(pkgs_release_dir) | true
	for f in pkgs/*.dmg; do mv "$$f" "$${f// /_}"; done

pkg-bin: $(backend_dir)/ncube
	@mkdir -p $(pkgs_release_dir)
	cp $(backend_dir)/ncube $(pkgs_release_dir)/ncube-$(shell uname -s | sed "y/ABCDEFGHIJKLMNOPQRSTUVWXYZ/abcdefghijklmnopqrstuvwxyz/")

pkg-deb: $(backend_dir)/ncube
	@mkdir -p $(pkgs_release_dir)
	cargo deb -p ncube
	cp target/debian/ncube*.deb $(pkgs_release_dir)

pkg-deb-ncubed: $(backend_dir)/ncubed $(backend_dir)/ncubectl
	@mkdir -p $(pkgs_release_dir)
	cargo deb -p ncubed
	cp target/debian/ncubed*.deb $(pkgs_release_dir)

pkg-web-ext: web-ext
	@mkdir -p $(pkgs_release_dir)
	node_modules/.bin/web-ext build -s $(webext_dir) -a $(pkgs_release_dir) --overwrite-dest

clean: clean-pkgs clean-dist

clean-dist:
	rm -rf $(backend_dir)/ncube
	rm -rf $(backend_dir)/ncubed
	rm -rf $(backend_dir)/ncubectl
	rm -rf $(ui_dir)
	rm -rf $(webext_dir)
	rm -rf $(workspace_dir)
	rm -f $(workspace_archive)

clean-build:
	rm -rf $(pkg_build_dir)

clean-web-ext:
	rm -rf $(webext_dir)

clean-pkgs:
	rm -rf $(pkgs_release_dir)

verify: verify-ui verify-web-ext verify-backend

verify-ui: $(node_deps_dir)
	yarn run-p lint:eslint lint:tsc

verify-web-ext: $(webext_dir)
	yarn web-ext:lint

verify-backend: $(ui_dir) $(workspace_archive)
	cargo check --all --all-features

test: test-ui test-backend

test-ui:
	yarn test

test-backend: $(ui_dir) $(workspace_archive)
	cargo test --all --all-features

fmt-backend:
	cargo fmt --all -- --check
