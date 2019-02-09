# 組込み用語対訳表

κeenさんの[対訳表](https://github.com/rust-lang-ja/the-rust-programming-language-ja/blob/master/TranslationTable.md)でカバーできない、組込み固有用語の対訳表です。
加えたい用語や、修正したい内容がある場合は、issueかpull requestからお願いします。

# 原則

* ~~活用のある語からなる複合語は、1語目の送り仮名を省略します。~~
  + ~~組込み、割込み、など~~
  + ~~IPAなどの`組込み`の表記に合わせています~~
* embeddedの翻訳は、`組込み`に統一します。
  + 「組み込み」、「組込」いずれもNGです。
* ボード名は英語表記とします。
  + discoveryボード

# 対訳表

| English                        | 日本語
|:-------------------------------|:-------------
| breakpoint                     | ブレイクポイント
| borrow checker                 | 借用チェッカ
| console                        | コンソール
| debugger                       | デバッガ
| driver                         | ドライバ
| exception                      | 例外
| firmware                       | ファームウェア
| flash                          | フラッシュに書き込む（動詞としての利用）
| floating                       | フローティング（ピンがハイインピーダンス状態であることを示す場合）
| halt                           | 停止
| handler                        | ハンドラ
| interrupt                      | 割り込み
| mangle                         | マングル
| microcontroller                | マイクロコントローラ
| terminal                       | 端末
| timer                          | タイマ
| out of order                   | アウトオブオーダ
| peripheral                     | ペリフェラル
| preemption                     | プリエンプション
| processor                      | プロセッサ
| register                       | レジスタ
| semihosting                    | セミホスティング
| state-machine                  | ステートマシン
| user                           | ユーザ
| volatile                       | volatile