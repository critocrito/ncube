(ns ncube.events
  (:require
   [re-frame.core :refer [reg-event-db reg-event-fx]]
   [re-frame.std-interceptors :refer [debug]]
   [day8.re-frame.tracing :refer-macros [fn-traced defn-traced]]
   [ncube.db :refer [default-db]]))

(reg-event-db
 :initialize
 (fn-traced
  []
  (js/console.log "initialize")
  default-db))

(reg-event-fx
 :set-active-page
 (fn-traced [{:keys [db]} [_ {:keys [page]}]]
   (let [set-page (assoc db :active-page page)]
     (case page
       :home
       {:db set-page}))))

(reg-event-db
 :inc-counter
 (fn-traced
  [{:keys [count]}]
   {:count (+ count 1)}))

(reg-event-db
 :dec-counter
 (fn-traced
  [{:keys [count]}]
  {:count (- count 1)}))
