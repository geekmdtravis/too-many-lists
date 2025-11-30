pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

// Stopped on 2.2 https://rust-unofficial.github.io/too-many-lists/first-new.html
