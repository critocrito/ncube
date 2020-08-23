<a name="0.9.0"></a>
## 0.9.0 (2020-08-23)


#### Features

* **ncubed:**
  *  list units by ids ([fb7ad41f](fb7ad41f))
  *  add HTTP endpoint to show a single data unit ([bf1d5934](bf1d5934))
* **ui:**
  *  download verification units as a CSV ([fdb394c6](fdb394c6))
  *  edit annotations during investigations ([a553dc4b](a553dc4b))
  *  define and render annotation templates ([49fa7dd0](49fa7dd0))
  *  show unit details during verification ([cf4ed5f1](cf4ed5f1))
  *  add a help screen for the search grammar ([5b136f8a](5b136f8a))

#### Bug Fixes

* **ui:**  replace source icon ([8700b4a2](8700b4a2))



<a name="0.8.0"></a>
## 0.8.0 (2020-08-19)


#### Features

* **ncube:**  open the project website when clicking on the home logo ([49383cb5](49383cb5))
* **ncubed:**
  *  extend search to limit results by tag and source ([02b1ad41](02b1ad41))
  *  stats for workspace wide verified and in process data ([ea9941d9](ea9941d9))
  *  fetch tags for data units ([654efc2f](654efc2f))
  *  add stat endpoint for verification data in progress ([efc4afdb](efc4afdb))
  *  add stat endpoint for verified data units of a segment ([d18d0fd5](d18d0fd5))
  *  update the state of a unit in verification ([91655160](91655160))
  *  list units for a segment that is part of an investigation ([900e9ce9](900e9ce9))
  *  verified count stat for investigations ([06c8a38f](06c8a38f))
  *  add stat of total data units for an investigation ([d987d68e](d987d68e))
  *  add stat of total segments for an investigation ([04c7c957](04c7c957))
  *  list segments for an investigation ([369a05ce](369a05ce))
  *  send segments to verify ([3a1f63d2](3a1f63d2))
  *  add total investigations stat ([19dd543b](19dd543b))
  *  add endpoints for methodologies and investigations ([53e76651](53e76651))
* **ui:**
  *  show query tags in data table ([754d49b1](754d49b1))
  *  render query tags for data units ([b29a7cfa](b29a7cfa))
  *  check if workspaces are created before allowing to open them ([6005b01f](6005b01f))
  *  humanize stat output ([f50e26e4](f50e26e4))
  *  render verification as a kanban board ([4a902147](4a902147))
  *  fetch data in progress for segment stat ([328582be](328582be))
  *  fetch verified data for segment stat ([a609a6ed](a609a6ed))
  *  fetch verified count stat for investigations ([bb4c4144](bb4c4144))
  *  list segments for an investigation ([dbf9b1d4](dbf9b1d4))
  *  scaffold interface copy using MDX ([8fa83474](8fa83474))
  *  fetch data count stat for investigations ([6f370cd1](6f370cd1))
  *  fetch segments count stat for investigations ([7e89b88d](7e89b88d))
  *  add total investigations stat to home ([71ffd69c](71ffd69c))
  *  fetch workspace data and sources stats on home screen ([5395f0f7](5395f0f7))
  *  send segments to a verification ([80f1e761](80f1e761))
  *  fetch units stat for segments ([79958f7a](79958f7a))
  *  create investigations ([c533e94f](c533e94f))
  *  go to workspace overview when clicking on breadcrumb ([4018274d](4018274d))
  *  add external link component ([03bb844b](03bb844b))
  *  list investigations ([092f49a3](092f49a3))
  *  use chevron icons for sidebar button ([b998c5ee](b998c5ee))
  *  add visual queue when copying to clipboard ([6d554b8f](6d554b8f))
* **workspace:**  upgrade to sugarcube 0.42.0 ([06f1d80f](06f1d80f))

#### Bug Fixes

