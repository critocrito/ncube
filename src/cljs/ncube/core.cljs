(ns ^:figwheel-hooks ncube.core
  (:require [reagent.core :as r]
            [reagent.dom :as rdom]
            [re-frame.core :as rf]
            [re-frame.std-interceptors :refer [debug]]
            [day8.re-frame.tracing :refer-macros [fn-traced defn-traced]]
            [clojure.string :as str]))

(def app-db  (r/atom {}))

(rf/reg-event-db
 :initialize
 (fn-traced
  [db event]
  {:count 4}))

(rf/reg-event-db
 :inc-counter
 (fn-traced
  [{:keys [count]}]
   {:count (+ count 7)}))

(rf/reg-event-db
 :dec-counter
 (fn-traced
  [{:keys [count]}]
  {:count (- count 1)}))

(rf/reg-sub
 :counter
 (fn
  [{:keys [count]}]
  count))

(defn ui
  []
  (let [counter @(rf/subscribe [:counter])]
    [:div
     [:h1 "Hello Universe!"]
     [:p counter]
     [:button
      {:on-click #(rf/dispatch [:inc-counter])}
      "Increment"]
     [:button
      {:on-click #(rf/dispatch [:dec-counter])}
      "Decrement"]]))

(defn mount [] (rdom/render [ui] (js/document.getElementById "app")))

(defn ^:after-load re-render [] (mount))

(defn ^:export startup
  []
  (js/console.log "Starting Ncube.")
  (rf/dispatch-sync [:initialize])
  (mount))
