(ns ncube.workspaces.events
  (:require [re-frame.core :as rf]
            [day8.re-frame.tracing :refer-macros [fn-traced defn-traced]]
            [ajax.core :as ajax]
            [fork.core :as fork]))

(rf/reg-event-fx
 ::create-workspace
 (fn-traced
  [_ _]
  {:navigate! :workspaces-create}))


(rf/reg-event-fx
 ::success
 [(fork/clean :form)]
 (fn-traced
  [_ [_ result]]
  {:dispatch [:fetch-workspaces]
   :navigate! :home}))


(rf/reg-event-fx
 ::failure
 (fn-traced
  [{db :db} [_ result]]
  (js/console.log result)
  {:db (-> db
           (fork/set-submitting :form false)
           (fork/set-status-code :form 500))}))

(rf/reg-event-fx
 ::submit-create-workspace-form
 [(fork/on-submit :form)]
 (fn-traced
  [{db :db} [_ {:keys [values]}]]
  (let [req-body {:name (values "name")
                  :kind "local"
                  :description (values "description")}]
    {:db (fork/set-submitting db :form false)
   :http-xhrio
   {:method :post
    :uri "http://127.0.0.1:40666/api/workspaces"
    :params req-body
    :format (ajax/json-request-format)
    :response-format (ajax/raw-response-format)
    :on-success [::success]
    :on-failure [::failure]}})))
