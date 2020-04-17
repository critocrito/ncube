(ns ^:figwheel-hooks ncube.core
  (:require [reagent.core :as r]
            [reagent.dom :as dom]
            [re-frame.core :as rf]
            [re-frame.std-interceptors :refer [debug]]
            [day8.re-frame.tracing :refer-macros [fn-traced defn-traced]]
            [ncube.router :as router]
            [ncube.db :refer [default-db]]
            [ncube.events]        ;; These three are only
            [ncube.subscriptions] ;; required to make Clojurescript
            [ncube.views]))       ;; load them

(def app-db  (r/atom default-db))

(defn mount
  []
  (dom/render [ncube.views/ui] (js/document.getElementById "app")))

(defn ^:after-load re-render [] (mount))

(defn ^:export startup
  []
  (js/console.log "Starting Ncube.")
  (router/start!)
  (rf/dispatch-sync [:initialize])
  (mount))
