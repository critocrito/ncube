(ns ^:figwheel-hooks ncube.cards
  (:require [devcards.core]
            [ncube.cards.colors]
            [ncube.cards.fonts]
            [ncube.cards.forms]
            [ncube.cards.tags]))

(enable-console-print!)

(defn render []
  (devcards.core/start-devcard-ui!))

(defn ^:after-load render-on-relaod []
  (render))

(render)
