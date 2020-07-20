<a name="0.6.1"></a>
## 0.6.1 (2020-07-20)




<a name="0.6.0"></a>
## 0.6.0 (2020-07-20)


#### Bug Fixes

* **ncubed:**
  *  handle empty responses for remote HTTP requests ([0d85bf3c](0d85bf3c))
  *  switch http client to allow HTTP redirects and TLS ([455bb7f5](455bb7f5))
  *  authenticate remote requests for stats, units and source tags ([659bcc03](659bcc03))
  *  account creation and password verification ([bf490b16](bf490b16))
* **ui:**
  *  allow to cancel linking new workspaces ([a12d4b07](a12d4b07))
  *  handle missing stats state in workspace screen ([1cd27e11](1cd27e11))
  *  render URL decoded in the data table ([3854a682](3854a682))
  *  use className as attribute name over class ([4cb302e5](4cb302e5))
  *  center sidebar navigation buttons ([31b45b9f](31b45b9f))

#### Features

*   scaffold the Ncube discovery browser extension ([46866667](46866667))
*   extend tables with a search function and enable data search ([42306df8](42306df8))
* **ncubed:**
  *  proxy remote data and sources database requests ([1dc4bdc7](1dc4bdc7))
  *  create search indices, extract search into it's own store ([1a0c5535](1a0c5535))
  *  paginated list of sources and expanded stats ([e53f52da](e53f52da))
  *  list units of data with pagination ([eded89ee](eded89ee))
  *  query stats about the data units ([b215de02](b215de02))
* **ui:**
  *  refactor source tag ui and use a multi select widget ([acdb2247](acdb2247))
  *  paginated source and data tables ([57c4672b](57c4672b))
  *  render data stats ([6671f3fd](6671f3fd))
  *  query stats about sources ([dd08b97e](dd08b97e))
  *  list and create tags for sources ([74249aa9](74249aa9))
  *  data table with selection, filters, pagination and delete modal ([979d56bc](979d56bc))
* **web-ext:**  add browser extension for source discovery and send them to Ncube ([ef4454bc](ef4454bc))



<a name="0.5.0"></a>
## 0.5.0 (2020-06-11)


#### Bug Fixes

*   respect -v flag for ncubed and ncubectl ([7b5c0795](7b5c0795))
* **ncubed:**
  *  include remote workspace endpoint in the connection object ([65882bbe](65882bbe))
  *  normalized sql include path in config store ([e9c0af05](e9c0af05))
  *  store database paths expanded ([7d2488bf](7d2488bf))
* **ui:**  send the name field when creating local workspaces ([69e31543](69e31543))

#### Features

* **ncubectl:**
  *  export connection details ([0bdc4472](0bdc4472))
  *  get and set configurations settings ([7ebb48a4](7ebb48a4))
  *  specify the path to the database ([0c045451](0c045451))
  *  create/list workspaces and accounts ([6a8c5854](6a8c5854))
* **ncubed:**
  *  finalize the account initializtion workflow ([145fbfba](145fbfba))
  *  allow login for http database backends ([3fd8827a](3fd8827a))
  *  create local account when creating a remote workspace ([93b3b8a6](93b3b8a6))
  *  add a config setting for the endpoint ([912c5c0b](912c5c0b))
  *  add enduser api for HTTP database ([7119b9dc](7119b9dc))
  *  add a HTTP database backend ([bac46d84](bac46d84))
  *  restrict access to endpoints ([67eeceb6](67eeceb6))
  *  read configuration from the command line arguments ([0af05061](0af05061))
  *  create remote workspaces using the HTTP api ([b5588394](b5588394))
  *  verify JWT tokens in HTTP requests ([16bdeb23](16bdeb23))
  *  add support for secret key in setting ([2f34d2bc](2f34d2bc))
  *  login account over HTTP and issue JWT tokens ([0563bec4](0563bec4))
  *  update passwords using an HTTP endpoint ([d4897f5c](d4897f5c))
  *  handler to verify a password login ([6077c208](6077c208))
  *  create accounts for a workspace ([77676bf1](77676bf1))
  *  run ncubed without an HTTP server ([5cc93d92](5cc93d92))
* **ui:**  upload connection files to link remote workspaces ([3255cad7](3255cad7))



<a name="0.4.0"></a>
## 0.4.0 (2020-05-25)


#### Bug Fixes

* **ui:**  bad pointer cursor style ([ef62550a](ef62550a))

#### Features

* **ncubed:**
  *  add HTTP endpoint to update sources for a workspace ([a2800007](a2800007))
  *  add HTTP endpoint to delete a source for a workspace ([72225c3a](72225c3a))
  *  add HTTP endpoint to list sources for a workspace ([2b567855](2b567855))
* **ui:**
  *  select sources ([3f5fe9ec](3f5fe9ec))
  *  create sources ([de53de9f](de53de9f))
  *  list and delete sources for a workspace ([8baed1bd](8baed1bd))



<a name="0.3.0"></a>
## 0.3.0 (2020-05-19)


#### Features

* **ncubed:**
  *  add HTTP endpoint to create sources ([2998e2dc](2998e2dc))
  *  add a database cache actor ([dc2f38fa](dc2f38fa))
  *  register workspace databases on the host ([26873ec1](26873ec1))

#### Bug Fixes

* **ncube-data:**  format sqlite connection string ([4e37ef3b](4e37ef3b))
* **ncubed:**  wrong parameter count in update workspace sql ([023a6b96](023a6b96))



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



