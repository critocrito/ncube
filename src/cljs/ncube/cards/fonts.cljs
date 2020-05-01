(ns ^:figwheel-hooks ncube.cards.fonts
  (:require
   [reagent.core :as r]
   [devcards.core])
  (:require-macros
   [devcards.core :as dc :refer [defcard defcard-rg]]))


(defcard header-fonts
  (r/as-element
   [:div {:class ["pa4 bg-buzzy-boop flex flex-column"]}
    [:h1 {:class ["back-to-reality header1"]} "Header One"]
    [:h2 {:class ["back-to-reality header2"]} "Header Two"]
    [:h3 {:class ["back-to-reality header3"]} "Header Three"]
    [:h4 {:class ["back-to-reality header4"]} "Header Four"]
    [:h5 {:class ["back-to-reality header5"]} "Header Five"]]))

(defcard text
  (r/as-element
   [:div {:class ["bg-buzzy-boop pa4"]}
    [:p "Cumque vel corporis cum animi rerum. Adipisci fuga voluptas temporibus maxime. Nihil praesentium accusantium et. Cumque vel corporis cum animi rerum. Adipisci fuga voluptas temporibus maxime. Nihil praesentium accusantium et. Cumque vel corporis cum animi rerum. Adipisci fuga voluptas temporibus maxime. Nihil praesentium accusantium et. Cumque vel corporis cum animi rerum. Adipisci fuga voluptas temporibus maxime. Nihil praesentium accusantium et. Cumque vel corporis cum animi rerum. Adipisci fuga voluptas temporibus maxime. Nihil praesentium accusantium et. Cumque vel corporis cum animi rerum. Adipisci fuga voluptas temporibus maxime. Nihil praesentium accusantium et. Cumque vel corporis cum animi rerum. Adipisci fuga voluptas temporibus maxime. Nihil praesentium accusantium et."]]))

(defcard text-bold
  (r/as-element
   [:div {:class ["bg-buzzy-boop pa4"]}
    [:p {:class ["b"]} "Cumque vel corporis cum animi rerum. Adipisci fuga voluptas temporibus maxime. Nihil praesentium accusantium et. Cumque vel corporis cum animi rerum. Adipisci fuga voluptas temporibus maxime. Nihil praesentium accusantium et. Cumque vel corporis cum animi rerum. Adipisci fuga voluptas temporibus maxime. Nihil praesentium accusantium et. Cumque vel corporis cum animi rerum. Adipisci fuga voluptas temporibus maxime. Nihil praesentium accusantium et. Cumque vel corporis cum animi rerum. Adipisci fuga voluptas temporibus maxime. Nihil praesentium accusantium et. Cumque vel corporis cum animi rerum. Adipisci fuga voluptas temporibus maxime. Nihil praesentium accusantium et. Cumque vel corporis cum animi rerum. Adipisci fuga voluptas temporibus maxime. Nihil praesentium accusantium et."]]))

(defcard text-uppercase-bold
  (r/as-element
   [:div {:class ["bg-buzzy-boop pa4"]}
    [:p {:class ["b ttu"]} "Cumque vel corporis cum animi rerum. Adipisci fuga voluptas temporibus maxime. Nihil praesentium accusantium et. Cumque vel corporis cum animi rerum. Adipisci fuga voluptas temporibus maxime. Nihil praesentium accusantium et. Cumque vel corporis cum animi rerum. Adipisci fuga voluptas temporibus maxime. Nihil praesentium accusantium et. Cumque vel corporis cum animi rerum. Adipisci fuga voluptas temporibus maxime. Nihil praesentium accusantium et. Cumque vel corporis cum animi rerum. Adipisci fuga voluptas temporibus maxime. Nihil praesentium accusantium et. Cumque vel corporis cum animi rerum. Adipisci fuga voluptas temporibus maxime. Nihil praesentium accusantium et. Cumque vel corporis cum animi rerum. Adipisci fuga voluptas temporibus maxime. Nihil praesentium accusantium et."]]))

