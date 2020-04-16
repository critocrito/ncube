(ns ^:figwheel-hooks ncube.cards.colors
  (:require
   [reagent.core :as r]
   [devcards.core])
  (:require-macros
   [devcards.core :as dc :refer [defcard defcard-rg]]))

(defcard main-background
  (r/as-element [:div {:class ["bg-buzzy-boop h5"]}]))

(defcard secondary-background
  (r/as-element [:div {:class ["bg-white h5"]}]))

(defcard tertiary-background
  (r/as-element [:div {:class ["bg-back-to-reality h5"]}]))

(defcard color-palette
  (r/as-element [:div {:class ["bg-buzzy-boop flex flex-column w-100 pa3 tc"]}
                 [:div {:class ["flex"]}
                  [:div {:class ["pa3 ma3 bg-fresh-frivolous"]}
                   [:h2 {:class ["tc"]} "Primary"]
                   [:p "In visual perception a color is almost never seen as it
                  really is – as it physically is. This fact makes color the
                  most relative medium in art. " [:i "- Josef Albers"]]]
                  [:div {:class ["pa3 ma3 bg-back-to-reality"]}
                   [:h2 {:class ["tc"]} "Secondary"]
                   [:p "In visual perception a color is almost never seen as it
                  really is – as it physically is. This fact makes color the
                  most relative medium in art. " [:i "- Josef Albers"]]]]

                 [:div {:class ["flex"]}
                  [:div {:class ["pa3 ma3 bg-nasty-color"]}
                   [:h2 {:class ["tc"]} "Disabled"]
                   [:p "In visual perception a color is almost never seen as it
                  really is – as it physically is. This fact makes color the
                  most relative medium in art. " [:i "- Josef Albers"]]]
                  [:div {:class ["pa3 ma3 bg-black white"]}
                   [:h2 {:class ["tc"]} "Caution"]
                   [:p "In visual perception a color is almost never seen as it
                  really is – as it physically is. This fact makes color the
                  most relative medium in art. " [:i "- Josef Albers"]]]]

                 [:div {:class ["flex"]}
                  [:div {:class ["pa3 ma3 bg-success"]}
                   [:h2 {:class ["tc"]} "Success"]
                   [:p "In visual perception a color is almost never seen as it
                  really is – as it physically is. This fact makes color the
                  most relative medium in art. " [:i "- Josef Albers"]]]
                  [:div {:class ["pa3 ma3 bg-warning"]}
                   [:h2 {:class ["tc"]} "Warning"]
                   [:p "In visual perception a color is almost never seen as it
                  really is – as it physically is. This fact makes color the
                  most relative medium in art. " [:i "- Josef Albers"]]]]
                 
                 [:div {:class ["flex"]}
                  [:div {:class ["pa3 ma3 bg-error"]}
                   [:h2 {:class ["tc"]} "Error"]
                   [:p "In visual perception a color is almost never seen as it
                  really is – as it physically is. This fact makes color the
                  most relative medium in art. " [:i "- Josef Albers"]]]
                  [:div {:class ["pa3 ma3 bg-info"]}
                   [:h2 {:class ["tc"]} "Info"]
                   [:p "In visual perception a color is almost never seen as it
                  really is – as it physically is. This fact makes color the
                  most relative medium in art. " [:i "- Josef Albers"]]]]
                 
                 [:div {:class ["flex"]}
                  [:div {:class ["pa3 ma3 bg-new"]}
                   [:h2 {:class ["tc"]} "New"]
                   [:p "In visual perception a color is almost never seen as it
                  really is – as it physically is. This fact makes color the
                  most relative medium in art. " [:i "- Josef Albers"]]]]]))
