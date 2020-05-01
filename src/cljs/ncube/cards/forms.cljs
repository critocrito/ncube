(ns ^:figwheel-hooks ncube.cards.forms
  (:require
   [reagent.core :as r]
   [devcards.core]
   [ncube.components :refer [text-input btn-large btn-small]])
  (:require-macros
   [devcards.core :as dc :refer [defcard defcard-rg]]))

(defcard large-buttons
  (r/as-element
   [:div {:class ["flex flex-column bg-buzzy-boop pa3"]}
    (btn-large {:label "Primary Button"})
    (btn-large {:label "Secondary Button" :style :secondary})
    (btn-large {:label "Caution Button" :style :caution})
    (btn-large {:label "Disabled Button" :disabled true})]))

(defcard small-buttons
  (r/as-element
   [:div {:class ["flex flex-column bg-buzzy-boop pa3"]}
    (btn-small {:label "Primary"})
    (btn-small {:label "Secondary" :style :secondary})
    (btn-small {:label "Caution" :style :caution})
    (btn-small {:label "Disabled" :disabled true})]))

(defcard input-forms
  (r/as-element
   [:div {:class ["flex flex-column bg-buzzy-boop pa3"]}
    [:div {:class ["ma3"]}
     (text-input {:name "basic-input"
                  :label "Basic Input"})]
    [:div {:class ["ma3"]}
     (text-input {:name "input-with-placeholder"
                  :label "Input with placeholder"
                  :placeholder "pick your fruit"})]
    [:div {:class ["ma3"]}
     (text-input {:name "input-with-default-value"
                  :label "Input with default value"
                  :value "banana"})]
    [:div {:class ["ma3"]}
     (text-input {:name "disabled-input"
                  :label "Disabled input"
                  :disabled true
                  :value "banana"})]]))
