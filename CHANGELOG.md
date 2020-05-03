<a name="0.1.3"></a>
## 0.1.3 (2020-05-03)




<a name="0.1.2"></a>
## 0.1.2 (2020-05-03)




<a name="0.1.1"></a>
## 0.1.1 (2020-05-03)




<a name="0.1.0"></a>
## 0.1.0 (2020-05-03)


#### Features

*   build platform binaries and debian packages ([194a943a](194a943a))
*   load a html page in the UI app from the HTTP background process ([18d71cff](18d71cff))
* **ncube:**
  *  build DMG installer images for macOS ([004c606f](004c606f))
  *  enable request log tracing ([8194cd12](8194cd12))
  *  start ncubed in the background when starting Ncube. ([6a282645](6a282645), closes [#17](17))
* **ncubed:**
  *  add workspace HTTP endpoints ([aeffe0eb](aeffe0eb))
  *  add global registry for xactor ([0f446dcc](0f446dcc))
  *  add Ncube configuration HTTP endpoints ([4092cf08](4092cf08), closes [#2](2))
  *  query the ncube config over http ([8e4a4da3](8e4a4da3))
  *  fetch ncube config from database ([61b9aeb8](61b9aeb8))
  *  check if ncube config is bootstrapped ([72cc4976](72cc4976))
  *  query root route and dispatch message ([c44653ad](c44653ad))
  *  access the sqlite data base using a pool ([a1f59660](a1f59660))
  *  migrate ncube sql database ([ba02e35b](ba02e35b))
* **ui:**
  *  add workspace part icons ([af42a0f4](af42a0f4))
  *  create local workspaces ([2439d38b](2439d38b))
  *  list all workspaces ([509fae97](509fae97))
  *  onboard the user on app initialization ([4684efd7](4684efd7))
  *  render basic input forms ([ff6729e4](ff6729e4))
  *  style fonts in devcards ([fc455ccb](fc455ccb))
  *  style fonts in devcards ([6c3a3a49](6c3a3a49))
  *  enable url routing on the frontend ([bbc4aa96](bbc4aa96))
  *  bootstrap the css design system ([e14a9f5d](e14a9f5d))

#### Bug Fixes

* **ncubed:**  return 404 Not Found for not bootstrapped Ncube ([ca091c6e](ca091c6e))
* **ui:**  apply base styling for production builds ([12484bcf](12484bcf))



