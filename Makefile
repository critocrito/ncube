.PHONY: prod dev js html build release clean

all: build

dist:
	mkdir resources/dist

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

js: resources/dist/app.js resources/dist/cards.js

html: resources/dist/index.html resources/dist/cards.html

build: js html

release: js html

prod:
	clj -A:fig-deps:prod-deps:prod

dev:
	clj -A:fig-deps:dev-deps:dev

cards:
	clj -A:fig-deps:prod-deps:cards-deps:cards

outdated:
	clj -A:fig-deps:dev-deps:prod-deps:outdated
