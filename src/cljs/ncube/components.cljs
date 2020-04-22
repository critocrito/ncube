(ns ncube.components)

(defn text-input
  "A typical input for element."
  [{:keys [label placeholder value disabled name on-change on-blur]}]
  (js/console.log on-change)
  [:div {:class ["flex flex-column fb1 w-two-thirds"]}
   [:label {:for name :class ["mb3"]} label]
   [:input {:class ["fb1 pa2 ba b--barrier"]
            :name name
            :placeholder placeholder
            :value value
            :disabled disabled
            :on-change on-change
            :on-blur on-blur}]])
