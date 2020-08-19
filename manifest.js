module.exports = {
  name: "ncube-discovery",

  version: "0.1.1",

  manifest_version: 2,
  icons: {
    "48": "icons/logo.png",
    "96": "icons/logo@2x.png",
  },

  permissions: ["activeTab"],

  browser_action: {
    browser_style: true,
    default_icon: "icons/logo.png",
    default_title: "Ncube Discovery",
    default_popup: "popup.html",
  },
};
