.PHONY: all test ui workspace dist clean-dist clean-build clean-pkg-dmg pkg-bin pkg-deb pkg-deb-ncubed verify

target_dir = target
release_dir = $(target_dir)/release
pkg_build_dir = $(target_dir)/pkgs
pkg_build_macos = $(pkg_build_dir)/Ncube.app
dist_dir = $(target_dir)/dist
dist_fonts_dir = $(dist_dir)/fonts
dist_images_dir = $(dist_dir)/images
css_dir = $(target_dir)/css
webpack_dir = $(target_dir)/webpack
workspace_dir = $(target_dir)/workspace
workspace_archive = $(workspace_dir)/workspace.tar.gz
pkgs_release_dir = pkgs

all: $(release_dir)/ncube

$(webpack_dir)/index.html $(webpack_dir)/app.js $(webpack_dir)/styles.css:
	yarn compile

$(dist_dir): $(webpack_dir)/index.html $(webpack_dir)/app.js $(webpack_dir)/styles.css
	cp -av $(webpack_dir) $(dist_dir)

$(workspace_archive):
	@mkdir -p $(workspace_dir)
	./scripts/build_workspace.sh $(workspace_dir) $(workspace_archive)

$(release_dir)/ncube: dist
	@mkdir -p $(release_dir)
	cargo build --bin ncube --release

$(release_dir)/ncubed: dist
	@mkdir -p $(release_dir)
	cargo build --bin ncubed --release

$(release_dir)/ncubectl: dist
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
	rm -rf $(webpack_dir)
	rm -rf $(dist_dir)
	rm -rf $(workspace_dir)
	rm $(workspace_archive)

clean-build:
	rm -rf $(pkg_build_dir)

clean:
	rm -rf $(target_dir)
	rm -rf $(pkgs_release_dir)

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

verify:
	yarn verify
	cargo check

test:
	cargo test

ui: $(dist_dir)

workspace: $(workspace_archive)

dist: ui workspace