* **ncubed:**  avoid duplicate investigation segments ([4d3f0bd0](4d3f0bd0))
* **ui:**
  *  use the href attribute to call external links ([775be970](775be970))
  *  prevent empty tags to be added to sources ([64bd43f4](64bd43f4))
  *  reload search results after updating a segment ([99c2b37c](99c2b37c))
  *  remove duplicate verification card when moving them around ([a1624307](a1624307))
  *  align data card table ([0efd3887](0efd3887))
  *  have consistent card table layout ([1ce3fc1e](1ce3fc1e))
  *  smoothened the investigation card table ([559f486c](559f486c))
  *  smoothened the investigation card table ([59ff8374](59ff8374))
  *  ignore errors when fetching stats ([99fac9d3](99fac9d3))
  *  clean table on process card ([f004e201](f004e201))
  *  don't mix singular and plural in stat descriptions ([b50948aa](b50948aa))
  *  dim sidebar and breadcrumbs links ([1c1b2e97](1c1b2e97))
  *  show hand cursor when hovering sidebar buttons ([f532dd16](f532dd16))
  *  correct a fatal error message for processes ([3d4b1efd](3d4b1efd))
  *  removed none existing css class name ([45f1f02e](45f1f02e))



<a name="0.7.0"></a>
## 0.7.0 (2020-08-10)


#### Bug Fixes

*   respect -v flag for ncubed and ncubectl ([1ca4205f](1ca4205f))
*   remove debug mode of workspace build script ([7a2df985](7a2df985))
* **ncube-data:**  format sqlite connection string ([9db462ec](9db462ec))
* **ncubed:**
  *  build errors with actor dependency ([cb164793](cb164793))
  *  verify a process is sufficiently configured before execution ([b13efa07](b13efa07))
  *  list processes from remote workspaces ([84d40d47](84d40d47))
  *  exclude embedding fonts since they are now included in the CSS ([4d6c8a73](4d6c8a73))
  *  resturn a 404 when a segment does not exist ([e144618f](e144618f))
  *  failures that sneaked in during a rebase ([a6c76902](a6c76902))
  *  handle empty responses for remote HTTP requests ([4ad1851c](4ad1851c))
  *  switch http client to allow HTTP redirects and TLS ([905cf778](905cf778))
  *  authenticate remote requests for stats, units and source tags ([073db813](073db813))
  *  account creation and password verification ([552e9781](552e9781))
  *  include remote workspace endpoint in the connection object ([8f0b2244](8f0b2244))
  *  normalized sql include path in config store ([585a80f3](585a80f3))
  *  store database paths expanded ([5ff37660](5ff37660))
  *  wrong parameter count in update workspace sql ([10c3e81b](10c3e81b))
  *  serialization error when creating workspaces ([378656b1](378656b1))
* **ui:**
  *  tag creation multi select renders correctly on firefox ([43fbf92e](43fbf92e))
  *  disable the video player since it breaks the data details view ([f08a84ab](f08a84ab))
  *  remove import that breaks the build ([4f51b704](4f51b704))
  *  remove import that breaks the build ([c8b6e4b1](c8b6e4b1))
  *  avoid flashing of error page when initiating a process run ([1857da2c](1857da2c))
  *  consistent styling of workspace select element ([7c440e83](7c440e83))
  *  use the right css class for url source tags ([6827c0fe](6827c0fe))
  *  allow to cancel linking new workspaces ([7573bcd7](7573bcd7))
  *  handle missing stats state in workspace screen ([8cc00150](8cc00150))
  *  render URL decoded in the data table ([565d2fb4](565d2fb4))
  *  use className as attribute name over class ([37369b2b](37369b2b))
  *  center sidebar navigation buttons ([d0b428d2](d0b428d2))
  *  send the name field when creating local workspaces ([52f2f273](52f2f273))
  *  bad pointer cursor style ([83da621d](83da621d))
  *  fix icon urls in workspace details ([d6ccece5](d6ccece5))
  *  apply base styling for production builds ([2be32823](2be32823))
