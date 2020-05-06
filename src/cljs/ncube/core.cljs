(ns ^:figwheel-hooks ncube.core
  (:require [reagent.core :as r]
            [reagent.dom :as dom]
            [re-frame.core :as rf]
            [re-frame.std-interceptors :refer [debug]]
            [day8.re-frame.async-flow-fx]
            [day8.re-frame.http-fx]
            [day8.re-frame.tracing :refer-macros [fn-traced defn-traced]]
            [reitit.core :as reitit]
            [reitit.frontend.easy :as rfe]
            [clerk.core :as clerk]
            [ncube.router :refer [router init-routes!]]
            [ncube.db :refer [default-db]]
            [ncube.events]        ;; These three are only
            [ncube.subscriptions] ;; required to make Clojurescript
            [ncube.views :refer [router-component]]))       ;; load them

(def debug? ^boolean goog.DEBUG)

(def app-db  (r/atom default-db))

(r/after-render clerk/after-render!)

(rf/reg-fx
 :navigate!
 (fn
   [args]
   (let [[route-name params] (cond
                               (vector? args) [(first args) (second args)]
                               :else [args {}])]
     ;; FIXME: Handle routes not found
     (let [route (reitit/match-by-name router route-name params)]
       (rfe/push-state (-> route :data :name) params)
       (clerk/navigate-page! (:path route))))))

(rf/reg-fx
 :history
 (fn
   [direction]
   (cond
     (= direction :back) (.back js/window.history)
     (= direction :forward) (.forward js/window.history)
     :else nil)))

(defn mount
  []
  (clerk/initialize!)
  (init-routes!)
  (dom/render
   [router-component {:router router}]
   (js/document.getElementById "app")))

(defn ^:after-load re-render [] (mount))

(defn ^:export startup
  []
  (js/console.log "Starting Ncube.")
  (rf/dispatch-sync [:boot])
  (mount))
