(ns ^:figwheel-hooks ncube.cards
  (:require [devcards.core]
            [ncube.cards.colors]))

(enable-console-print!)

(defn render []
  (devcards.core/start-devcard-ui!))

(defn ^:after-load render-on-relaod []
  (render))

(render)