* **web-ext:**  avoid internal transition to make it work in cosmos ([10abec9b](10abec9b))
* **workspace:**  skip download of Apache Tika ([62b3e38f](62b3e38f))

#### Features

*   display media downloads (rudimentary) ([4e7c8f00](4e7c8f00))
*   show stat for all process sources ([aa16836b](aa16836b))
*   configure process capabilities ([e91ffe12](e91ffe12))
*   scaffold the Ncube discovery browser extension ([ca63593e](ca63593e))
*   extend tables with a search function and enable data search ([d00632d7](d00632d7))
*   build platform binaries and debian packages ([eda010eb](eda010eb))
* **discovery:**
  *  test if Ncube is running ([949d00b7](949d00b7))
  *  cross browser compatibility for Google Chrome ([9cb39834](9cb39834))
  *  reduce the required permission ([c20beb6f](c20beb6f))
  *  only preserve valid URLs ([abe8dc8d](abe8dc8d))
* **ncube:**
  *  log configuration path when starting ([3a8cf4c8](3a8cf4c8))
  *  store application config in platform directories ([c1590dc7](c1590dc7))
  *  new logo, less pointy ([0da32c72](0da32c72))
* **ncubectl:**
  *  migrate databases of workspaces ([58cfcedf](58cfcedf))
  *  export connection details ([4dfc5244](4dfc5244))
  *  get and set configurations settings ([b34d673e](b34d673e))
  *  specify the path to the database ([a2dbac54](a2dbac54))
  *  create/list workspaces and accounts ([452f21e8](452f21e8))
* **ncubed:**
  *  add HTTP endpoint to initiate a process run ([548ddcd0](548ddcd0))
  *  update the workspaces secrets when setting a process config ([d7f30ea9](d7f30ea9))
  *  add HTTP endpoint to list processes and capabilities ([a7fabd98](a7fabd98))
  *  add HTTP endpoint to update a segment ([7a51fdc8](7a51fdc8))
  *  add HTTP endpoint to remove segments ([be070e23](be070e23))
  *  add HTTP endpoint to list segments ([2aadb511](2aadb511))
  *  add HTTP endpoint to show a single segment ([22912934](22912934))
  *  add HTTP endpoint to create segments ([98af24b6](98af24b6))
  *  run data migrations when setting up a workspace ([161497c2](161497c2))
  *  reset a database object in the cache ([d993d073](d993d073))
  *  proxy remote data and sources database requests ([75520583](75520583))
  *  create search indices, extract search into it's own store ([995d7e8d](995d7e8d))
  *  paginated list of sources and expanded stats ([516d46dd](516d46dd))
  *  list units of data with pagination ([47dcb1c5](47dcb1c5))
  *  query stats about the data units ([a84c5233](a84c5233))
  *  finalize the account initializtion workflow ([3020ddc7](3020ddc7))
  *  allow login for http database backends ([5c349f72](5c349f72))
  *  create local account when creating a remote workspace ([41c7e421](41c7e421))
  *  add a config setting for the endpoint ([3cce095d](3cce095d))
  *  add enduser api for HTTP database ([55516825](55516825))
  *  add a HTTP database backend ([9463b01f](9463b01f))
  *  restrict access to endpoints ([faf5bf58](faf5bf58))
  *  read configuration from the command line arguments ([6aa6dba6](6aa6dba6))
  *  create remote workspaces using the HTTP api ([8a552980](8a552980))
  *  verify JWT tokens in HTTP requests ([9d1cf346](9d1cf346))
  *  add support for secret key in setting ([8efed849](8efed849))
  *  login account over HTTP and issue JWT tokens ([bd5ec738](bd5ec738))
  *  update passwords using an HTTP endpoint ([47ad0a3e](47ad0a3e))
  *  handler to verify a password login ([7fd96b57](7fd96b57))
  *  create accounts for a workspace ([da1ce17d](da1ce17d))
  *  run ncubed without an HTTP server ([e3a45a2e](e3a45a2e))
  *  add HTTP endpoint to update sources for a workspace ([1d1fc0a0](1d1fc0a0))
  *  add HTTP endpoint to delete a source for a workspace ([db5a2bba](db5a2bba))
  *  add HTTP endpoint to list sources for a workspace ([87f6a22e](87f6a22e))
  *  add HTTP endpoint to create sources ([7977f55c](7977f55c))
  *  add a database cache actor ([2712dd97](2712dd97))
  *  register workspace databases on the host ([cab9e4e0](cab9e4e0))
  *  wrap HTTP responses in an envelope ([2676161b](2676161b))
  *  add a database cache to reuse connection pools ([6cf0d045](6cf0d045))
  *  package ffmpeg and youtubedl into the workspace ([43bdbe01](43bdbe01))
  *  migrate the workspace sqlite database ([221b1ea7](221b1ea7))
  *  install sugarcube for new workspaces ([2988ec62](2988ec62))
  *  build a workspace archive file including NodeJS ([b654d5ae](b654d5ae))
  *  create workspace from zipped template ([7abe8d68](7abe8d68))
  *  build the template workspace as a zip file ([372fc1d0](372fc1d0))
  *  add workspace HTTP endpoints ([5681926c](5681926c))
