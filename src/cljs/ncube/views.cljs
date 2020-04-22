(ns ncube.views
  (:require
   [re-frame.core :refer [subscribe dispatch]]
   [reitit.core :as r]
   [reitit.frontend.easy :as rfe]))

(defn home-panel
  []
  [:div {:class "mw9 flex flex-column pa2"}
   [:h1 {:class "fh1"} "Ncube Home."]
   [:p {:class "fb1"} "All set to preserve, explore and verify."]])

(defn router-component
  [{:keys [router]}]
  (let [current-route @(subscribe [:current-route])]
    [:div
     (when current-route
       [(-> current-route :data :view)])]))
