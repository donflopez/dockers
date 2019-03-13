# Dockers

> A rust docker library.

The main difference with other docker libs is that I didn't see the need of making this async (and use futures) for my use case so this is a plain sync method wrapper around the docker api. You can make sync work with this and, if needed, implement async code on top of it, but for that, you probably prefer other options out there.

## Example


```rust
extern dockers;

use dockers::Container;
use dockers::Image;

fn main () {
    let img = Image::pull("debian".to_owned(), None)
        .expect("Cannot pull image");

    let cont = Container::new(None, Some("debian".to_owned()))
        .create(Some("my_debian_cont_name".to_owned(), None))
        .expect("Cannot create container");

    // Do your things...

    cont.remove();
    img.remove();
}
```

## Roadmap
- [ ] Really simple api for common use cases
- [ ] Cover complex use cases with custom methods
- [ ] Allow configurations ala docker compose