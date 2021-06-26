<a name="4.4.0"></a>
## 4.4.0 (2021-04-24)


#### Bug Fixes

*   Fix spelling error in main.rs ([91ee27f2](https://github.com/rust-lang/rustlings/commit/91ee27f22bd3797a9db57e5fd430801c170c5db8))
*   typo in default out text ([644c49f1](https://github.com/rust-lang/rustlings/commit/644c49f1e04cbb24e95872b3a52b07d692ae3bc8))
* **collections:**  Naming exercises for vectors and hashmap ([bef39b12](https://github.com/rust-lang/rustlings/commit/bef39b125961310b34b34871e480a82e82af4678))
* **from_str:**
  *  Correct typos ([5f7c89f8](https://github.com/rust-lang/rustlings/commit/5f7c89f85db1f33da01911eaa479c3a2d4721678))
  *  test for error instead of unwrap/should_panic ([15e71535](https://github.com/rust-lang/rustlings/commit/15e71535f37cfaed36e22eb778728d186e2104ab))
  *   use trait objects for from_str ([c3e7b831](https://github.com/rust-lang/rustlings/commit/c3e7b831786c9172ed8bd5d150f3c432f242fba9))
* **functions3:**  improve function argument type (#687) ([a6509cc4](https://github.com/rust-lang/rustlings/commit/a6509cc4d545d8825f01ddf7ee37823b372154dd))
* **hashmap2:**  Update incorrect assertion (#660) ([72aaa15e](https://github.com/rust-lang/rustlings/commit/72aaa15e6ab4b72b3422f1c6356396e20a2a2bb8))
* **info:**  Fix typo (#635) ([cddc1e86](https://github.com/rust-lang/rustlings/commit/cddc1e86e7ec744ee644cc774a4887b1a0ded3e8))
* **iterators2:**  Moved errors out of tests. ([baf4ba17](https://github.com/rust-lang/rustlings/commit/baf4ba175ba6eb92989e3dd54ecbec4bedc9a863), closes [#359](https://github.com/rust-lang/rustlings/issues/359))
* **iterators3:**  Enabled iterators3.rs to run without commented out tests. ([c6712dfc](https://github.com/rust-lang/rustlings/commit/c6712dfccd1a093e590ad22bbc4f49edc417dac0))
* **main:**  Let find_exercise work with borrows ([347f30bd](https://github.com/rust-lang/rustlings/commit/347f30bd867343c5ace1097e085a1f7e356553f7))
* **move_semantics4:**
  *  Remove redundant "instead" (#640) ([cc266d7d](https://github.com/rust-lang/rustlings/commit/cc266d7d80b91e79df3f61984f231b7f1587218e))
  *  Small readbility improvement (#617) ([10965920](https://github.com/rust-lang/rustlings/commit/10965920fbdf8a1efc85bed869e55a1787006404))
* **option2:**  Rename uninformative variables (#675) ([b4de6594](https://github.com/rust-lang/rustlings/commit/b4de6594380636817d13c2677ec6f472a964cf43))
* **quiz3:**  Force an answer to Q2 (#672) ([0d894e6f](https://github.com/rust-lang/rustlings/commit/0d894e6ff739943901e1ae8c904582e5c2f843bd))
* **structs:**  Add 5.3 to structs/README (#652) ([6bd791f2](https://github.com/rust-lang/rustlings/commit/6bd791f2f44aa7f0ad926df767f6b1fa8f12a9a9))
* **structs2:**  correct grammar in hint (#663) ([ebdb66c7](https://github.com/rust-lang/rustlings/commit/ebdb66c7bfb6d687a14cc511a559a222e6fc5de4))
* **structs3:**
  * reword heading comment (#664) ([9f3e8c2d](https://github.com/rust-lang/rustlings/commit/9f3e8c2dde645e5264c2d2200e68842b5f47bfa3))
  *   add check to prevent naive implementation of is_international ([05a753fe](https://github.com/rust-lang/rustlings/commit/05a753fe6333d36dbee5f68c21dec04eacdc75df))
* **threads1:**  line number correction ([7857b0a6](https://github.com/rust-lang/rustlings/commit/7857b0a689b0847f48d8c14cbd1865e3b812d5ca))
* **try_from_into:**  use trait objects ([2e93a588](https://github.com/rust-lang/rustlings/commit/2e93a588e0abe8badb7eafafb9e7d073c2be5df8))

#### Features

*   Replace clap with argh ([7928122f](https://github.com/rust-lang/rustlings/commit/7928122fcef9ca7834d988b1ec8ca0687478beeb))
*   Replace emojis when NO_EMOJI env variable present ([8d62a996](https://github.com/rust-lang/rustlings/commit/8d62a9963708dbecd9312e8bcc4b47049c72d155))
*   Added iterators5.rs exercise. ([b29ea17e](https://github.com/rust-lang/rustlings/commit/b29ea17ea94d1862114af2cf5ced0e09c197dc35))
* **arc1:**  Add more details to description and hint (#710) ([81be4044](https://github.com/rust-lang/rustlings/commit/81be40448777fa338ebced3b0bfc1b32d6370313))
* **cli:**  Improve the list command with options, and then some ([8bbe4ff1](https://github.com/rust-lang/rustlings/commit/8bbe4ff1385c5c169c90cd3ff9253f9a91daaf8e))
* **list:**
  *  updated progress percentage ([1c6f7e4b](https://github.com/rust-lang/rustlings/commit/1c6f7e4b7b9b3bd36f4da2bb2b69c549cc8bd913))
  *  added progress info ([c0e3daac](https://github.com/rust-lang/rustlings/commit/c0e3daacaf6850811df5bc57fa43e0f249d5cfa4))



<a name="4.3.0"></a>
## 4.3.0 (2020-12-29)

#### Features

*   Rewrite default out text ([44d39112](https://github.com/rust-lang/rustlings/commit/44d39112ff122b29c9793fe52e605df1612c6490))
*   match exercise order to book chapters (#541) ([033bf119](https://github.com/rust-lang/rustlings/commit/033bf1198fc8bfce1b570e49da7cde010aa552e3))
*   Crab? (#586) ([fa9f522b](https://github.com/rust-lang/rustlings/commit/fa9f522b7f043d7ef73a39f003a9272dfe72c4f4))
*   add "rustlings list" command ([838f9f30](https://github.com/rust-lang/rustlings/commit/838f9f30083d0b23fd67503dcf0fbeca498e6647))
* **try_from_into:**  remove duplicate annotation ([04f1d079](https://github.com/rust-lang/rustlings/commit/04f1d079aa42a2f49af694bc92c67d731d31a53f))

#### Bug Fixes

*   update structs README ([bcf14cf6](https://github.com/rust-lang/rustlings/commit/bcf14cf677adb3a38a3ac3ca53f3c69f61153025))
*   added missing exercises to info.toml ([90cfb6ff](https://github.com/rust-lang/rustlings/commit/90cfb6ff28377531bfc34acb70547bdb13374f6b))
*   gives a bit more context to magic number ([30644c9a](https://github.com/rust-lang/rustlings/commit/30644c9a062b825c0ea89435dc59f0cad86b110e))
* **functions2:**  Change signature to trigger precise error message: (#605) ([0ef95947](https://github.com/rust-lang/rustlings/commit/0ef95947cc30482e63a7045be6cc2fb6f6dcb4cc))
* **structs1:**  Adjust wording (#573) ([9334783d](https://github.com/rust-lang/rustlings/commit/9334783da31d821cc59174fbe8320df95828926c))
* **try_from_into:**
  *  type error ([4f4cfcf3](https://github.com/rust-lang/rustlings/commit/4f4cfcf3c36c8718c7c170c9c3a6935e6ef0618c))
  *  Update description (#584) ([96347df9](https://github.com/rust-lang/rustlings/commit/96347df9df294f01153b29d9ad4ba361f665c755))
* **vec1:**  Have test compare every element in a and v ([9b6c6293](https://github.com/rust-lang/rustlings/commit/9b6c629397b24b944f484f5b2bbd8144266b5695))

<a name="4.2.0"></a>
## 4.2.0 (2020-11-07)

#### Features

*   Add HashMap exercises ([633c00cf](https://github.com/rust-lang/rustlings/commit/633c00cf8071e1e82959a3010452a32f34f29fc9))
*   Add Vec exercises ([0c12fa31](https://github.com/rust-lang/rustlings/commit/0c12fa31c57c03c6287458a0a8aca7afd057baf6))
* **primitive_types6:**  Add a test (#548) ([2b1fb2b7](https://github.com/rust-lang/rustlings/commit/2b1fb2b739bf9ad8d6b7b12af25fee173011bfc4))
* **try_from_into:**  Add tests (#571) ([95ccd926](https://github.com/rust-lang/rustlings/commit/95ccd92616ae79ba287cce221101e0bbe4f68cdc))

#### Bug Fixes

*   log error output when inotify limit is exceeded ([d61b4e5a](https://github.com/rust-lang/rustlings/commit/d61b4e5a13b44d72d004082f523fa1b6b24c1aca))
*   more unique temp_file ([5643ef05](https://github.com/rust-lang/rustlings/commit/5643ef05bc81e4a840e9456f4406a769abbe1392))
* **installation:**  Update the MinRustVersion ([21bfb2d4](https://github.com/rust-lang/rustlings/commit/21bfb2d4777429c87d8d3b5fbf0ce66006dcd034))
* **iterators2:**  Update description (#578) ([197d3a3d](https://github.com/rust-lang/rustlings/commit/197d3a3d8961b2465579218a6749b2b2cefa8ddd))
* **primitive_types6:**
  *  remove 'unused doc comment' warning ([472d8592](https://github.com/rust-lang/rustlings/commit/472d8592d65c8275332a20dfc269e7ac0d41bc88))
  *  missing comma in test ([4fb230da](https://github.com/rust-lang/rustlings/commit/4fb230daf1251444fcf29e085cee222a91f8a37e))
* **quiz3:**  Second test is for odd numbers, not even. (#553) ([18e0bfef](https://github.com/rust-lang/rustlings/commit/18e0bfef1de53071e353ba1ec5837002ff7290e6))

<a name="4.1.0"></a>
## 4.1.0 (2020-10-05)

#### Bug Fixes

*   Update rustlings version in Cargo.lock ([1cc40bc9](https://github.com/rust-lang/rustlings/commit/1cc40bc9ce95c23d56f6d91fa1c4deb646231fef))
* **arc1:**  index mod should equal thread count ([b4062ef6](https://github.com/rust-lang/rustlings/commit/b4062ef6993e80dac107c4093ea85166ad3ee0fa))
* **enums3:**  Update Message::ChangeColor to take a tuple. (#457) ([4b6540c7](https://github.com/rust-lang/rustlings/commit/4b6540c71adabad647de8a09e57295e7c7c7d794))
* **exercises:**  adding question mark to quiz2 ([101072ab](https://github.com/rust-lang/rustlings/commit/101072ab9f8c80b40b8b88cb06cbe38aca2481c5))
* **generics3:**  clarify grade change ([47f7672c](https://github.com/rust-lang/rustlings/commit/47f7672c0307732056e7426e81d351f0dd7e22e5))
* **structs3:**  Small adjustment of variable name ([114b54cb](https://github.com/rust-lang/rustlings/commit/114b54cbdb977234b39e5f180d937c14c78bb8b2))
* **using_as:**  Add test so that proper type is returned. (#512) ([3286c5ec](https://github.com/rust-lang/rustlings/commit/3286c5ec19ea5fb7ded81d047da5f8594108a490))

#### Features

*   Added iterators1.rs exercise ([9642f5a3](https://github.com/rust-lang/rustlings/commit/9642f5a3f686270a4f8f6ba969919ddbbc4f7fdd))
*   Add ability to run rustlings on repl.it (#471) ([8f7b5bd0](https://github.com/rust-lang/rustlings/commit/8f7b5bd00eb83542b959830ef55192d2d76db90a))
*   Add gitpod support (#473) ([4821a8be](https://github.com/rust-lang/rustlings/commit/4821a8be94af4f669042a06ab917934cfacd032f))
*   Remind the user of the hint option (#425) ([816b1f5e](https://github.com/rust-lang/rustlings/commit/816b1f5e85d6cc6e72673813a85d0ada2a8f84af))
*   Remind the user of the hint option (#425) ([9f61db5d](https://github.com/rust-lang/rustlings/commit/9f61db5dbe38538cf06571fcdd5f806e7901e83a))
* **cli:**  Added 'cls' command to 'watch' mode (#474) ([4f2468e1](https://github.com/rust-lang/rustlings/commit/4f2468e14f574a93a2e9b688367b5752ed96ae7b))
* **try_from_into:**  Add insufficient length test (#469) ([523d18b8](https://github.com/rust-lang/rustlings/commit/523d18b873a319f7c09262f44bd40e2fab1830e5))

<a name="4.0.0"></a>
## 4.0.0 (2020-07-08)

#### Breaking Changes

*   Add a --nocapture option to display test harnesses' outputs ([8ad5f9bf](https://github.com/rust-lang/rustlings/commit/8ad5f9bf531a4848b1104b7b389a20171624c82f))
*   Rename test to quiz, fixes #244 ([010a0456](https://github.com/rust-lang/rustlings/commit/010a04569282149cea7f7a76fc4d7f4c9f0f08dd))

#### Features

*   Add traits README ([173bb141](https://github.com/rust-lang/rustlings/commit/173bb14140c5530cbdb59e53ace3991a99d804af))
*   Add box1.rs exercise ([7479a473](https://github.com/rust-lang/rustlings/commit/7479a4737bdcac347322ad0883ca528c8675e720))
*   Rewrite try_from_into (#393) ([763aa6e3](https://github.com/rust-lang/rustlings/commit/763aa6e378a586caae2d8d63755a85eeba227933))
*   Add if2 exercise ([1da84b5f](https://github.com/rust-lang/rustlings/commit/1da84b5f7c489f65bd683c244f13c7d1ee812df0))
*   Added exercise structs3.rs ([b66e2e09](https://github.com/rust-lang/rustlings/commit/b66e2e09622243e086a0f1258dd27e1a2d61c891))
*   Add exercise variables6 covering const (#352) ([5999acd2](https://github.com/rust-lang/rustlings/commit/5999acd24a4f203292be36e0fd18d385887ec481))

#### Bug Fixes

*   Change then to than ([ddd98ad7](https://github.com/rust-lang/rustlings/commit/ddd98ad75d3668fbb10eff74374148aa5ed2344d))
*   rename quiz1 to tests1 in info (#420) ([0dd1c6ca](https://github.com/rust-lang/rustlings/commit/0dd1c6ca6b389789e0972aa955fe17aa15c95f29))
*   fix quiz naming inconsistency (#421) ([5563adbb](https://github.com/rust-lang/rustlings/commit/5563adbb890587fc48fbbc9c4028642687f1e85b))
*   confine the user further in variable exercises ([06ef4cc6](https://github.com/rust-lang/rustlings/commit/06ef4cc654e75d22a526812919ee49b8956280bf))
*   update iterator and macro text for typos and clarity ([95900828](https://github.com/rust-lang/rustlings/commit/959008284834bece0196a01e17ac69a7e3590116))
*   update generics2       closes #362 ([964c974a](https://github.com/rust-lang/rustlings/commit/964c974a0274199d755073b917c2bc5da0c9b4f1))
*   confusing comment in conversions/try_from_into.rs ([c9e4f2cf](https://github.com/rust-lang/rustlings/commit/c9e4f2cfb4c48d0b7451263cfb43b9426438122d))
* **arc1:**  Passively introduce attributes (#429) ([113cdae2](https://github.com/rust-lang/rustlings/commit/113cdae2d4e4c55905e8056ad326ede7fd7de356))
* **box1:**  fix comment typo (#426) ([bb2ca251](https://github.com/rust-lang/rustlings/commit/bb2ca251106b27a7272d9a30872904dd1376654c))
* **errorsn:**  Try harder to confine the user. (#388) ([2b20c8a0](https://github.com/rust-lang/rustlings/commit/2b20c8a0f5774d07c58d110d75879f33fc6273b5))
* **from_into.rs:**  typo ([a901499e](https://github.com/rust-lang/rustlings/commit/a901499ededd3ce1995164700514fe4e9a0373ea))
* **generics2:**  Guide students to the answer (#430) ([e6bd8021](https://github.com/rust-lang/rustlings/commit/e6bd8021d9a7dd06feebc30c9d5f953901d7b419))
* **installation:**
  *  Provide a backup git reference when tag can't be curl ([9e4fb100](https://github.com/rust-lang/rustlings/commit/9e4fb1009f1c9e3433915c03e22c2af422e5c5fe))
  *  Check if python is available while checking for git,rustc and cargo ([9cfb617d](https://github.com/rust-lang/rustlings/commit/9cfb617d5b0451b4b51644a1298965390cda9884))
* **option1:**
  *  Don't add only zeros to the numbers array ([cce6a442](https://github.com/rust-lang/rustlings/commit/cce6a4427718724a9096800754cd3abeca6a1580))
  *  Add cast to usize, as it is confusing in the context of an exercise about Option ([f6cffc7e](https://github.com/rust-lang/rustlings/commit/f6cffc7e487b42f15a6f958e49704c93a8d4465b))
* **option2:**  Add TODO to comments (#400) ([10967bce](https://github.com/rust-lang/rustlings/commit/10967bce57682812dc0891a9f9757da1a9d87404))
* **options1:**  Add hint about Array Initialization (#389) ([9f75554f](https://github.com/rust-lang/rustlings/commit/9f75554f2a30295996f03f0160b98c0458305502))
* **test2:**  name of type String and &str (#394) ([d6c0a688](https://github.com/rust-lang/rustlings/commit/d6c0a688e6a96f93ad60d540d4b326f342fc0d45))
* **variables6:**  minor typo (#419) ([524e17df](https://github.com/rust-lang/rustlings/commit/524e17df10db95f7b90a0f75cc8997182a8a4094))

<a name="3.0.0"></a>
## 3.0.0 (2020-04-11)

#### Breaking Changes

* make "compile" exercises print output (#278) ([3b6d5c](https://github.com/fmoko/rustlings/commit/3b6d5c3aaa27a242a832799eb66e96897d26fde3))

#### Bug Fixes

* **primitive_types:** revert primitive_types4 (#296) ([b3a3351e](https://github.com/rust-lang/rustlings/commit/b3a3351e8e6a0bdee07077d7b0382953821649ae))
* **run:**  compile clippy exercise files (#295) ([3ab084a4](https://github.com/rust-lang/rustlings/commit/3ab084a421c0f140ae83bf1fc3f47b39342e7373))
* **conversions:**
  * add additional test to meet exercise rules (#284) ([bc22ec3](https://github.com/fmoko/rustlings/commit/bc22ec382f843347333ef1301fc1bad773657f38))
  * remove duplicate not done comment (#292) ([dab90f](https://github.com/fmoko/rustlings/commit/dab90f7b91a6000fe874e3d664f244048e5fa342))
* don't hardcode documentation version for traits (#288) ([30e6af](https://github.com/fmoko/rustlings/commit/30e6af60690c326fb5d3a9b7335f35c69c09137d))

#### Features

*   add Option2 exercise (#290) ([86b5c08b](https://github.com/rust-lang/rustlings/commit/86b5c08b9bea1576127a7c5f599f5752072c087d))
*   add excercise for option (#282) ([135e5d47](https://github.com/rust-lang/rustlings/commit/135e5d47a7c395aece6f6022117fb20c82f2d3d4))
*   add new exercises for generics (#280) ([76be5e4e](https://github.com/rust-lang/rustlings/commit/76be5e4e991160f5fd9093f03ee2ba260e8f7229))
* **ci:**  add buildkite config ([b049fa2c](https://github.com/rust-lang/rustlings/commit/b049fa2c84dba0f0c8906ac44e28fd45fba51a71))

<a name="2.2.1"></a>
### 2.2.1 (2020-02-27)

#### Bug Fixes

*   Re-add cloning the repo to install scripts ([3d9b03c5](https://github.com/rust-lang/rustlings/commit/3d9b03c52b8dc51b140757f6fd25ad87b5782ef5))

#### Features

*   Add clippy lints (#269) ([1e2fd9c9](https://github.com/rust-lang/rustlings/commit/1e2fd9c92f8cd6e389525ca1a999fca4c90b5921))

<a name="2.2.0"></a>
## 2.2.0 (2020-02-25)


#### Bug Fixes

*   Update deps to version compatable with aarch64-pc-windows (#263) ([19a93428](https://github.com/rust-lang/rustlings/commit/19a93428b3c73d994292671f829bdc8e5b7b3401))
* **docs:**
  * Added a necessary step to Windows installation process (#242) ([3906efcd](https://github.com/rust-lang/rustlings/commit/3906efcd52a004047b460ed548037093de3f523f))
  * Fixed mangled sentence from book; edited for clarity (#266) ([ade52ff](https://github.com/rust-lang/rustlings/commit/ade52ffb739987287ddd5705944c8777705faed9))
  * Updated iterators readme to account for iterators4 exercise (#273) ([bec8e3a](https://github.com/rust-lang/rustlings/commit/bec8e3a644cbd88db1c73ea5f1d8a364f4a34016))
* **installation:**  make fatal errors more obvious (#272) ([17d0951e](https://github.com/rust-lang/rustlings/commit/17d0951e66fda8e11b204d5c4c41a0d5e22e78f7))
* **iterators2:**
  *  Remove reference to missing iterators2.rs (#245) ([419f7797](https://github.com/rust-lang/rustlings/commit/419f7797f294e4ce6a2b883199731b5bde77d262))
* **as_ref_mut:** Enable a test and improve per clippy's suggestion (#256) ([dfdf809](https://github.com/rust-lang/rustlings/commit/dfdf8093ebbd4145864995627b812780de52f902))
* **tests1:**
  * Change test command ([fe10e06c](https://github.com/rust-lang/rustlings/commit/fe10e06c3733ddb4a21e90d09bf79bfe618e97ce)
  * Correct test command in tests1.rs comment (#263) ([39fa7ae](https://github.com/rust-lang/rustlings/commit/39fa7ae8b70ad468da49b06f11b2383135a63bcf))

#### Features

*   Add variables5.rs exercise (#264) ([0c73609e](https://github.com/rust-lang/rustlings/commit/0c73609e6f2311295e95d6f96f8c747cfc4cba03))
*   Show a completion message when watching (#253) ([d25ee55a](https://github.com/rust-lang/rustlings/commit/d25ee55a3205882d35782e370af855051b39c58c))
*   Add type conversion and parsing exercises (#249) ([0c85dc11](https://github.com/rust-lang/rustlings/commit/0c85dc1193978b5165491b99cc4922caf8d14a65))
*   Created consistent money unit (#258) ([fd57f8f](https://github.com/rust-lang/rustlings/commit/fd57f8f2c1da2af8ddbebbccec214e6f40f4dbab))
*   Enable test for exercise test4 (#276) ([8b971ff](https://github.com/rust-lang/rustlings/commit/8b971ffab6079a706ac925f5917f987932b55c07))
*   Added traits exercises (#274 but specifically #216, which originally added
    this :heart:) ([b559cdd](https://github.com/rust-lang/rustlings/commit/b559cdd73f32c0d0cfc1feda39f82b3e3583df17))


<a name="2.1.0"></a>
## 2.1.0 (2019-11-27)

#### Bug Fixes

* add line numbers in several exercises and hints ([b565c4d3](https://github.com/rust-lang/rustlings/commit/b565c4d3e74e8e110bef201a082fa1302722a7c3))
* **arc1:**  Fix some words in the comment ([c42c3b21](https://github.com/rust-lang/rustlings/commit/c42c3b2101df9164c8cd7bb344def921e5ba3e61))
* **enums:**  Add link to chapter on pattern syntax (#242) ([615ce327](https://github.com/rust-lang/rustlings/commit/615ce3279800c56d89f19d218ccb7ef576624feb))
* **primitive_types4:**
  *  update outdated hint ([4c5189df](https://github.com/rust-lang/rustlings/commit/4c5189df2bdd9a231f6b2611919ba5aa14da0d3f))
  *  update outdated comment ([ded2c034](https://github.com/rust-lang/rustlings/commit/ded2c034ba93fa1e3c2c2ea16b83abc1a57265e8))
* **strings2:**  update line number in hint ([a09f684f](https://github.com/rust-lang/rustlings/commit/a09f684f05c58d239a6fc59ec5f81c2533e8b820))
* **variables1:**  Correct wrong word in comment ([fda5a470](https://github.com/rust-lang/rustlings/commit/fda5a47069e0954f16a04e8e50945e03becb71a5))

#### Features

* **watch:**  show hint while watching ([8143d57b](https://github.com/rust-lang/rustlings/commit/8143d57b4e88c51341dd4a18a14c536042cc009c))

<a name="2.0.0"></a>
## 2.0.0 (2019-11-12)

#### Bug Fixes

* **default:**  Clarify the installation procedure ([c371b853](https://github.com/rust-lang/rustlings/commit/c371b853afa08947ddeebec0edd074b171eeaae0))
* **info:**  Fix trailing newlines for hints ([795b6e34](https://github.com/rust-lang/rustlings/commit/795b6e348094a898e9227a14f6232f7bb94c8d31))
* **run:**  make `run` never prompt ([4b265465](https://github.com/rust-lang/rustlings/commit/4b26546589f7d2b50455429482cf1f386ceae8b3))

#### Breaking Changes

*   Refactor hint system ([9bdb0a12](https://github.com/rust-lang/rustlings/commit/9bdb0a12e45a8e9f9f6a4bd4a9c172c5376c7f60))
*   improve `watch` execution mode ([2cdd6129](https://github.com/rust-lang/rustlings/commit/2cdd61294f0d9a53775ee24ad76435bec8a21e60))
*   Index exercises by name ([627cdc07](https://github.com/rust-lang/rustlings/commit/627cdc07d07dfe6a740e885e0ddf6900e7ec336b))
* **run:**  makes `run` never prompt ([4b265465](https://github.com/rust-lang/rustlings/commit/4b26546589f7d2b50455429482cf1f386ceae8b3))

#### Features

* **cli:**  check for rustc before doing anything ([36a033b8](https://github.com/rust-lang/rustlings/commit/36a033b87a6549c1e5639c908bf7381c84f4f425))
* **hint:**  Add test for hint ([ce9fa6eb](https://github.com/rust-lang/rustlings/commit/ce9fa6ebbfdc3e7585d488d9409797285708316f))

<a name="1.5.1"></a>
### 1.5.1 (2019-11-11)

#### Bug Fixes

* **errors3:**  Update hint ([dcfb427b](https://github.com/rust-lang/rustlings/commit/dcfb427b09585f0193f0a294443fdf99f11c64cb), closes [#185](https://github.com/rust-lang/rustlings/issues/185))
* **if1:**  Remove `return` reference ([ad03d180](https://github.com/rust-lang/rustlings/commit/ad03d180c9311c0093e56a3531eec1a9a70cdb45))
* **strings:**  Move Strings before Structs ([6dcecb38](https://github.com/rust-lang/rustlings/commit/6dcecb38a4435593beb87c8e12d6314143631482), closes [#204](https://github.com/rust-lang/rustlings/issues/204))
* **structs1:**  Remove misleading comment ([f72e5a8f](https://github.com/rust-lang/rustlings/commit/f72e5a8f05568dde04eaeac10b9a69872f21cb37))
* **threads:**  Move Threads behind SLT ([fbe91a67](https://github.com/rust-lang/rustlings/commit/fbe91a67a482bfe64cbcdd58d06ba830a0f39da3), closes [#205](https://github.com/rust-lang/rustlings/issues/205))
* **watch:** clear screen before each `verify()`  ([3aff590](https://github.com/rust-lang/rustlings/commit/3aff59085586c24196a547c2693adbdcf4432648))

<a name="1.5.0"></a>
## 1.5.0 (2019-11-09)

#### Bug Fixes

* **test1:** Rewrite logic ([79a56942](https://github.com/rust-lang/rustlings/commit/79a569422c8309cfc9e4aed25bf4ab3b3859996b))
* **installation:**  Fix rustlings installation check ([7a252c47](https://github.com/rust-lang/rustlings/commit/7a252c475551486efb52f949b8af55803b700bc6))
* **iterators:**  Rename iterator3.rs ([433d2115](https://github.com/rust-lang/rustlings/commit/433d2115bc1c04b6d34a335a18c9a8f3e2672bc6))
* **iterators2:**  Remove syntax resulting in misleading error message ([4cde8664](https://github.com/rust-lang/rustlings/commit/4cde86643e12db162a66e62f23b78962986046ac))
* **option1:**
  *  Fix arguments passed to assert! macro (#222) ([4c2cf6da](https://github.com/rust-lang/rustlings/commit/4c2cf6da755efe02725e05ecc3a303304c10a6da))
  *  Fix arguments passed to assert! macro ([ead4f7af](https://github.com/rust-lang/rustlings/commit/ead4f7af9e10e53418efdde5c359159347282afd))
  *  Add test for prematurely passing exercise ([a750e4a1](https://github.com/rust-lang/rustlings/commit/a750e4a1a3006227292bb17d57d78ce84da6bfc6))
* **primitive_types4:**  Fail on a slice covering the wrong area ([5b1e673c](https://github.com/rust-lang/rustlings/commit/5b1e673cec1658afc4ebbbc800213847804facf5))
* **readme:**  http to https ([70946b85](https://github.com/rust-lang/rustlings/commit/70946b85e536e80e70ed9505cb650ca0a3a1fbb5))
* **test1:**
  *  Swap assertion parameter order ([4086d463](https://github.com/rust-lang/rustlings/commit/4086d463a981e81d97781851d17db2ced290f446))
  *  renamed function name to snake case closes #180 ([89d5186c](https://github.com/rust-lang/rustlings/commit/89d5186c0dae8135ecabf90ee8bb35949bc2d29b))

#### Features

*   Add enums exercises ([dc150321](https://github.com/rust-lang/rustlings/commit/dc15032112fc485226a573a18139e5ce928b1755))
*   Added exercise for struct update syntax ([1c4c8764](https://github.com/rust-lang/rustlings/commit/1c4c8764ed118740cd4cee73272ddc6cceb9d959))
* **iterators2:**  adds iterators2 exercise including config ([9288fccf](https://github.com/rust-lang/rustlings/commit/9288fccf07a2c5043b76d0fd6491e4cf72d76031))

<a name="1.4.1"></a>
### 1.4.1 (2019-08-13)


#### Bug Fixes

* **iterators2:**  Remove syntax resulting in misleading error message ([4cde8664](https://github.com/rust-lang/rustlings/commit/4cde86643e12db162a66e62f23b78962986046ac))
* **option1:**  Add test for prematurely passing exercise ([a750e4a1](https://github.com/rust-lang/rustlings/commit/a750e4a1a3006227292bb17d57d78ce84da6bfc6))
* **test1:**  Swap assertion parameter order ([4086d463](https://github.com/rust-lang/rustlings/commit/4086d463a981e81d97781851d17db2ced290f446))



<a name="1.4.0"></a>
## 1.4.0 (2019-07-13)

#### Bug Fixes

* **installation:**  Fix rustlings installation check ([7a252c47](https://github.com/rust-lang/rustlings/commit/7a252c475551486efb52f949b8af55803b700bc6))
* **iterators:**  Rename iterator3.rs ([433d2115](https://github.com/rust-lang/rustlings/commit/433d2115bc1c04b6d34a335a18c9a8f3e2672bc6))
* **readme:**  http to https ([70946b85](https://github.com/rust-lang/rustlings/commit/70946b85e536e80e70ed9505cb650ca0a3a1fbb5))
* **test1:**  renamed function name to snake case ([89d5186c](https://github.com/rust-lang/rustlings/commit/89d5186c0dae8135ecabf90ee8bb35949bc2d29b))
* **cli:** Check if changed exercise file exists before calling verify ([ba85ca3](https://github.com/rust-lang/rustlings/commit/ba85ca32c4cfc61de46851ab89f9c58a28f33c88))
* **structs1:** Fix the irrefutable let pattern warning ([cc6a141](https://github.com/rust-lang/rustlings/commit/cc6a14104d7c034eadc98297eaaa972d09c50b1f))

#### Features

* **changelog:**  Use clog for changelogs ([34e31232](https://github.com/rust-lang/rustlings/commit/34e31232dfddde284a341c9609b33cd27d9d5724))
* **iterators2:**  adds iterators2 exercise including config ([9288fccf](https://github.com/rust-lang/rustlings/commit/9288fccf07a2c5043b76d0fd6491e4cf72d76031))

<a name="1.3.0"></a>
### 1.3.0 (2019-06-05)

#### Features

- Adds a simple exercise for structures (#163, @briankung)

#### Bug Fixes

- Add Result type signature as it is difficult for new comers to understand Generics and Error all at once. (#157, @veggiemonk)
- Rustfmt and whitespace fixes (#161, @eddyp)
- errorsn.rs: Separate also the hints from each other to avoid accidental viewing (#162, @eddyp)
- fixed outdated links (#165, @gushroom)
- Fix broken link (#164, @HanKruiger)
- Remove highlighting and syntect (#167, @komaeda)

<a name="1.2.2"></a>
### 1.2.2 (2019-05-07)

#### Bug Fixes

- Reverted `--nocapture` flag since it was causing tests to pass unconditionally

<a name="1.2.1"></a>
### 1.2.1 (2019-04-22)

#### Bug Fixes

- Fix the `--nocapture` feature (@komaeda)
- Provide a nicer error message for when you're in the wrong directory

<a name="1.2.0"></a>
### 1.2.0 (2019-04-22)

#### Features

- Add errors to exercises that compile without user changes (@yvan-sraka)
- Use --nocapture when testing, enabling `println!` when running (@komaeda)

<a name="1.1.1"></a>
### 1.1.1 (2019-04-14)

#### Bug fixes

- Fix permissions on exercise files (@zacanger, #133)
- Make installation checks more thorough (@komaeda, 1b3469f236bc6979c27f6e1a04e4138a88e55de3)
- Fix order of true/false in tests for executables (@mgeier, #137)
- Stop run from panicking when compile fails (@cjpearce, #141)
- Fix intermittent test failure caused by race condition (@cjpearce, #140)
- Fix links by deleting book version (@diodfr, #142)
- Canonicalize paths to fix path matching (@cjpearce, #143)

<a name="1.1.0"></a>
### 1.1.0 (2019-03-20)

- errors2.rs: update link to Rust book (#124)
- Start verification at most recently modified file (#120)
- Watch for file creation events in watch mode (#117)
- Add standard library types to exercises suite (#119)
- Give a warning when Rustlings isn't run from the right directory (#123)
- Verify that rust version is recent enough to install Rustlings (#131)

<a name="1.0.1"></a>
### 1.0.1 (2019-03-06)

- Adds a way to install Rustlings in one command (`curl -L https://git.io/rustlings | bash`)
- Makes `rustlings watch` react to create file events (@shaunbennett, #117)
- Reworks the exercise management to use an external TOML file instead of just listing them in the code

<a name="1.0.0"></a>
### 1.0.0 (2019-03-06)

Initial release.
