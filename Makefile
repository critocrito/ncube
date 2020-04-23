.PHONY: prod dev cards js css fonts html ui ncube release clean

all: release

dist:
	mkdir -p resources/dist/fonts

clean:
	rm -rf target/release/ncube
	rm -rf target/public
	rm -rf resources/dist

target/public/cljs-out/prod-main.js:
	clojure -A:fig-deps:prod-deps:min

target/public/cljs-out/cards-main.js:
	clojure -A:fig-deps:prod-deps:cards-deps:cards

target/release/ncube:
	cargo build --bin ncube --release

resources/dist/app.js: dist target/public/cljs-out/prod-main.js
	cp target/public/cljs-out/prod-main.js resources/dist/app.js

resources/dist/cards.js: dist target/public/cljs-out/cards-main.js
	cp target/public/cljs-out/cards-main.js resources/dist/cards.js

resources/dist/index.html: dist
	cp resources/public/prod.html resources/dist/index.html

resources/dist/cards.html: dist
	cp resources/public/cards.html resources/dist/cards.html

resources/dist/index.css: dist
	postcss -o resources/dist/index.css src/css/*.css

resources/dist/fonts/NotoSans-Regular.ttf: dist
	cp resources/public/fonts/NotoSans-Regular.ttf resources/dist/fonts

resources/dist/fonts/NotoSans-Bold.ttf: dist
	cp resources/public/fonts/NotoSans-Bold.ttf resources/dist/fonts

ncube: target/release/ncube

css: resources/dist/index.css

fonts: resources/dist/fonts/NotoSans-Regular.ttf resources/dist/fonts/NotoSans-Bold.ttf

js: resources/dist/app.js

html: resources/dist/index.html

ui: js html css fonts

release: clean ui ncube

prod:
	clj -A:fig-deps:prod-deps:prod

dev:
	clj -A:fig-deps:dev-deps:dev

cards:
	clj -A:fig-deps:prod-deps:cards-deps:cards

outdated:
	clj -A:fig-deps:dev-deps:prod-deps:outdated
