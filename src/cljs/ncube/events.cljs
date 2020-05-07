(ns ncube.events
  (:require
   [re-frame.core :as rf]
   [re-frame.std-interceptors :refer [debug]]
   [day8.re-frame.tracing :refer-macros [fn-traced defn-traced]]
   [reitit.core :as r]
   [reitit.frontend.easy :as rfe]
   [ncube.db :refer [default-db]]
   [ncube.router :refer [router]]
   [ncube.workspaces.events :as workspaces]
   [ajax.core :as ajax]))

(defn boot-flow
  []
  {:first-dispatch [:bootstrapped?]
   :rules
   [{:when :seen? :events :bootstrap-found}
    {:when :seen? :events :bootstrap}]})

(rf/reg-event-fx
 :boot
 (fn-traced
  [_ _]
  (js/console.log "initialize")
  {:db default-db
   :async-flow (boot-flow)}))

(rf/reg-event-fx
 :bootstrapped?
 (fn-traced
  [_ _]
  {:http-xhrio {:method :get
                :uri "http://127.0.0.1:40666/api"
                :response-format (ajax/json-response-format {:keywords? true})
                :on-success [:bootstrap-found]
                :on-failure [:bootstrap]}}))

(rf/reg-event-fx
 :bootstrap-found
 (fn-traced
  [_ _]
  (let [matched-route (r/match-by-path router (.. js/document -location -pathname))
        navigation-target (cond
                            (= (and matched-route (-> matched-route :data :name)) :onboarding) :home
                            matched-route (-> matched-route :data :name)
                            :else :home)]
    {:navigate! navigation-target
     :dispatch [::workspaces/fetch-workspaces]})))

(rf/reg-event-fx
 :bootstrap
 (fn-traced
  [_ _]
  {:navigate! :onboarding}))

(rf/reg-event-db
 :navigated
 (fn-traced [db [_ new-match]]
   (assoc db :current-route new-match)))

(rf/reg-event-fx
 :navigate
 (fn-traced
  [_ [_ route]]
  {:navigate! route}))


(rf/reg-event-fx
 :unimplemented
 [debug]
 (fn-traced
  [_ _]))

(rf/reg-event-fx
 :http-error
 [debug]
 (fn-traced
  [_ _]))

(rf/reg-event-db
 :toggle-sidebar
 (fn-traced
  [db _]
  (let [sidebar? (:sidebar? db)]
    (assoc db :sidebar? (not sidebar?)))))

(rf/reg-event-fx
 :history-back
 (fn-traced
  [_ _]
  {:history :back}))
