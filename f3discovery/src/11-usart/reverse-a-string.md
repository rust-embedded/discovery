<!-- # Reverse a string -->

# 文字列の反転

<!-- 
Alright, next let's make the server more interesting by having it respond to the client with the
reverse of the text that they sent. The server will respond to the client every time they press the
ENTER key. Each server response will be in a new line.
 -->

それでは、次はサーバーをもっとおもしろくしましょう。送信されたデータを反転したテキストをクライアントに返信します。
サーバーは、ENTERキーが押されるたびに、クライアントに返信します。
サーバーの各返信は、新しい行になります。

<!-- This time you'll need a buffer; you can use [`heapless::Vec`]. Here's the starter code: -->

今回は、バッファが必要になります。[`heapless::Vec`]が使えます。スターターコードは次の通りです。

[`heapless::Vec`]: https://docs.rs/heapless/0.2.1/heapless/struct.Vec.html

``` rust
#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux11::{entry, iprint, iprintln};
use heapless::{consts, Vec};

#[entry]
fn main() -> ! {
    let (usart1, mono_timer, itm) = aux11::init();

    // 32バイト容量のバッファ
    let mut buffer: Vec<u8, consts::U32> = Vec::new();

    loop {
        buffer.clear();

        // TODO ユーザーリクエストを受信します。各ユーザーリクエストはENTERで終わります。
        // 注記　`buffer.push`は、`Result`を返します。
        // エラーメッセージを返信することで、エラーを処理して下さい。

        // TODO 反転した文字列を送り返します
    }
}
```
