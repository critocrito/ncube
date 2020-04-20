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

(reg-event-db
 :navigated
 (fn-traced [db [_ new-match]]
   (assoc db :current-route new-match)))
