# Reverse a string

Alright, next let's make the server more interesting by having it respond to the client with the
reverse of the text that they sent. The server will respond to the client every time they press the
ENTER key. Each server response will be in a new line.

This time you'll need a buffer; you can use [`heapless::Vec`]. Here's the starter code:

[`heapless::Vec`]: https://docs.rs/heapless/latest/heapless/struct.Vec.html

``` rust
{{#include examples/reverse-string.rs}}
```
