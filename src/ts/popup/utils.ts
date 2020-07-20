export const currentUrl = async (): Promise<string> => {
  const tabs = await browser.tabs.query({currentWindow: true, active: true});
  if (tabs.length === 0) {
    throw new Error("Couldn't query current tab.");
  }
  if (tabs[0].url === undefined) {
    throw new Error("Couldn't query current URL.");
  }
  return tabs[0].url;
};
