(ns ncube.onboarding.events
  (:require [re-frame.core :as rf]
            [day8.re-frame.tracing :refer-macros [fn-traced]]
            [day8.re-frame.http-fx]
            [ajax.core :as ajax]
            [fork.core :as fork]
            [ncube.components :as c]))

(defn cfg->body
  [cfg]
  (->> cfg
       (into [])
       (map (fn [[n v]] {:name (name n) :value (str v)}))))

(rf/reg-event-fx
 ::success
 [(fork/clean :form)]
 (fn-traced
  [{db :db} [_ {:keys [data]}]]
  {:db (assoc db :result data)
   :dispatch [:boot]}))


(rf/reg-event-fx
 ::failure
 (fn-traced
  [{db :db} [_ result]]
  (js/console.log result)
  {:db (-> db
           (fork/set-submitting :form false)
           (fork/set-status-code :form 500))}))

(rf/reg-event-fx
 ::submit-onboarding-form
 [(fork/on-submit :form)]
 (fn-traced
  [{db :db} [_ {:keys [values]}]]
  {:db (fork/set-submitting db :form false)
   :http-xhrio
   {:method :post
    :uri "http://127.0.0.1:40666/api"
    :params (cfg->body values)
    :format (ajax/json-request-format)
    :response-format (ajax/raw-response-format)
    :on-success [::success]
    :on-failure [::failure]}}))
