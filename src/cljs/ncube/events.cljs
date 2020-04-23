(ns ncube.events
  (:require
   [re-frame.core :refer [reg-event-db reg-event-fx]]
   [re-frame.std-interceptors :refer [debug]]
   [day8.re-frame.tracing :refer-macros [fn-traced defn-traced]]
   [reitit.core :as r]
   [reitit.frontend.easy :as rfe]
   [ncube.db :refer [default-db]]
   [ncube.router :refer [router]]
   [ajax.core :as ajax]))

(defn boot-flow
  []
  {:first-dispatch [:bootstrapped?]
   :rules
   [{:when :seen? :events :bootstrap-found}
    {:when :seen? :events :bootstrap}]})

(reg-event-fx
 :boot
 (fn-traced
  [_ _]
  (js/console.log "initialize")
  {:db default-db
   :async-flow (boot-flow)}))

(reg-event-fx
 :bootstrapped?
 (fn-traced
  [_ _]
  {:http-xhrio {:method :get
                :uri "http://127.0.0.1:40666/api"
                :response-format (ajax/json-response-format {:keywords? true})
                :on-success [:bootstrap-found]
                :on-failure [:bootstrap]}}))

(reg-event-fx
 :bootstrap-found
 (fn-traced
  [_ _]
  (let [matched-route (r/match-by-path router (.. js/document -location -pathname))
        ]
    (if matched-route
      {:navigate! (-> matched-route :data :name)}
      {:navigate! :home}))))

(reg-event-fx
 :bootstrap
 (fn-traced
  [_ _]
  {:navigate! :onboarding}))

(reg-event-db
 :navigated
 (fn-traced [db [_ new-match]]
   (assoc db :current-route new-match)))