* **ui:**
  *  select source type from a drop down menu ([1e184cc4](1e184cc4))
  *  show media in form of a carousel ([c76fd4d9](c76fd4d9))
  *  show data unit details ([7f55d7e4](7f55d7e4))
  *  replace data table columns and truncate cell text ([73321e6d](73321e6d))
  *  run processes using the UI ([d3f9d271](d3f9d271))
  *  Render details of sources ([1fe76e96](1fe76e96))
  *  list all available processes ([06cc7944](06cc7944))
  *  add process card component ([836cf5f1](836cf5f1))
  *  create and update segments ([c376712d](c376712d))
  *  list segments for a database and explore them ([2db2ac1b](2db2ac1b))
  *  add loading spinner while loading stats and pages ([fd451e91](fd451e91))
  *  fetch segments when opening the data section ([7a998b5e](7a998b5e))
  *  add expandable button component ([e8024a90](e8024a90))
  *  refactor source tag ui and use a multi select widget ([801f176a](801f176a))
  *  paginated source and data tables ([11639c33](11639c33))
  *  render data stats ([2c67a56f](2c67a56f))
  *  query stats about sources ([836fb64d](836fb64d))
  *  list and create tags for sources ([745ecf7e](745ecf7e))
  *  data table with selection, filters, pagination and delete modal ([9fad65c1](9fad65c1))
  *  upload connection files to link remote workspaces ([6209d3da](6209d3da))
  *  select sources ([92a21575](92a21575))
  *  create sources ([2549bab4](2549bab4))
  *  list and delete sources for a workspace ([720f4e06](720f4e06))
  *  add cancel button to the create workspace form ([d516c359](d516c359))
  *  render the workspace details screen ([d21799c2](d21799c2))
  *  add workspace part icons ([0e0d2f87](0e0d2f87))
  *  create local workspaces ([c8936a09](c8936a09))
  *  list all workspaces ([74f42a8c](74f42a8c))
* **web-ext:**  add browser extension for source discovery and send them to Ncube ([6543e053](6543e053))
* **workspace:**
  *  upgrade sugarcube dependency to 0.41.0 ([3ebdac9a](3ebdac9a))
  *  added twitter data processes and fetch images, videos and screenshots ([0f42b0ec](0f42b0ec))
  *  add preliminary youtube video and channel processes ([f59ae596](f59ae596))



<a name="0.6.3"></a>
## 0.6.3 (2020-07-20)




<a name="0.6.2"></a>
## 0.6.2 (2020-07-20)




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



