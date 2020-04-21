(ns ^:figwheel-hooks ncube.cards.forms
  (:require
   [reagent.core :as r]
   [devcards.core]
   [ncube.components :refer [text-input]])
  (:require-macros
   [devcards.core :as dc :refer [defcard defcard-rg]]))

(defcard buttons
  (r/as-element
   [:div {:class ["flex flex-column bg-buzzy-boop pa3"]}
    [:a {:class ["btn-primary"]} "Primary Button"]
    [:a {:class ["btn-secondary"]} "Secondary Button"]
    [:a {:class ["btn-caution"]} "Caution Button"]
    [:a {:class ["btn-disabled"] :disabled true} "Disabled Button"]]))

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
