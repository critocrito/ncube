(ns ncube.components)

(defn text-input
  "A typical input for element."
  [{:keys [label placeholder value disabled name on-change on-blur]}]
  [:div {:class ["flex flex-column fb1 w-two-thirds"]}
   [:label {:for name :class ["mb3"]} label]
   [:input {:class ["fb1 pa2 ba b--barrier"]
            :name name
            :placeholder placeholder
            :value value
            :disabled disabled
            :on-change on-change
            :on-blur on-blur}]])

(defn btn-large
  "A large button."
  [{:keys [label disabled name on-submit style]}]
  (let [styled-button (cond
                        (= style :secondary) "btn-secondary"
                        (= style :caution) "btn-caution"
                        :else "btn-primary")]
    [:button {:class ["btn large" styled-button (when disabled "btn-disabled")]
              :disabled disabled}
     label]))

(defn btn-small
  "A button component."
  [{:keys [label disabled name on-submit style]}]
  (let [styled-button (cond
                        (= style :secondary) "btn-secondary"
                        (= style :caution) "btn-caution"
                        :else "btn-primary")]
    [:button {:class ["btn small" styled-button (when disabled "btn-disabled")]
              :disabled disabled}
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
