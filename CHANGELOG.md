# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).


## [Unreleased]


## [1.0.0-beta.4] - 2023-07-01

This crate has been renamed to [`beancount-parser`](https://github.corm/jcornaz/beancount-parser). The old name `beancount_parser_2` is now discontinued in favor of `beancount-parser`.

See [beancount-parser 2.0.0-beta.1](https://github.com/jcornaz/beancount-parser/releases/tag/v2.0.0-beta.1)


## [1.0.0-beta.3] - 2023-06-19

This release finishes the work of removing lifetimes from the result types that was started `1.0.0-beta.2`.

### Breaking changes

* All `HashMap<&str, MetadataValue<D>>` have been replaced with `metadata::Map<D>`
* The types `Transaction`, `Posting`, `Event`, `Directive`, `BeanOption`, `BeancountFile` and `Error` no longer have lifetimes
* The fields `payee` and `narration` in `Transaction` are now of type `Option<String>` (formerly `Option<&str>`)
* The fields `tags` and `links` in `Transaction` are now of type `HashSet<Tag>` and `HashSet<Link>` respectively (formerly `HashSet<&str>`)
* The field `includes` in `BeancountFile` is now of type `PathBuf` (formerly `&Path`)
* Removed deprecated `includes()` getter from `BeancountFile`

### Added

* implement `Borrow<str>` and `AsRef<str>` for `Currency` and `Amount`


### Deprecated

* The type alias `MetadataValue` is deprecated. Use its target type directly instead `metadata::Value`


## [1.0.0-beta.2] - 2023-06-18

I realized that there is a major flaw in the design of having the parsing result bound to the lifetime of the input.
Those lifetime becomes very problematic when one wants to solve the `include` directives, because it becomes very difficult (if possible at all?)
to write code that dynamically loads a file, parse it, do the same for each includes and merge everything together in a single data-structure
before starting to analyze the ledger as a whole.

Therefore, I started the process of removing lifetimes from the parser's result.
Once that'll be done, I'll try to add string interning for currencies, accounts and metadata keys, to hopefully mitigate the (CPU & memory) performance cost of this new approach.


### Breaking changes

* The type `Account`, `Currency`, `MetadataValue`, `Open`, `Close`, `Pad`, `Price` and `Balance` no longer have a lifetime parameter
* The type `Account` and `Currency` no longer implements `Copy`, but are still reasonably cheap to clone (they contain an `Arc`)
* The enum `Flag` has been removed
* The `flag` field in `Transaction` and `Posting` is now of type `Option<char>`


### Added

* Implement `Display` for `Account` and `Currency`
* Implement `Ord` for `Date`
* Implement `TryFrom<&str>` for `Currency`
* Accept any non-lowercase character as transaction/posting flag (example of valid flags: `*`, `!`, `?`, `P`)


## [1.0.0-beta.1] - 2023-06-17

### Added

* `BeanOption` type
* Make the `includes` and `options` fields public
* Support multiple options with the same key


### Deprecated

* The `includes` getter in `BeancountType`. Use the field instead. 


## [1.0.0-alpha.12] - 2023-06-16

### Fixed 

* Support comment or empty line between postings (#29) 
* Support comment or empty line between metadata 


## [1.0.0-alpha.11] - 2023-06-15

### Fixed

* Error when account component started with a number


### Thank you

@doriath


## [1.0.0-alpha.10] - 2023-06-13

### Added

* Support metadata in posting

### Thank you

@doriath


## [1.0.0-alpha.9] - 2023-06-13

### Added

* Expose the `Cost` type publicly

### Documentation

* Fix mixed-up documentation of `cost` and `price`


## [1.0.0-alpha.8] - 2023-06-11

* Support links (^link) in transaction directive.

## [1.0.0-alpha.7] - 2023-06-11

### Improved

* Don't dump the full end-of-input when debugging the content of an error


## [1.0.0-alpha.6] - 2023-06-05

### Breaking changes

This Release reverts the change made on the last release which made the parser generic over the string type.
The function `parse` and all structs have now one less generic argument, and instead are bound to the lifetime of the input.

The change turned out to not be as beneficial as anticipated for a higher cost in complexity than anticipated.


#### Other breaking changes

* Make private the field `options` from `BeancountFile`. Use the new `option` getter instead.
* Make private the field `includes` from `BeancountFile`. Use the new `includes` iterator instead.
* Make private the field `directives` from `BeancountFile`. Use the new `directives` or `take_directives` methods instead.


### Dependencies

* Rust requirement (MSRV) bumped to 1.70


### Added

* `BeancountFile::option` getter


## [1.0.0-alpha.5] - 2023-05-31

### Breaking changes

The `parse` method and all structs are now generics over the string type `S`.
One must choose how to store strings in the results, with either `&str` or `String`.

With `parse::<&str, D>` the results will contains string slices from the input string.
This is very performant but a bit less convenient to use as one cannot move nor discard the input string until done working with the results.

With `parse::<String, D>` more memory will be allocated to copy strings from the input.
This is less performant but allow continue using the results after discarding the input string.


### Relaxed requirements

* Require less traits for the `Decimal` type, and extend the blanket implementation accordingly


### Documentation

* Document `Event` directive
* Document `Flag` directive


## [1.0.0-alpha.4] - 2023-05-27

### Breaking changes

* `MetadataValue` is now generic over the decimal type `D`
* `currencies` in the `Open` directive is now a `HashSet` instead of `Vec`


### New syntax supported

* `include` directive
* `pushtag` and `poptag` operations
* number and currency as metadata value


### Fixed

* Accept comment on `option` directive


### Documentation

* Document `Price`, `Amount`, `Currency` and `MetadataValue` types


## [1.0.0-alpha.3] - 2023-05-24

### Breaking changes

* The decimal type must now implement the `beancount_parser_2::Decimal` trait.
  There is a blanket implementation for all types that could be used as a decimal type,
  including `f64` and `rust_decimal::Decimal`


### New syntax supported

* Expression for amount value


### Documentation

* Document `Account` and related directives (`Open`, `Close`, `Balance` and `Pad`)


## [1.0.0-alpha.2] - 2023-05-22


### Breaking changes

* The type of the `price` field in `Posting` has changed to `Option<PostingPrice<'a, D>>`.
* The type `MetadataValue` no longer implements `Eq`

### Added

* Support for total price in posting (`@@` syntax)
* implement `Clone` for all types
* implement `Copy`, `Eq`, `Ord` and `Hash` on `Account` and `Currency`

### Documentation

* Write documentation for the `Transaction` and `Posting` types


## [1.0.0-alpha.1] - 2023-05-21

### Added

* Support for the pad directive
* Line number in error
* Line number in directive
* implement `std::error::Error` for `Error`

### Documentation

* Improve/write documentation for `parser`, `BeancountFile`, `Directive`, `Error` and `Date`


## [1.0.0-alpha.0] - 2023-05-20

### Supported beancount syntax

* Transaction
  * flag
  * payee and description
  * tags
  * postings
    * account
    * amount
    * price
    * cost
      * amount
      * date
* Price directive
* Open and close directives
* Balance assertion
* Commodity declaration
* Events
* Options
* Directive metadata (string values only)

[Unreleased]: https://github.com/jcornaz/beancount_parser_2/compare/v1.0.0-beta.4...HEAD
[1.0.0-beta.4]: https://github.com/jcornaz/beancount_parser_2/compare/v1.0.0-beta.3...v1.0.0-beta.4
[1.0.0-beta.3]: https://github.com/jcornaz/beancount_parser_2/compare/v1.0.0-beta.2...v1.0.0-beta.3
[1.0.0-beta.2]: https://github.com/jcornaz/beancount_parser_2/compare/v1.0.0-beta.1...v1.0.0-beta.2
[1.0.0-beta.1]: https://github.com/jcornaz/beancount_parser_2/compare/v1.0.0-alpha.12...v1.0.0-beta.1
[1.0.0-alpha.12]: https://github.com/jcornaz/beancount_parser_2/compare/v1.0.0-alpha.11...v1.0.0-alpha.12
[1.0.0-alpha.11]: https://github.com/jcornaz/beancount_parser_2/compare/v1.0.0-alpha.10...v1.0.0-alpha.11
[1.0.0-alpha.10]: https://github.com/jcornaz/beancount_parser_2/compare/v1.0.0-alpha.9...v1.0.0-alpha.10
[1.0.0-alpha.9]: https://github.com/jcornaz/beancount_parser_2/compare/v1.0.0-alpha.8...v1.0.0-alpha.9
[1.0.0-alpha.8]: https://github.com/jcornaz/beancount_parser_2/compare/v1.0.0-alpha.7...v1.0.0-alpha.8
[1.0.0-alpha.7]: https://github.com/jcornaz/beancount_parser_2/compare/v1.0.0-alpha.6...v1.0.0-alpha.7
[1.0.0-alpha.6]: https://github.com/jcornaz/beancount_parser_2/compare/v1.0.0-alpha.5...v1.0.0-alpha.6
[1.0.0-alpha.5]: https://github.com/jcornaz/beancount_parser_2/compare/v1.0.0-alpha.4...v1.0.0-alpha.5
[1.0.0-alpha.4]: https://github.com/jcornaz/beancount_parser_2/compare/v1.0.0-alpha.3...v1.0.0-alpha.4
[1.0.0-alpha.3]: https://github.com/jcornaz/beancount_parser_2/compare/v1.0.0-alpha.2...v1.0.0-alpha.3
[1.0.0-alpha.2]: https://github.com/jcornaz/beancount_parser_2/compare/v1.0.0-alpha.1...v1.0.0-alpha.2
[1.0.0-alpha.1]: https://github.com/jcornaz/beancount_parser_2/compare/v1.0.0-alpha.0...v1.0.0-alpha.1
[1.0.0-alpha.0]: https://github.com/jcornaz/beancount_parser_2/compare/beancount-parser-v1.16.0...v1.0.0-alpha.0

