module.exports = {
  name: "ncube-discovery",

  manifest_version: 2,
  icons: {
    "48": "icons/sd-48.png",
  },

  permissions: ["tabs", "activeTab", "storage"],

  browser_action: {
    browser_style: true,
    default_icon: "icons/sd-48.png",
    default_title: "Ncube Discovery",
    default_popup: "popup.html",
  },

  applications: {
    gecko: {
      id: "christo@cryptodrunks.net",
    },
  },
};
