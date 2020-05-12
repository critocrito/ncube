<a name="0.2.1"></a>
## 0.2.1 (2020-05-12)


#### Features

*   build platform binaries and debian packages ([194a943a](194a943a))
*   load a html page in the UI app from the HTTP background process ([18d71cff](18d71cff))
* **ncube:**
  *  log configuration path when starting ([f1d53c9e](f1d53c9e))
  *  store application config in platform directories ([774c9048](774c9048))
  *  new logo, less pointy ([914b71cf](914b71cf))
  *  build DMG installer images for macOS ([004c606f](004c606f))
  *  enable request log tracing ([8194cd12](8194cd12))
  *  start ncubed in the background when starting Ncube. ([6a282645](6a282645), closes [#17](17))
* **ncubed:**
  *  wrap HTTP responses in an envelope ([8aba3f27](8aba3f27))
  *  add a database cache to reuse connection pools ([e88cec97](e88cec97))
  *  package ffmpeg and youtubedl into the workspace ([da8c0552](da8c0552))
  *  migrate the workspace sqlite database ([5bd5e988](5bd5e988))
  *  install sugarcube for new workspaces ([1f39e7e8](1f39e7e8))
  *  build a workspace archive file including NodeJS ([d2628b87](d2628b87))
  *  create workspace from zipped template ([06e1bc46](06e1bc46))
  *  build the template workspace as a zip file ([56fe8a43](56fe8a43))
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
  *  add cancel button to the create workspace form ([4c5cb571](4c5cb571))
  *  render the workspace details screen ([9a141815](9a141815))
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

*   remove debug mode of workspace build script ([fdeb97b3](fdeb97b3))
* **ncubed:**
  *  serialization error when creating workspaces ([542b546f](542b546f))
  *  return 404 Not Found for not bootstrapped Ncube ([ca091c6e](ca091c6e))
* **ui:**
  *  fix icon urls in workspace details ([4220bacb](4220bacb))
  *  apply base styling for production builds ([12484bcf](12484bcf))



<a name="0.2.0"></a>
## 0.2.0 (2020-05-11)




<a name="0.1.4"></a>
## 0.1.4 (2020-05-03)




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



