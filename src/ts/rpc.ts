interface MyExternal extends External {
  invoke: <T extends unknown>(arg: T) => void;
}

/*
 * The RPC calls communicates to the Rust/WebView host through message passing.
 */
const rpc = {
  // Do the actual call to the host.
  invoke: <T extends unknown>(arg: T): void => {
    const {invoke} = window.external as MyExternal;
    if (invoke) invoke(JSON.stringify(arg));
  },

  // Open an external URL in the default browser.
  openExternal: (url: string): void => rpc.invoke({cmd: "url", url}),
};

export default rpc;
