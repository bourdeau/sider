# Changelog

## [0.7.0](https://github.com/bourdeau/sider/compare/v0.6.0...v0.7.0) (2025-03-08)


### Features

* Multiplexing ([#79](https://github.com/bourdeau/sider/issues/79)) ([cfe4a22](https://github.com/bourdeau/sider/commit/cfe4a22981225eb433451ffe8eb9d9e6de4abc16))


### Bug Fixes

* logging DB restore ([#85](https://github.com/bourdeau/sider/issues/85)) ([a9bb69e](https://github.com/bourdeau/sider/commit/a9bb69e0c39b68a835859ba742308ba4cf97076c))

## [0.6.0](https://github.com/bourdeau/sider/compare/v0.5.1...v0.6.0) (2025-02-24)


### Features

* RESP ([#77](https://github.com/bourdeau/sider/issues/77)) ([8d777a0](https://github.com/bourdeau/sider/commit/8d777a0da99014b905f28415d75c4eb36846adc5))

## [0.5.1](https://github.com/bourdeau/sider/compare/v0.5.0...v0.5.1) (2025-02-18)


### Bug Fixes

* AOF read cmd (closes [#64](https://github.com/bourdeau/sider/issues/64)) ([#72](https://github.com/bourdeau/sider/issues/72)) ([870bee3](https://github.com/bourdeau/sider/commit/870bee32e56dbe13b80dcc77018267a2862402ca))
* ttl for all keys (closes [#45](https://github.com/bourdeau/sider/issues/45))  ([#75](https://github.com/bourdeau/sider/issues/75)) ([9a949f5](https://github.com/bourdeau/sider/commit/9a949f5c87bb4cbefdd574cc9774fef676ea82e7))

## [0.5.0](https://github.com/bourdeau/sider/compare/v0.4.1...v0.5.0) (2025-02-15)


### Features

* HDEL ([#67](https://github.com/bourdeau/sider/issues/67)) ([0260bb6](https://github.com/bourdeau/sider/commit/0260bb6ccdce363082bd04fa3932d28d70f95187))
* HGET ([#62](https://github.com/bourdeau/sider/issues/62)) ([78f13e6](https://github.com/bourdeau/sider/commit/78f13e6b0dc35d7380ce92834eb11c584e9aecc9))
* HGETALL ([#65](https://github.com/bourdeau/sider/issues/65)) ([ebf4d6b](https://github.com/bourdeau/sider/commit/ebf4d6bbabd5673d88aa0281db0555d98810bb08))
* HSET ([#59](https://github.com/bourdeau/sider/issues/59)) ([2face4c](https://github.com/bourdeau/sider/commit/2face4c2e73cddde541647133c0aee6598a60ff0))


### Bug Fixes

* double quotes (closes [#61](https://github.com/bourdeau/sider/issues/61)) ([#69](https://github.com/bourdeau/sider/issues/69)) ([e28a928](https://github.com/bourdeau/sider/commit/e28a9285cac8a446bc3ab69c45a2a463e58eb6c9))

## [0.4.1](https://github.com/bourdeau/sider/compare/v0.4.0...v0.4.1) (2025-02-11)


### Bug Fixes

* IndexMap to preserve order & unit test ([#54](https://github.com/bourdeau/sider/issues/54)) ([e034745](https://github.com/bourdeau/sider/commit/e0347452a46d04dc286a843e42140ab4a7de2348))

## [0.4.0](https://github.com/bourdeau/sider/compare/v0.3.1...v0.4.0) (2025-02-10)


### Features

* clean up database task (closes [#33](https://github.com/bourdeau/sider/issues/33)) ([#38](https://github.com/bourdeau/sider/issues/38)) ([82a92be](https://github.com/bourdeau/sider/commit/82a92be914359c99dcb88198a0a7ef1b9ce99f0f))
* Configuration & CLI (closes [#34](https://github.com/bourdeau/sider/issues/34)) ([#36](https://github.com/bourdeau/sider/issues/36)) ([fb7a799](https://github.com/bourdeau/sider/commit/fb7a7998bc51c7273903f3fcc6b16675b135bdb9))
* LPOP ([#49](https://github.com/bourdeau/sider/issues/49)) ([d06e7e6](https://github.com/bourdeau/sider/commit/d06e7e669dab0ac48d9914373d3991fff73fa88e))
* LPUSH & Logging ([#31](https://github.com/bourdeau/sider/issues/31)) ([22ef7af](https://github.com/bourdeau/sider/commit/22ef7af03b7c4674f3fa4237e464cc7b32545faf))
* LRANGE ([#44](https://github.com/bourdeau/sider/issues/44)) ([6a0e73c](https://github.com/bourdeau/sider/commit/6a0e73c72eeec4688fa329c4cf43713f9f882e9d))
* RPOP ([#51](https://github.com/bourdeau/sider/issues/51)) ([247b663](https://github.com/bourdeau/sider/commit/247b6632bb7cb38d60d40c15d9d0f82766f40dfa))
* RPUSH ([#47](https://github.com/bourdeau/sider/issues/47)) ([e1eb9b4](https://github.com/bourdeau/sider/commit/e1eb9b4268cf75ff1c0b59230b995200d01a45fb))


### Bug Fixes

* IO blocking reading files ([#43](https://github.com/bourdeau/sider/issues/43)) ([65fb016](https://github.com/bourdeau/sider/commit/65fb01602d2cdcd34be0d20b00095ca5d2cf9e3b))

## [0.3.1](https://github.com/bourdeau/sider/compare/v0.3.0...v0.3.1) (2025-02-08)


### Bug Fixes

* DEL return & refactoring (closes [#14](https://github.com/bourdeau/sider/issues/14)) ([#29](https://github.com/bourdeau/sider/issues/29)) ([7278e20](https://github.com/bourdeau/sider/commit/7278e2049ed347fb77dcb8768156a83313a94fa3))

## [0.3.0](https://github.com/bourdeau/sider/compare/v0.2.0...v0.3.0) (2025-02-07)


### Features

* DECR ([#26](https://github.com/bourdeau/sider/issues/26)) ([b2b10c3](https://github.com/bourdeau/sider/commit/b2b10c3ea012341f76b03d905c085546907f76ec))
* INCR ([#24](https://github.com/bourdeau/sider/issues/24)) ([a2aaf08](https://github.com/bourdeau/sider/commit/a2aaf0833d91e435cd7318bd76474dd321efd847))
* INCRBY ([#27](https://github.com/bourdeau/sider/issues/27)) ([0b3b346](https://github.com/bourdeau/sider/commit/0b3b346647387e12e2062871c5b2fc42197aa09d))

## [0.2.0](https://github.com/bourdeau/sider/compare/v0.1.0...v0.2.0) (2025-02-07)


### Features

* add build to CI ([#4](https://github.com/bourdeau/sider/issues/4)) ([2e86a73](https://github.com/bourdeau/sider/commit/2e86a73432ea0735833fa3b4a9ec1daeed78f43e))
* AOF ([#21](https://github.com/bourdeau/sider/issues/21)) ([ed1b371](https://github.com/bourdeau/sider/commit/ed1b3713378db35fb27a495b4d22d77252214819))
* command EXISTS ([#17](https://github.com/bourdeau/sider/issues/17)) ([69edff8](https://github.com/bourdeau/sider/commit/69edff8e14c9dfc20906e0a367b9332f7bd3aa43))
* DEL multiple keys ([#8](https://github.com/bourdeau/sider/issues/8)) ([e655f80](https://github.com/bourdeau/sider/commit/e655f80b17240f1a9769d59d3cc6a5408c1ad0a1))
* delete multiple keys ([#10](https://github.com/bourdeau/sider/issues/10)) ([0bf7b43](https://github.com/bourdeau/sider/commit/0bf7b43aec4b17fa93f58c671e98d73732c20795))
* EXPIRE & TTL ([#19](https://github.com/bourdeau/sider/issues/19)) ([a545935](https://github.com/bourdeau/sider/commit/a54593503a15c01787df8c226b8de1ac034d81b8))


### Bug Fixes

* added Clippy ([#15](https://github.com/bourdeau/sider/issues/15)) ([b737332](https://github.com/bourdeau/sider/commit/b73733262f3a0c2f8c8f1f71f3459ba7cce26c47))
* upgrade tokio ([#6](https://github.com/bourdeau/sider/issues/6)) ([7ba1032](https://github.com/bourdeau/sider/commit/7ba103265f1bf3ffb7fae885a5057fd0aab79286))

## 0.1.0 (2025-02-04)


### Bug Fixes

* path  of rustfmt ([#1](https://github.com/bourdeau/sider/issues/1)) ([6aca6d7](https://github.com/bourdeau/sider/commit/6aca6d714519141b98a19d1bb763aa81149cf0d9))
* release branch ([#2](https://github.com/bourdeau/sider/issues/2)) ([37bbefe](https://github.com/bourdeau/sider/commit/37bbefe087cdff204754437de5a8317d8a3b3bcb))
