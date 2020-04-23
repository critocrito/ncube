.PHONY: all ui devcards clean-dist clean-build clean-devcards pkg-macos

target_dir = target
release_dir = $(target_dir)/release
pkg_build_dir = $(target_dir)/pkgs
pkg_build_macos = $(pkg_build_dir)/Ncube.app
dist_dir = $(target_dir)/dist
dist_fonts_dir = $(dist_dir)/fonts
css_dir = $(target_dir)/css
cljs_dir = $(target_dir)/public/cljs-out
pkgs_release_dir = pkgs
devcards_dir = devcards

all: $(release_dir)/ncube

$(cljs_dir)/prod-main.js:
	@mkdir -p $(cljs_dir)
	clojure -A:fig-deps:prod-deps:min

$(cljs_dir)/cards-main.js:
	@mkdir -p $(cljs_dir)
	clojure -A:fig-deps:prod-deps:cards-deps:cards

$(css_dir)/styles.css:
	@mkdir -p $(css_dir)
	node_modules/.bin/postcss -o $(css_dir)/styles.css src/css/styles.css

$(dist_dir)/app.js: $(cljs_dir)/prod-main.js
	@mkdir -p  $(dist_dir)
	cp $(cljs_dir)/prod-main.js $(dist_dir)/app.js

$(dist_dir)/cards.js: $(cljs_dir)/cards-main.js
	@mkdir -p $(dist_dir)
	cp $(cljs_dir)/cards-main.js $(dist_dir)/cards.js

$(dist_dir)/index.html:
	@mkdir -p $(dist_dir)
	cp resources/public/prod.html $(dist_dir)/index.html

$(dist_dir)/styles.css: $(css_dir)/styles.css
	@mkdir -p  $(dist_dir)
	cp $(css_dir)/styles.css $(dist_dir)/styles.css

$(dist_fonts_dir)/NotoSans-Regular.ttf:
	@mkdir -p $(dist_fonts_dir)
	cp resources/public/fonts/NotoSans-Regular.ttf $(dist_fonts_dir)

$(dist_fonts_dir)/NotoSans-Bold.ttf:
	@mkdir -p $(dist_fonts_dir)
	cp resources/public/fonts/NotoSans-Bold.ttf $(dist_fonts_dir)

$(devcards_dir)/app.js: $(cljs_dir)/cards-main.js
	@mkdir -p $(devcards_dir)
	cp $(cljs_dir)/cards-main.js $(devcards_dir)/app.js

$(devcards_dir)/index.html:
	@mkdir -p $(devcards_dir)
	cp resources/public/cards.html $(devcards_dir)/index.html

$(devcards_dir)/styles.css: $(css_dir)/styles.css
	@mkdir -p $(devcards_dir)
	cp $(css_dir)/styles.css $(devcards_dir)/styles.css

$(devcards_dir)/fonts:
	@mkdir -p $(devcards_dir)/fonts
	cp resources/public/fonts/NotoSans-Regular.ttf $(devcards_dir)/fonts
	cp resources/public/fonts/NotoSans-Bold.ttf $(devcards_dir)/fonts

devcards: clean-devcards \
			$(devcards_dir)/app.js \
			$(devcards_dir)/styles.css \
			$(devcards_dir)/index.html \
			$(devcards_dir)/fonts

$(release_dir)/ncube: $(dist_dir)/app.js \
						$(dist_dir)/styles.css \
						$(dist_dir)/index.html \
						$(dist_fonts_dir)/NotoSans-Regular.ttf \
						$(dist_fonts_dir)/NotoSans-Bold.ttf
	@mkdir -p $(release_dir)
	cargo build --bin ncube --release

$(release_dir)/ncubed:
	@mkdir -p $(release_dir)
	cargo build --bin ncubed --release

$(release_dir)/ncubectl:
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
	rm -rf $(cljs_dir)
	rm -rf $(dist_dir)

clean-build:
	rm -rf $(pkg_build_dir)

clean-devcards:
	rm -rf $(devcards_dir)

clean:
	rm -rf $(target_dir)
	rm -rf $(pkgs_release_dir)

# TODO: If I don't provide a signing code the create-dmg command returns an
# error. To make make not choke up on that I force a good return.
pkg-dmg: $(pkg_build_macos)
	@mkdir -p  $(pkgs_release_dir)
	npx create-dmg --overwrite $(pkg_build_macos) $(pkgs_release_dir) | true

ui: $(dist_dir)/app.js \
	$(dist_dir)/index.html \
	$(dist_dir)/styles.css \
	$(dist_fonts_dir)/NotoSans-Regular.ttf \
	$(dist_fonts_dir)/NotoSans-Bold.ttf
