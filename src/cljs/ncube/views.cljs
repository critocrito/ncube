(ns ncube.views
  (:require
   [re-frame.core :refer [subscribe dispatch]]
   [reitit.core :as r]
   [ncube.utils :refer [href]]))

(defn home-panel
  []
  (let [name "Home"]
    (fn []
      [:div name])))

(defn bootstrap-panel
  []
  (let [name "Bootstrap"]
    (fn []
      [:div name])))

(defn nav [{:keys [router current-route]}]
  [:ul
   (for [route-name (r/route-names router)
         :let       [route (r/match-by-name router route-name)
                     text (-> route :data :link-text)]]
     [:li {:key route-name}
       (when (= route-name (-> current-route :data :name))
         "> ")
      [:a {:href (href route-name)} (name route-name)]])])

(defn router-component
  [{:keys [router]}]
  (let [current-route @(subscribe [:current-route])]
    [:div
     [nav {:router router :current-route current-route}]
     (when current-route
       [(-> current-route :data :view)])]))
