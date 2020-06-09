(ns ncube.components
  (:require [re-frame.core :as rf]))

(defn text-input
  "A typical input for element."
  [{:keys [label placeholder value disabled name on-change on-blur]}]
  [:div {:class ["flex flex-column fb1 mt3 mb2 w-two-thirds"]}
   [:label {:for name :class ["mb1"]} label]
   [:input {:class ["fb1 pa2 ba b--barrier"]
            :name name
            :placeholder placeholder
            :value value
            :disabled disabled
            :on-change on-change
            :on-blur on-blur}]])

(defn btn-large
  "A large button."
  [{:keys [label disabled name on-click style]}]
  (let [styled-button (cond
                        (= style :secondary) "btn-secondary"
                        (= style :caution) "btn-caution"
                        :else "btn-primary")]
    [:button {:class ["btn large" styled-button (when disabled "btn-disabled")]
              :disabled disabled
              :on-click on-click}
     label]))

(defn btn-small
  "A button component."
  [{:keys [label disabled name on-click style]}]
  (let [styled-button (cond
                        (= style :secondary) "btn-secondary"
                        (= style :caution) "btn-caution"
                        :else "btn-primary")]
    [:button {:class ["btn small" styled-button (when disabled "btn-disabled")]
              :disabled disabled
              :on-click on-click}
     label]))

(defn tag
  "A label tag."
  [{:keys [label style]}]
  (let [tag-style (cond
                    (= style :local) "bg-local-workspace"
                    (= style :remote) "bg-remote-workspace")]
    [:div {:class ["tag flex flex-column justify-around ma1 br4 back-to-reality text-middle tc noto b ttu" tag-style]}
     [:span
      label]]))

(defn desc-text
  [text]
  [:div {:class "h4 pa2"}
   text])

(defn overline
  [label]
  [:div {:class "b bb b--back-to-reality ttu back-to-reality pb"}
   (str label ":")])

(defn navbar
  []
  [:div {:class "flex justify-between bg-white"}
   [:button {:class "link back-to-reality text-medium pl2 pr2"
             :on-click #(rf/dispatch [:history-back])} "< Back"]
   [:div {:class "back-to-reality text-medium pl2 pr2 bg-nasty-color"} "Process console"]])

(defn workspace-header
  [workspace]
  [:div {:class "bb b--back-to-reality w-100 flex justify-between items-center"}
   [:div {:class "b text-medium back-to-reality ttu"} "Workspace: " (:name workspace)]
   [tag {:label "Remote" :style :remote}]])

(defn page-header
  [title description]
  [:div
   [:h1 {:class "header1 back-to-reality"} title]
   [:p {:class "text-medium"} description]])

(defn sidebar
  []
  [:div {:class ["sidebar w5 vh-100 flex flex-column justify-between"] }
   [:div {:class "bg-back-to-reality h4"} "Workspace Selector"]
   [:div {:class "bg-white flex flex-column justify-between h-100"}
    [:div]
    [:div]]])

(defn panel
  [{:keys [sidebar? title description workspace]} children]
  [:div {:class "flex"}
   [:div {:class ["absolute"]
          :style {:top "50%"
                  :left (if sidebar? "238px" "0px")}}
    [:button {:class "link back-to-reality b bg-white shadow-1"
              :on-click #(rf/dispatch [:toggle-sidebar])} (if sidebar? "<<" ">>")]]
   [:div {:class [(when-not sidebar? "dn")]}
    [sidebar]]
   [:div {:class ["h5 w-100"]}
    [navbar]
    [:div {:class ["pa3 ma2 mw8 center" (when-not sidebar? "mw7")]}
     [workspace-header workspace]
     [page-header title description]
     children]]])
