(ns ncube.sources.events
  (:require [re-frame.core :as rf]
            [re-frame.std-interceptors :refer [debug]]
            [day8.re-frame.tracing :refer-macros [fn-traced defn-traced]]
            [ajax.core :as ajax]
            [fork.core :as fork]))

(rf/reg-event-fx
 ::create-source
 (fn-traced
  [_ _]
  {:navigate! :sources-create}))

(rf/reg-event-fx
 ::sources-fetched
 (fn-traced
  [{db :db} [_ slug {:keys [data]}]]
  {:navigate! [:sources-list {:slug slug}]
   :db (assoc db :sources data)}))

(rf/reg-event-fx
 ::list-sources
 (fn-traced
  [_ [_ slug]]
  {:http-xhrio {:method :get
                :uri (str "http://127.0.0.1:40666/api/workspaces/" slug "/sources")
                :response-format (ajax/json-response-format {:keywords? true})
                :on-success [::sources-fetched slug]
                :on-failure [:http-error]}}))


(rf/reg-event-fx
 ::delete-source
 (fn-traced
  [_ [_ slug id]]
  {:http-xhrio {:method :delete
                :uri (str "http://127.0.0.1:40666/api/workspaces/" slug "/sources/" id)
                :format (ajax/json-request-format)
                :response-format (ajax/raw-response-format) 
                :on-success [::list-sources slug]
                :on-failure [:http-error]}}))
