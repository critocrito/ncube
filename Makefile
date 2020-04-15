.PHONY: prod dev js html build release clean

all: build

clean:
	rm -f target/public/cljs-out/prod-main.js
	rm -f resources/dist/app.js
	rm -f resources/dist/index.html

target/public/cljs-out/prod-main.js:
	clojure -A:fig-deps:prod-deps:min

resources/dist/app.js: target/public/cljs-out/prod-main.js
	cp target/public/cljs-out/prod-main.js resources/dist/app.js

resources/dist/index.html:
	cp resources/public/prod.html resources/dist/index.html

js: resources/dist/app.js

html: resources/dist/index.html

build: js html

release: js html

prod:
	clj -A:fig-deps:prod-deps:prod

dev:
	clj -A:fig-deps:dev-deps:dev

outdated:
	clj -A:fig-deps:dev-deps:prod-deps:outdated