(defcard text-medium
  (r/as-element
   [:div {:class ["bg-buzzy-boop pa4"]}
    [:p {:class ["text-medium"]} "Cumque vel corporis cum animi rerum. Adipisci fuga voluptas temporibus maxime. Nihil praesentium accusantium et. Cumque vel corporis cum animi rerum. Adipisci fuga voluptas temporibus maxime. Nihil praesentium accusantium et. Cumque vel corporis cum animi rerum. Adipisci fuga voluptas temporibus maxime. Nihil praesentium accusantium et. Cumque vel corporis cum animi rerum. Adipisci fuga voluptas temporibus maxime. Nihil praesentium accusantium et. Cumque vel corporis cum animi rerum. Adipisci fuga voluptas temporibus maxime. Nihil praesentium accusantium et. Cumque vel corporis cum animi rerum. Adipisci fuga voluptas temporibus maxime. Nihil praesentium accusantium et. Cumque vel corporis cum animi rerum. Adipisci fuga voluptas temporibus maxime. Nihil praesentium accusantium et."]]))

(defcard text-medium-bold
  (r/as-element
   [:div {:class ["bg-buzzy-boop pa4"]}
    [:p {:class ["text-medium b"]} "Cumque vel corporis cum animi rerum. Adipisci fuga voluptas temporibus maxime. Nihil praesentium accusantium et. Cumque vel corporis cum animi rerum. Adipisci fuga voluptas temporibus maxime. Nihil praesentium accusantium et. Cumque vel corporis cum animi rerum. Adipisci fuga voluptas temporibus maxime. Nihil praesentium accusantium et. Cumque vel corporis cum animi rerum. Adipisci fuga voluptas temporibus maxime. Nihil praesentium accusantium et. Cumque vel corporis cum animi rerum. Adipisci fuga voluptas temporibus maxime. Nihil praesentium accusantium et. Cumque vel corporis cum animi rerum. Adipisci fuga voluptas temporibus maxime. Nihil praesentium accusantium et. Cumque vel corporis cum animi rerum. Adipisci fuga voluptas temporibus maxime. Nihil praesentium accusantium et."]]))

(defcard text-small
  (r/as-element
   [:div {:class ["bg-buzzy-boop pa4"]}
    [:p {:class ["text-small"]} "Cumque vel corporis cum animi rerum. Adipisci fuga voluptas temporibus maxime. Nihil praesentium accusantium et. Cumque vel corporis cum animi rerum. Adipisci fuga voluptas temporibus maxime. Nihil praesentium accusantium et. Cumque vel corporis cum animi rerum. Adipisci fuga voluptas temporibus maxime. Nihil praesentium accusantium et. Cumque vel corporis cum animi rerum. Adipisci fuga voluptas temporibus maxime. Nihil praesentium accusantium et. Cumque vel corporis cum animi rerum. Adipisci fuga voluptas temporibus maxime. Nihil praesentium accusantium et. Cumque vel corporis cum animi rerum. Adipisci fuga voluptas temporibus maxime. Nihil praesentium accusantium et. Cumque vel corporis cum animi rerum. Adipisci fuga voluptas temporibus maxime. Nihil praesentium accusantium et."]]))

(defcard text-tiny
  (r/as-element
   [:div {:class ["bg-buzzy-boop pa4"]}
    [:p {:class ["text-tiny"]} "Cumque vel corporis cum animi rerum. Adipisci fuga voluptas temporibus maxime. Nihil praesentium accusantium et. Cumque vel corporis cum animi rerum. Adipisci fuga voluptas temporibus maxime. Nihil praesentium accusantium et. Cumque vel corporis cum animi rerum. Adipisci fuga voluptas temporibus maxime. Nihil praesentium accusantium et. Cumque vel corporis cum animi rerum. Adipisci fuga voluptas temporibus maxime. Nihil praesentium accusantium et. Cumque vel corporis cum animi rerum. Adipisci fuga voluptas temporibus maxime. Nihil praesentium accusantium et. Cumque vel corporis cum animi rerum. Adipisci fuga voluptas temporibus maxime. Nihil praesentium accusantium et. Cumque vel corporis cum animi rerum. Adipisci fuga voluptas temporibus maxime. Nihil praesentium accusantium et."]]))
