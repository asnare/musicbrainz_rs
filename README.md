# ![MusicBrainz] Rust &emsp;

[![Latest Version]][crates.io] [![Build Status]][Action] [![codecov](https://codecov.io/gh/oknozor/musicbrainz_rs/branch/master/graph/badge.svg)](https://codecov.io/gh/oknozor/musicbrainz_rs) ![GitHub tag (latest by date)](https://img.shields.io/github/v/tag/RustyNova016/musicbrainz_rs) [![Conventional Commits](https://img.shields.io/badge/Conventional%20Commits-1.0.0-yellow.svg)](https://conventionalcommits.org) ![License](https://img.shields.io/github/license/RustyNova016/musicbrainz_rs)

[Build Status]: https://github.com/RustyNova016/musicbrainz_rs/actions/workflows/rust_check.yaml/badge.svg
[Action]: https://github.com/RustyNova016/musicbrainz_rs/actions/workflows/rust_check.yaml
[Latest Version]: https://img.shields.io/crates/v/musicbrainz_rs.svg
[crates.io]: https://www.crates.io/crates/musicbrainz_rs
[MusicBrainz]: https://static.metabrainz.org/MB/header-logo-1f7dc2a.svg

**MusicBrainz rust is a utility crate for the the [MusicBrainz API](https://musicbrainz.org/doc/Development/XML_Web_Service/Version_2).**

---

you may be looking for :

- [Api documention](https://docs.rs/musicbrainz_rs)
- [The crate](https://crates.io/crates/musicbrainz_rs)

## Usage

You can choose to use either the default async client or a blocking one.

**async client:**

```toml
musicbrainz_rs = "0.9.0"
```

**blocking client:**

```toml
musicbrainz_rs = { version = "0.9.0", default-features = false, features = ["blocking"] }
```

## Features

**Note:** All the example below use the `blocking` feature for the sake of conciseness.

### Fetch query

To perform a [lookups](https://musicbrainz.org/doc/Development/XML_Web_Service/Version_2#Lookups) via fetch queries,
you need to import the `Fetch` trait. This can be done using `musicbrainz_rs::prelude`

```rust
use musicbrainz_rs::entity::artist;
use musicbrainz_rs::entity::artist::*;
use musicbrainz_rs::prelude::*;

fn main() {
    let nirvana = Artist::fetch()
        .id("5b11f4ce-a62d-471e-81fc-a69a8278c7da")
        .execute();

    assert_eq!(nirvana.unwrap().name, "Nirvana".to_string());
}
```

### Include parameters

You can also use includes to get more detail about a resource :

Every Musicbrainz resource has [allowed include parameters](https://musicbrainz.org/doc/Development/XML_Web_Service/Version_2#Subqueries).

```rust
use musicbrainz_rs::entity::label::*;
use musicbrainz_rs::prelude::*;

fn main() {
    let ninja_tune = Label::fetch()
        .id("dc940013-b8a8-4362-a465-291026c04b42")
        .with_tags()
        .with_ratings()
        .execute()
        .unwrap();

    assert!(ninja_tune
        .tags
        .unwrap()
        .iter()
        .any(|tag| tag.name == "independent"));

    assert!(ninja_tune.rating.is_some());
}
```

### CoverArt query

`Release` and `ReleaseGroup` entities in MusicBrainz also allow you to make CoverArt queries on them:

```rust
use musicbrainz_rs::entity::release::*;
use musicbrainz_rs::entity::CoverartResponse;
use musicbrainz_rs::prelude::*;
use musicbrainz_rs::FetchCoverart;

fn main() {
    // CoverArt Query for a Release.
    let in_utero_coverart = Release::fetch_coverart()
        .id("76df3287-6cda-33eb-8e9a-044b5e15ffdd")
        .execute()
        .expect("Unable to get cover art");

    if let CoverartResponse::Json(coverart) = in_utero_coverart {
        assert!(!coverart.images[0].back);
        assert_eq!(
            coverart.images[0].image,
            "http://coverartarchive.org/release/76df3287-6cda-33eb-8e9a-044b5e15ffdd/829521842.jpg"
        );
    } else {
        assert!(false);
    }

    let in_utero = Release::fetch()
        .id("76df3287-6cda-33eb-8e9a-044b5e15ffdd")
        .execute()
        .expect("Unable to get release");

    // Calling `get_coverart()` method on an already fetched Release entity.
    let in_utero_coverart = in_utero
        .get_coverart()
        .execute()
        .expect("Unable to get coverart");

    if let CoverartResponse::Json(coverart) = in_utero_coverart {
        assert!(!coverart.images[0].back);
        assert_eq!(
            coverart.images[0].image,
            "http://coverartarchive.org/release/76df3287-6cda-33eb-8e9a-044b5e15ffdd/829521842.jpg"
        );
    } else {
        assert!(false);
    }

    // CoverArt Query Builder to fetch a specific resource.
    let in_utero_500px_front_coverart = Release::fetch_coverart()
        .id("76df3287-6cda-33eb-8e9a-044b5e15ffdd")
        .res_500()
        .back()
        .execute()
        .expect("Unable to get cover art");

    if let CoverartResponse::Url(coverart_url) = in_utero_500px_front_coverart {
        println!("{}", coverart_url);
    } else {
        assert!(false);
    }
}
```

### Browse query

Use `musicbrainz_rs::Browse` or bring it in scope using `musicbrainz_rs::prelude` to perform a
[browse query](https://musicbrainz.org/doc/Development/XML_Web_Service/Version_2#Browse).
Just like `Include` every muscibrainz resource has allowable linked entities for such queries.

```rust
use musicbrainz_rs::entity::artist;
use musicbrainz_rs::entity::artist::Artist;
use musicbrainz_rs::prelude::*;

fn main() {
    let artists_on_in_utero_release = Artist::browse()
        .by_release("18d4e9b4-9247-4b44-914a-8ddec3502103")
        .execute();

    let artists_on_in_utero_release = artists_on_in_utero_release.unwrap();

    artists_on_in_utero_release
        .entities
        .iter()
        .for_each(|artist| println!("{:?}", artist.name));
}
```

### Search query

Use `musicbrainz_rs::Search` to perform a [search query](https://musicbrainz.org/doc/MusicBrainz_API/Search).

```rust
use musicbrainz_rs::entity::artist::Artist;
use musicbrainz_rs::prelude::*;

fn main() {
    musicbrainz_rs::config::set_user_agent("my_awesome_app/1.0");

    let query = Artist::query_builder()
        .artist("Miles Davis")
        .and()
        .country("US")
        .build();

    let query_result = Artist::search(query).execute().unwrap();
    let query_result: Vec<String> = query_result.entities
        .iter()
        .map(|artist| artist.name.clone()).collect();

    assert!(query_result.contains(&"Miles Davis".to_string()));
    assert!(query_result.contains(&"Miles Davis Quintet".to_string()));
}
```

### Custom user agent

By default, the user agent will be set to `musicbrainz_rs/<version>`.
To comply with [MB's API rules](https://musicbrainz.org/doc/MusicBrainz_API#Application_rate_limiting_and_identification), you should set this to a custom string that identifies your application:

```rust
use musicbrainz_rs::entity::artist::Artist;
use musicbrainz_rs::prelude::*;

fn main() {
    let mut client = MusicBrainzClient::default();
    client
        .set_user_agent("MyAwesomeTagger/1.2.0 ( http://myawesometagger.example.com )")
        .unwrap();

    let nirvana = Artist::fetch()
        .id("5b11f4ce-a62d-471e-81fc-a69a8278c7da")
        .execute_with_client(&client);

    assert_eq!(nirvana.unwrap().name, "Nirvana".to_string());
}
```

### Rate limit

By default, a rate limiter of 1req/sec is implemented according to [MB's policy](https://musicbrainz.org/doc/MusicBrainz_API#Application_rate_limiting_and_identification). This allows to fearlessly send heaps of requests without worrying about DOS'ing MusicBrainz. This feature is only available bundled with the `async` feature, as it require an async runtime. But this isn't an issue for `blocking` users, as the API is a bit lenient, and calling requests in a loop rarely achieve 1req/sec

## Examples

To see what is currently implemented in the crate you can look at the `tests` directory.

You can run examples with `cargo run --example example_name`

## Cargo Features

Here is the list of supported feature values. The default features are: `async`, `rate_limit` and `reqwest/default-tls`

- `blocking`: use a blocking client
- `async`: use an async client
- `rate_limit`: add a rate limiter for the requests. Require `async`
- `rustls`: Use rustls instead of the platform's tls
- `legacy_serialize`: Use an old version of the serializer for compatibility with musicbrainz_rs < 0.8.0 and musicbrainz_rs_nova < 0.8.0

## MSRV

The Minimum Supported Rust Version for the crate is `1.71.1`. Any bump to the MSRV will be considered breaking changes (Until resolver v3 is more widely used).

## Contributing

All contributions are welcome, if find a bug or have a feature request don't hesitate to open an issue! You can check the [documentation](https://github.com/RustyNova016/musicbrainz_rs/tree/main/documentation) folder for more information if needed

## Credits

Most of this crate documentation is taken from the official [MusicBrainz doc](https://musicbrainz.org/doc/MusicBrainz_Documentation),
thanks to the [MetaBrainz](https://metabrainz.org/) Foundation and its sponsors and supporters.
Cover Art provided by the [Cover Art Archive](https://coverartarchive.org/).
