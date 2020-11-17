# qutonium

<p align="center">
  <p align="left">
    <a href="https://github.com/qurity">
      <img alt="github" src="https://img.shields.io/badge/github.com/qurity-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">
    </a>
    <a href="https://crates.io/crates/qutonium">
      <img alt="crates.io" src="https://img.shields.io/crates/v/qutonium.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">
    </a>
    <a href="https://docs.rs/qutonium">
      <img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-qutonium-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K" height="20">
    </a>
    <a href="https://crate-trends.herokuapp.com/qutonium">
<img alt="Crates.io" src="https://img.shields.io/crates/d/qutonium?style=for-the-badge&labelColor=555555" height="20">
    </a>
  </p>
</p>

---

> *simplify unit testing with a minimal testing framework* 📈 

## <a name="disclaimer"></a> DISCLAIMER

[qutonium](https://github.com/qurity/qutonium) is takes is inspiration from the [rust-testing-microframework](https://github.com/wilkie/rust-testing-microframework). the goal is to provide a minimal testing framework for assertions (see the list of features [here](#goals)). the project is still in work in progress so.. DO NOT USE IN PRODUCTION. DO NOT USE IN PRODUCTION. DO NOT USE IN PRODUCTION. feedbacks appreciated!

## <a name="example"></a> Examples

more examples [here](https://github.com/qurity/qutonium/tree/master/examples)   

**basic syntax**

```rust
fn main () {}

#[cfg(test)]
mod tests {
  extern crate qutonium;
  use qutonium::prelude::*;

  #[test]
  fn from_test () {
    suite!("the suite case name ~basic", {
      "compare integer" || {
        expect!(2 + 2).to(be_equal(4));
      }
    });
  }
}
```

**sugar syntax**

```rust
fn main () {}

#[cfg(test)]
mod tests {
  extern crate qutonium;
  use qutonium::prelude::*;

  #[test]
  fn from_test () {
    suite!("the suite case name ~sugar", {
      "compare integer" || {
        must!(2 + 2; be 4);
      }
    });
  }
}
```

## Overview

**stdout**

The output format follow the syntax from `crystal` | `ruby`. s/o [Siegfried Ehret](https://ehret.me/fr/crystal-01.html)   

```
SUITE the suite case name
  TEST compare integer (0ms)
    ERROR
       👎 received: <0> expected: <1>
       📍 src/main.rs:7:7
    END
  END
  TEST compare boolean (0ms)
    SUCCESS
       👍 ok
       📍 src/main.rs:12:7
    END
  END
  TEST testing should panic (0ms)
    SUCCESS
       👍 ok
       📍 src/main.rs:16:7
    END
  END
END

DIAGNOSTIC
  👎 failed: 1   👍 passed: 2   📈 total: 3
END
```

## Usage

Add the following to your Cargo.toml:

```toml
[dependencies]
qutonium = "0.1.3"
```

## <a name="goals"></a> Goals

* [x] describe suite case | `suite!`
* [x] test function |
* [x] assertions | `expect`
* [x] marker | `not` & `to`
* [x] `matchers` | see the complete list here
* [ ] extended `matchers`
* [ ] custom `matchers`
* [ ] diagnostics
* [x] error `location`
* [ ] helpful error messages
* [ ] sugar syntax
* [ ] no nested suites
* [ ] inline stdout
* [x] colorized terminal output | using [qute](https://github.com/qurity/qute) crate
* [ ] `snapshot` diff | using [differences](https://github.com/johannhof/difference.rs) crate
* [x] extended `matchers`
* [x] hooks | `before:all`, `before:each`, `after:all` and `after:each`
* [ ] export `json`, `xml` | [generic execution format](https://docs.sonarqube.org/latest/analysis/generic-test/#header-2) for sonar *(xml only)*
* [ ] Exclude test cases | `skip` and `only`
* [ ] `store` | state container
* [ ] should `panic` can be testing
* [ ] writing unit testing
* [ ] documentation

## <a name="api"></a> API

### Exports

**json**

| handler | snippet                                                             |
|---------|---------------------------------------------------------------------|
| to_json | see more informations about the export [here](./doc/export/json.md) |

### Markers

| marker | snippet                     |
|---------|----------------------------|
| to      | `expect!(expr).to(matcher)`   |
| not     | `expect!(expr).not(matcher)`  |

### Matchers

**boolean**

| matcher   | snippet                         |
|-----------|---------------------------------|
| be_false  | `expect!(expr).to(be_false())`  |
| be_true   | `expect!(expr).to(be_true())`   |

**common**

| matcher     | snippet                              |
|-------------|--------------------------------------|
| be_equal    | `expect!(expr).to(be_equal(expr))`   |

## <a name="license"></a> License   

Copyright ©️ 2020 Qurity    

Released under the [MIT](LICENSE) license   
