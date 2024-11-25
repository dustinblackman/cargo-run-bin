# Changelog

All notable changes to this project will be documented in this file.

## [1.7.4] - 2024-11-25
[v1.7.3..v1.7.4](https://github.com/dustinblackman/cargo-run-bin/compare/v1.7.3..v1.7.4)


### 🐛 Bug Fixes

- Handle plain strings in --sync-aliases ([#25](https://github.com/dustinblackman/cargo-run-bin/issues/25)) - ([3c4d168](https://github.com/dustinblackman/cargo-run-bin/commit/3c4d168f34bf19e2da1099b6351f00b0ced2383c))

## [1.7.3] - 2024-07-05
[v1.7.2..v1.7.3](https://github.com/dustinblackman/cargo-run-bin/compare/v1.7.2..v1.7.3)


### ⛰️  Features

- Release v1.7.3 - ([c6bd9ec](https://github.com/dustinblackman/cargo-run-bin/commit/c6bd9ecf95fdc54c955090837b57d6fdca82b7d1))

### 🐛 Bug Fixes

- Fallback to sh for non POSIX shells - ([7281bbb](https://github.com/dustinblackman/cargo-run-bin/commit/7281bbbf0a15b043d66e8f9f307485e210877df0))

### 📚 Documentation

- Add distro packages section ([#22](https://github.com/dustinblackman/cargo-run-bin/issues/22)) - ([7e8378c](https://github.com/dustinblackman/cargo-run-bin/commit/7e8378c0ebff7c044e475407583ed8caf6bf2ec0))
- Update version in README - ([42f87f9](https://github.com/dustinblackman/cargo-run-bin/commit/42f87f989cda444b841608db79c83cec2435a618))

### ⚙️ Miscellaneous Tasks

- Fix lint - ([a8a5d45](https://github.com/dustinblackman/cargo-run-bin/commit/a8a5d452f81eb29483f584d4bc636cfe2393386c))
- Drop owo - ([1dd54c4](https://github.com/dustinblackman/cargo-run-bin/commit/1dd54c478e35f22e4ba7fd1e0d46c100ecd64b62))
- Update bin deps - ([33a2c0f](https://github.com/dustinblackman/cargo-run-bin/commit/33a2c0ff5694f75e9ddc7af8f65b223ab090a32f))
- Drop dprint - ([02a968f](https://github.com/dustinblackman/cargo-run-bin/commit/02a968fc095cf8afc1615ea4cd4e936b4fc1e7e6))
- Update binstall - ([ea0de75](https://github.com/dustinblackman/cargo-run-bin/commit/ea0de75ef4b0ff8d4c99ce3ab0cdd051da9f7006))
- Update cargo-gha to latest - ([0dd4f2d](https://github.com/dustinblackman/cargo-run-bin/commit/0dd4f2d23d5909374f4513424490ff887a41d742))

## [1.7.2] - 2024-01-02
[v1.7.1..v1.7.2](https://github.com/dustinblackman/cargo-run-bin/compare/v1.7.1..v1.7.2)


### ⛰️  Features

- Release v1.7.2 - ([d11b188](https://github.com/dustinblackman/cargo-run-bin/commit/d11b1885a946214e4027a4f564f108b93c0ef0ea))

### ⚙️ Miscellaneous Tasks

- Reduce MSRV - ([813f0bc](https://github.com/dustinblackman/cargo-run-bin/commit/813f0bc35ece0092cf04be043cc89571dea12ff2))
- Remove version pinning - ([d466351](https://github.com/dustinblackman/cargo-run-bin/commit/d466351f0ea2098d063959b63585d24c9fd0e06d))

## [1.7.1] - 2024-01-01
[v1.7.0..v1.7.1](https://github.com/dustinblackman/cargo-run-bin/compare/v1.7.0..v1.7.1)


### ⛰️  Features

- Release v1.7.1 - ([c4c0861](https://github.com/dustinblackman/cargo-run-bin/commit/c4c0861672c77e0315070e30cf86719fa8e377b9))

### 🐛 Bug Fixes

- Version flag, no parameters - ([e00ebe3](https://github.com/dustinblackman/cargo-run-bin/commit/e00ebe3c93367df53c91a6eb08fe3e8dc6e9fda8))

## [1.7.0] - 2024-01-01
[v1.6.1..v1.7.0](https://github.com/dustinblackman/cargo-run-bin/compare/v1.6.1..v1.7.0)


### ⛰️  Features

- Release v1.7.0 - ([416012f](https://github.com/dustinblackman/cargo-run-bin/commit/416012fa4cf20bc8c1a233a76f738c71c25e7d08))
- Add get_shim_paths - ([fd60a88](https://github.com/dustinblackman/cargo-run-bin/commit/fd60a88054b194cd56e71faaff57044f47c7ce7f))
- Add docs and features to support library usage - ([e180db6](https://github.com/dustinblackman/cargo-run-bin/commit/e180db69aee09faff006435fb83da3c59155f1d8))

### 🐛 Bug Fixes

- Show help when no args are passed - ([6951082](https://github.com/dustinblackman/cargo-run-bin/commit/69510826e8c144093aeb0bfb7eb20475028e2d9d))

### 🚜 Refactor

- Run tests as integration test ([#17](https://github.com/dustinblackman/cargo-run-bin/issues/17)) - ([4c5c49a](https://github.com/dustinblackman/cargo-run-bin/commit/4c5c49aeff217333024945d51f5483160c8be8ec))
- Run tests as integration test - ([1e51847](https://github.com/dustinblackman/cargo-run-bin/commit/1e518477918de1d900702134b604f8a10d7ad265))

### 🧪 Testing

- Shims imports - ([ca5da68](https://github.com/dustinblackman/cargo-run-bin/commit/ca5da684618025b4ea19fa6699c5978d9e67298a))
- Move artifacts out of target - ([2f2fd57](https://github.com/dustinblackman/cargo-run-bin/commit/2f2fd579b60d0b5639c83dc7dc7dc8bd9cb337a9))

### ⚙️ Miscellaneous Tasks

- Merge branch 'lib-support' - ([c21ce48](https://github.com/dustinblackman/cargo-run-bin/commit/c21ce487ccbf72626f32a54e71fac531905a2a61))
- Fix warning on Windows builds - ([cadddd7](https://github.com/dustinblackman/cargo-run-bin/commit/cadddd77d84a072b62fdd6281cee4d87136d3539))
- Revert unit test move - ([cfb552e](https://github.com/dustinblackman/cargo-run-bin/commit/cfb552ebf08403dda681f8c4b5b2cb5c811b0173))
- Lint - ([834d089](https://github.com/dustinblackman/cargo-run-bin/commit/834d0898739fe86e47f5fa9f901b50dbcb041797))
- Fix the rust-version field in Cargo.toml ([#19](https://github.com/dustinblackman/cargo-run-bin/issues/19)) - ([23b2f39](https://github.com/dustinblackman/cargo-run-bin/commit/23b2f398f1cbff666384089bf88bb16430630ba7))

## [1.6.1] - 2023-12-16
[v1.6.0..v1.6.1](https://github.com/dustinblackman/cargo-run-bin/compare/v1.6.0..v1.6.1)


### ⛰️  Features

- Release v1.6.1 - ([5d18ad7](https://github.com/dustinblackman/cargo-run-bin/commit/5d18ad7e827fb7bb0417218610a13a23bf7b5fd0))

### 🐛 Bug Fixes

- Set MSRV - ([a5fc8ac](https://github.com/dustinblackman/cargo-run-bin/commit/a5fc8ac68dbba9f4f8a4b5d02bfd1e031bcdc56d))
- Release command - ([032e97f](https://github.com/dustinblackman/cargo-run-bin/commit/032e97f52c612d68c4668d333e46cf56015b7eb6))

## [1.6.0] - 2023-12-01
[v1.5.0..v1.6.0](https://github.com/dustinblackman/cargo-run-bin/compare/v1.5.0..v1.6.0)


### ⛰️  Features

- Release v1.6.0 - ([fd34f2b](https://github.com/dustinblackman/cargo-run-bin/commit/fd34f2bd1713eefacd59d429d73d71da3c638325))
- Merge branch 'windows' - ([a559325](https://github.com/dustinblackman/cargo-run-bin/commit/a5593250fc83922d9327a6896fed1c27625af73e))
- Add Windows support - ([8e4a6cc](https://github.com/dustinblackman/cargo-run-bin/commit/8e4a6cc298a94190fe4ab209d9fa48a9a1afc791))

### ⚙️ Miscellaneous Tasks

- Remove cargo-dist - ([553852d](https://github.com/dustinblackman/cargo-run-bin/commit/553852d8af838d7d5665e57ff5c4f8b9642c661e))
- Cleanup CI - ([42403a2](https://github.com/dustinblackman/cargo-run-bin/commit/42403a2d311e346f78d5d28f729229639f69bb4b))
- Add release script - ([2b10208](https://github.com/dustinblackman/cargo-run-bin/commit/2b10208d1e4a7be7fcf4a38a367525a82046ce2f))
- Add banner - ([ddca279](https://github.com/dustinblackman/cargo-run-bin/commit/ddca279206775df41a98d05f476d579e6145c586))

## [1.5.0] - 2023-10-27
[v1.4.1..v1.5.0](https://github.com/dustinblackman/cargo-run-bin/compare/v1.4.1..v1.5.0)


### ⛰️  Features

- Support path in package source - ([d4c8c39](https://github.com/dustinblackman/cargo-run-bin/commit/d4c8c392e5f8b40e330cd5b596eda68b1d378e79))

### 🐛 Bug Fixes

- Cargo.lock - ([505a155](https://github.com/dustinblackman/cargo-run-bin/commit/505a1559ae74893d55b9712ddafec0b57f7e1c5d))
- Upgrade cargo-binstall dep - ([e2f746c](https://github.com/dustinblackman/cargo-run-bin/commit/e2f746c37f3954688fb535318d73cddc4255daf4))
- Changelog - ([1ac6a3d](https://github.com/dustinblackman/cargo-run-bin/commit/1ac6a3d7cd331967c3bd5d9061d6b6862e33c030))

### 📚 Documentation

- Add path - ([21fc3db](https://github.com/dustinblackman/cargo-run-bin/commit/21fc3db067967e36994e297cc09236c089ecc9b9))

### ⚙️ Miscellaneous Tasks

- Release cargo-run-bin version 1.5.0 - ([7b1d981](https://github.com/dustinblackman/cargo-run-bin/commit/7b1d98199b17aa241f063e47a0ce3b1087968104))
- Fix binary.rs lint - ([8f50248](https://github.com/dustinblackman/cargo-run-bin/commit/8f5024858944f82a9660a8cc49b69ad244e6d6c3))

## [1.4.1] - 2023-10-06
[v1.4.0..v1.4.1](https://github.com/dustinblackman/cargo-run-bin/compare/v1.4.0..v1.4.1)


### 📚 Documentation

- Change changelog format - ([b784580](https://github.com/dustinblackman/cargo-run-bin/commit/b784580ebd613f273527bec147baad6b385b848a))

### ⚙️ Miscellaneous Tasks

- Release cargo-run-bin version 1.4.1 - ([4529a15](https://github.com/dustinblackman/cargo-run-bin/commit/4529a155562985e8a893317664997252eb389947))
- Lint - ([16124e7](https://github.com/dustinblackman/cargo-run-bin/commit/16124e7f64308981c902fca5d8e5c3e42863e086))
- Aliases -> Shims - ([00ca465](https://github.com/dustinblackman/cargo-run-bin/commit/00ca465948bc27afaaf767f263ff3c62e464f003))
- `tauri-mobile` -> `cargo-mobile2` - ([e2130fe](https://github.com/dustinblackman/cargo-run-bin/commit/e2130fe1b0813e153ac49ac78bdbb3d289d64bc2))

## [1.4.0] - 2023-10-04
[v1.3.2..v1.4.0](https://github.com/dustinblackman/cargo-run-bin/compare/v1.3.2..v1.4.0)


### ⛰️  Features

- Support git in package source - ([7151499](https://github.com/dustinblackman/cargo-run-bin/commit/7151499b4b47861f84c63b958af8cd36aa8648a9))

### 📚 Documentation

- Add git - ([a97993c](https://github.com/dustinblackman/cargo-run-bin/commit/a97993c9abb74259084f37857a40c3f6b2dad576))

### 🧪 Testing

- Additional git flows - ([a6144eb](https://github.com/dustinblackman/cargo-run-bin/commit/a6144ebcb9bb38479a78b8caf50afbd27551983d))

### ⚙️ Miscellaneous Tasks

- Release cargo-run-bin version 1.4.0 - ([32f72e8](https://github.com/dustinblackman/cargo-run-bin/commit/32f72e8b600164ae188af332af66954e177f88e7))
- Lint - ([ebba9c4](https://github.com/dustinblackman/cargo-run-bin/commit/ebba9c44f97ab0331f8114b72a99caf60c0d8de4))

## [1.3.2] - 2023-09-20
[v1.3.1..v1.3.2](https://github.com/dustinblackman/cargo-run-bin/compare/v1.3.1..v1.3.2)


### 🐛 Bug Fixes

- Deserailizing default-features - ([3c15340](https://github.com/dustinblackman/cargo-run-bin/commit/3c153408947f3aee7a09d7041c9a1d9fdfe5f3a5))

### ⚙️ Miscellaneous Tasks

- Release cargo-run-bin version 1.3.2 - ([be3cc40](https://github.com/dustinblackman/cargo-run-bin/commit/be3cc4059b7d961819d3c47268f30ee1b87b0ec1))

## [1.3.1] - 2023-09-20
[v1.3.0..v1.3.1](https://github.com/dustinblackman/cargo-run-bin/compare/v1.3.0..v1.3.1)


### 🐛 Bug Fixes

- Duplicate PATH entries - ([0914ff7](https://github.com/dustinblackman/cargo-run-bin/commit/0914ff7ba3316d3b9cddc025febf75df3135a002))

### ⚙️ Miscellaneous Tasks

- Release cargo-run-bin version 1.3.1 - ([83349f9](https://github.com/dustinblackman/cargo-run-bin/commit/83349f99e01d665336d26ff7753ffb2a5a6ffb12))

## [1.3.0] - 2023-09-20
[v1.2.0..v1.3.0](https://github.com/dustinblackman/cargo-run-bin/compare/v1.2.0..v1.3.0)


### ⛰️  Features

- Add cargo-gha to path if it exists - ([c404b48](https://github.com/dustinblackman/cargo-run-bin/commit/c404b480e17e9abf12586dc865111d7546609849))
- Add alias scripts - ([3c6dbac](https://github.com/dustinblackman/cargo-run-bin/commit/3c6dbace4e5f2e1442f7da9805aa5a61e8ba5c9b))
- Add crate features config - ([e7d0f4d](https://github.com/dustinblackman/cargo-run-bin/commit/e7d0f4d9ccff32aeb9a04088d84ce933d85d4885))

### ⚙️ Miscellaneous Tasks

- Release cargo-run-bin version 1.3.0 - ([f044465](https://github.com/dustinblackman/cargo-run-bin/commit/f044465c6df800fd4694334982cffa76a7157ec2))
- Update lockfile - ([c6f752f](https://github.com/dustinblackman/cargo-run-bin/commit/c6f752f24a01077fb260c9561c7b4aeed1972fdf))

## [1.2.0] - 2023-08-26
[v1.1.5..v1.2.0](https://github.com/dustinblackman/cargo-run-bin/compare/v1.1.5..v1.2.0)


### ⛰️  Features

- Add support for Cargo workspaces - ([208b000](https://github.com/dustinblackman/cargo-run-bin/commit/208b000f47ee7666009a066a9f2a5124275ea381))

### 🐛 Bug Fixes

- Nightly setup - ([c50b3a5](https://github.com/dustinblackman/cargo-run-bin/commit/c50b3a520f5f0aee7579f816fc8b89b6309fd5d5))

### 📚 Documentation

- Add commit hashes to changelog - ([28e7c84](https://github.com/dustinblackman/cargo-run-bin/commit/28e7c8488da0972b382cb729ad07e208b3535e81))

### ⚙️ Miscellaneous Tasks

- Release cargo-run-bin version 1.2.0 - ([b600075](https://github.com/dustinblackman/cargo-run-bin/commit/b600075b1af551d509fdf9fcfeeff97d74c5d446))

## [1.1.5] - 2023-08-12
[v1.1.4..v1.1.5](https://github.com/dustinblackman/cargo-run-bin/compare/v1.1.4..v1.1.5)


### 🐛 Bug Fixes

- Help command when using cargo - ([25ace47](https://github.com/dustinblackman/cargo-run-bin/commit/25ace47c61a1c7d4e46d734619917ac8539fa2df))

### 📚 Documentation

- Fix links - ([c40edbf](https://github.com/dustinblackman/cargo-run-bin/commit/c40edbfb888aec37400dd70bc5dc39b60d5e65b8))
- Update parameters table - ([5eae9a9](https://github.com/dustinblackman/cargo-run-bin/commit/5eae9a9968478e7fc1eeae6b14333ef83e8283c7))

### 🧪 Testing

- CLI integration snapshots - ([2524b71](https://github.com/dustinblackman/cargo-run-bin/commit/2524b7111f7cb2ac6cbc31f07143c6702f0844ac))

### ⚙️ Miscellaneous Tasks

- Release cargo-run-bin version 1.1.5 - ([d803d74](https://github.com/dustinblackman/cargo-run-bin/commit/d803d74f79384f6b9707eecc6bc58a156f7d3ba4))
- Update dprint - ([499c1bc](https://github.com/dustinblackman/cargo-run-bin/commit/499c1bccc3e1b5b7cb4a61f6610892c2c9fbb5b6))
- Speed up nightly setup command - ([dd30048](https://github.com/dustinblackman/cargo-run-bin/commit/dd30048696a9e7e013415a0ba8820d6afeb763a7))
- Add release profile - ([e12575c](https://github.com/dustinblackman/cargo-run-bin/commit/e12575ced1d842c29158f17c9826e6fd24517883))
- Upgrade binstall to 1.2.0 - ([f015f5c](https://github.com/dustinblackman/cargo-run-bin/commit/f015f5ca265c7d0ac9af9db364e7e1456355d956))
- Remove unused deps - ([2562b7f](https://github.com/dustinblackman/cargo-run-bin/commit/2562b7f16e7b0bed1c92bc1a83301fae5d1697eb))
- Add commit linting - ([b6b358c](https://github.com/dustinblackman/cargo-run-bin/commit/b6b358cc5781b899d8362ae153564ff9060feda5))
- Add release and changelog scripts - ([6b0e2b0](https://github.com/dustinblackman/cargo-run-bin/commit/6b0e2b0d9b6ce0bd6302ca87f38d592ef22824a4))

## [1.1.4] - 2023-08-08
[v1.1.3..v1.1.4](https://github.com/dustinblackman/cargo-run-bin/compare/v1.1.3..v1.1.4)


### ⚙️ Miscellaneous Tasks

- Release v1.1.4 - ([494d7ac](https://github.com/dustinblackman/cargo-run-bin/commit/494d7acfefc7ac8a3e2124b50fcd3f951fc5d196))
- Drop windows releases - ([d65fb49](https://github.com/dustinblackman/cargo-run-bin/commit/d65fb495283573ee2ecac0c7edba66c9551b6a95))

## [1.1.3] - 2023-08-08
[v1.1.2..v1.1.3](https://github.com/dustinblackman/cargo-run-bin/compare/v1.1.2..v1.1.3)


### 🐛 Bug Fixes

- Release workflow - ([f012617](https://github.com/dustinblackman/cargo-run-bin/commit/f012617a3993c58608bcffe9e2d4b9379bbc25d9))

### ⚙️ Miscellaneous Tasks

- Release v1.1.3 - ([7783dbd](https://github.com/dustinblackman/cargo-run-bin/commit/7783dbd4b9f2460c5900c7e00b65d9ec2ab4e54b))

## [1.1.2] - 2023-08-08
[v1.1.1..v1.1.2](https://github.com/dustinblackman/cargo-run-bin/compare/v1.1.1..v1.1.2)


### 🐛 Bug Fixes

- Release workflow - ([27ce8e0](https://github.com/dustinblackman/cargo-run-bin/commit/27ce8e0f63a4a7181ba780791ac8a206fbbe5d5d))

### ⚙️ Miscellaneous Tasks

- Release v1.1.2 - ([883a75f](https://github.com/dustinblackman/cargo-run-bin/commit/883a75ffad28d62d29c7ab87b7cc8bc186898dea))

## [1.1.1] - 2023-08-08
[v1.0.1..v1.1.1](https://github.com/dustinblackman/cargo-run-bin/compare/v1.0.1..v1.1.1)


### ⛰️  Features

- Add binstall support - ([f702124](https://github.com/dustinblackman/cargo-run-bin/commit/f702124692e23f8c14040cf0935b90c4877ed4ee))

### 🐛 Bug Fixes

- Release command - ([af9715b](https://github.com/dustinblackman/cargo-run-bin/commit/af9715b4f741ede4556a6fecfb8d70a9dbd32a9a))

### ⚙️ Miscellaneous Tasks

- Release v1.1.1 - ([a6ddf14](https://github.com/dustinblackman/cargo-run-bin/commit/a6ddf14394f1c251f55899257931a374466bed76))
- Drop cargo release - ([f87e73c](https://github.com/dustinblackman/cargo-run-bin/commit/f87e73c8a2b6fa68832363b0e28f3c106f5a622b))
- Add artifact releases - ([ced0630](https://github.com/dustinblackman/cargo-run-bin/commit/ced06305199c2a47b3465ba23efeeccfd9dfb722))
- Add cargo-cmd - ([150af5b](https://github.com/dustinblackman/cargo-run-bin/commit/150af5b4e1a75f5ca7b816716dbbcd2be0c66000))
- Add clippy to toolchain - ([aa7161e](https://github.com/dustinblackman/cargo-run-bin/commit/aa7161e0bf3ff522144e318b26746359801b0b07))

## [1.0.1] - 2023-08-07
[v1.0.0..v1.0.1](https://github.com/dustinblackman/cargo-run-bin/compare/v1.0.0..v1.0.1)


### 🐛 Bug Fixes

- Sorting aliases - ([b4ccdd1](https://github.com/dustinblackman/cargo-run-bin/commit/b4ccdd189376506d64d1911de0f17e3c49dda0d0))
- Integration tests with code coverage commands - ([9a940f7](https://github.com/dustinblackman/cargo-run-bin/commit/9a940f7dee59fca6136d4f82a39b141fae95f580))
- Test lcov - ([49ea7c1](https://github.com/dustinblackman/cargo-run-bin/commit/49ea7c10cb3a311dc10dd4bef2f9a76668419378))
- Test scripts exiting - ([1440910](https://github.com/dustinblackman/cargo-run-bin/commit/1440910eadfc638cb6a1da65807a2e24bb7e0a97))
- Lint - ([74b66ad](https://github.com/dustinblackman/cargo-run-bin/commit/74b66ad09d2346e7def0ea7275e16f4bd37bd5e6))

## [1.0.0] - 2023-08-07
[..v1.0.0](https://github.com/dustinblackman/cargo-run-bin/compare/..v1.0.0)


### ⛰️  Features

- Add sync aliases - ([5d7ffed](https://github.com/dustinblackman/cargo-run-bin/commit/5d7ffed87bb9ff7b78ce7a228379a35492e45f11))
- Add build and run functionality - ([9a7218c](https://github.com/dustinblackman/cargo-run-bin/commit/9a7218c20c09797097b518a605c1d2b6c365ec30))
- Rewrite reading cargo.toml - ([12bccba](https://github.com/dustinblackman/cargo-run-bin/commit/12bccbaf5e60f7781f27b6b28fb9c4bd0922acc4))
- Add metadata options for binaries - ([3d176a1](https://github.com/dustinblackman/cargo-run-bin/commit/3d176a170b7b1bec81eb8a06d98706164b218657))

### 🐛 Bug Fixes

- Llvm-cov - ([257af9d](https://github.com/dustinblackman/cargo-run-bin/commit/257af9dc7903eddcf923e0ab55b3575917d0baaf))
- Cargo commands, add initial tests - ([66a2271](https://github.com/dustinblackman/cargo-run-bin/commit/66a2271ca4ea57013d85d36a528c749d057d2959))

### 📚 Documentation

- Format - ([9f1b0db](https://github.com/dustinblackman/cargo-run-bin/commit/9f1b0dbba03293a3f120bfc8f4dcf0eeb1540734))
- Update readme - ([8ab7f77](https://github.com/dustinblackman/cargo-run-bin/commit/8ab7f777fcbd99bb1e50067476ad621b0102013e))

### 🧪 Testing

- Cli integration tests - ([d087a06](https://github.com/dustinblackman/cargo-run-bin/commit/d087a066c3b160240a33c7096e38d3414b8fcaf1))
- Cargo_config - ([281b18b](https://github.com/dustinblackman/cargo-run-bin/commit/281b18b52516c8af89f16e2266c11aa33e321939))
- Binary - ([12146af](https://github.com/dustinblackman/cargo-run-bin/commit/12146afcb515b837a36900afcc3185a0987aec36))
- Bin target packages - ([edf2c1f](https://github.com/dustinblackman/cargo-run-bin/commit/edf2c1f905c28f68244f5af65bdf79e55b233d8f))

### ⚙️ Miscellaneous Tasks

- Add release command - ([b40f043](https://github.com/dustinblackman/cargo-run-bin/commit/b40f0436e8eb231975168fda87fdb8da0f675d0a))
- Update test commands - ([c12a4d6](https://github.com/dustinblackman/cargo-run-bin/commit/c12a4d6c90aed2cc944082dae4f779a5cd8cc5cf))
- Update ci - ([6671333](https://github.com/dustinblackman/cargo-run-bin/commit/6671333b5978890d804b2be71f2d3f2f1299f83e))
- Lint - ([cd157a7](https://github.com/dustinblackman/cargo-run-bin/commit/cd157a727734d06322a4941ff35aa3db6a092670))
- Clean error outputs, unused deps - ([b0171f4](https://github.com/dustinblackman/cargo-run-bin/commit/b0171f4a4d321dae6c783d224ab645ffaa67a555))
- Clean - ([10216bb](https://github.com/dustinblackman/cargo-run-bin/commit/10216bb2399e3d22ef71f6d87954cf207eab1ee0))
- Upgrade cargo_toml - ([b81dcd6](https://github.com/dustinblackman/cargo-run-bin/commit/b81dcd607089b05eb939e915514334b070a4bb35))
- Drop fstrings - ([b5388e3](https://github.com/dustinblackman/cargo-run-bin/commit/b5388e33b000d54444fa5213472546eb3164872d))
- Format - ([cbc9e3a](https://github.com/dustinblackman/cargo-run-bin/commit/cbc9e3ab710c863ef28fcb68583ab4bb07c9dadc))

<!-- generated by git-cliff -->
