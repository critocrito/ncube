.PHONY: all test test-ui test-backend ui workspace web-ext clean-dist clean-build \
clean-pkgs pkg-bin pkg-deb pkg-deb-ncubed pkg-web-ext verify verify-ui \
verify-backend deps server

target_dir = target
release_dir = $(target_dir)/release
pkg_build_dir = $(target_dir)/pkgs
pkg_build_macos = $(pkg_build_dir)/Ncube.app
dist_dir = $(target_dir)/dist
dist_fonts_dir = $(dist_dir)/fonts
dist_images_dir = $(dist_dir)/images
css_dir = $(target_dir)/css
webpack_dir = $(target_dir)/webpack
webext_dir = $(target_dir)/web-ext
workspace_dir = $(target_dir)/workspace
workspace_archive = $(workspace_dir)/workspace.tar.gz
pkgs_release_dir = pkgs

all: $(release_dir)/ncube

$(webpack_dir): deps
	yarn compile

web-ext: deps
	yarn web-ext:prod

$(dist_dir): $(webpack_dir)
	@mkdir -p $(dist_dir)
	@cp -av $(webpack_dir)/* $(dist_dir)

$(workspace_archive):
	@mkdir -p $(workspace_dir)
	./scripts/build_workspace.sh $(workspace_dir) $(workspace_archive)

$(release_dir)/ncube: $(dist_dir) $(workspace_archive)
	@mkdir -p $(release_dir)
	cargo build --bin ncube --release

$(release_dir)/ncubed: $(dist_dir) $(workspace_archive)
	@mkdir -p $(release_dir)
	cargo build --bin ncubed --release

$(release_dir)/ncubectl: $(dist_dir) $(workspace_archive)
	@mkdir -p $(release_dir)
	cargo build --bin ncubectl --release

$(pkg_build_macos): $(release_dir)/ncube
	@mkdir -p $(pkg_build_macos)/Contents/{MacOS,Resources}
	cp -r resources/Info.plist $(pkg_build_macos)/Contents
	cp -r resources/icon.icns $(pkg_build_macos)/Contents/Resources/ncube.icns
	cp -r $(release_dir)/ncube $(pkg_build_macos)/Contents/MacOS/ncube-bin
	cp -r scripts/macos_launch.sh $(pkg_build_macos)/Contents/MacOS/Ncube
	chmod +x $(pkg_build_macos)/Contents/MacOS/ncube-bin
	chmod +x $(pkg_build_macos)/Contents/MacOS/Ncube

clean-dist:
	rm -rf $(release_dir)/ncube
	rm -rf $(release_dir)/ncubed
	rm -rf $(release_dir)/ncubectl
	rm -rf $(webpack_dir)
	rm -rf $(webext_dir)
	rm -rf $(dist_dir)
	rm -rf $(workspace_dir)
	rm -f $(workspace_archive)

clean-build:
	rm -rf $(pkg_build_dir)

clean-web-ext:
	rm -rf $(webext_dir)

clean-pkgs:
	rm -rf $(pkgs_release_dir)

clean: clean-pkgs
	rm -rf $(target_dir)

# TODO: If I don't provide a signing code the create-dmg command returns an
# error. To make make not choke up on that I force a good return.
pkg-dmg: $(pkg_build_macos)
	@mkdir -p  $(pkgs_release_dir)
	npx create-dmg --overwrite $(pkg_build_macos) $(pkgs_release_dir) | true
	for f in pkgs/*.dmg; do mv "$$f" "$${f// /_}"; done

pkg-bin: $(release_dir)/ncube
	@mkdir -p $(pkgs_release_dir)
	cp $(release_dir)/ncube $(pkgs_release_dir)/ncube-$(shell uname -s | sed "y/ABCDEFGHIJKLMNOPQRSTUVWXYZ/abcdefghijklmnopqrstuvwxyz/")

pkg-deb: $(release_dir)/ncube
	@mkdir -p $(pkgs_release_dir)
	cargo deb -p ncube
	cp target/debian/ncube*.deb $(pkgs_release_dir)

pkg-deb-ncubed: $(release_dir)/ncubed $(release_dir)/ncubectl
	@mkdir -p $(pkgs_release_dir)
	cargo deb -p ncubed
	cp target/debian/ncubed*.deb $(pkgs_release_dir)

pkg-web-ext: web-ext
	@mkdir -p $(pkgs_release_dir)
	node_modules/.bin/web-ext build -s $(webext_dir) -a $(pkgs_release_dir) --overwrite-dest

verify-ui: deps $(dist_dir)
	yarn lint
	yarn type-check

check-backend: $(dist_dir) $(workspace_archive)
	cargo check --all --all-features

format-backend: $(dist_dir) $(workspace_archive)
	cargo fmt --all -- --check

verify-backend: check-backend format-backend

verify: verify-ui verify-backend

deps:
	yarn install

test-ui:
	yarn test

test-backend: $(dist_dir) $(workspace_archive)
	cargo test --all --all-features

test: test-ui test-backend

ui: deps $(dist_dir)

backend: $(release_dir)/ncube $(release_dir)/ncubed $(release_dir)/ncubectl

workspace: $(workspace_archive)

server: $(release_dir)/ncubed $(release_dir)/ncubectl
