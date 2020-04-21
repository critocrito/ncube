.PHONY: prod dev js html build release clean

all: build

dist:
	mkdir -p resources/dist/fonts

clean:
	rm -rf target/public
	rm -rf resources/dist

target/public/cljs-out/prod-main.js:
	clojure -A:fig-deps:prod-deps:min

target/public/cljs-out/cards-main.js:
	clojure -A:fig-deps:prod-deps:cards-deps:cards

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

css: resources/dist/index.css

fonts: resources/dist/fonts/NotoSans-Regular.ttf resources/dist/fonts/NotoSans-Bold.ttf

js: resources/dist/app.js resources/dist/cards.js

html: resources/dist/index.html resources/dist/cards.html

build: js html css fonts

release: build

prod:
	clj -A:fig-deps:prod-deps:prod

dev:
	clj -A:fig-deps:dev-deps:dev

cards:
	clj -A:fig-deps:prod-deps:cards-deps:cards

outdated:
	clj -A:fig-deps:dev-deps:prod-deps:outdated
