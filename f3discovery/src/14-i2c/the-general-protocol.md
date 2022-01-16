<!-- # General protocol -->

# 一般的なプロトコル

<!-- 
The I2C protocol is more elaborated than the serial communication protocol because it has to support
communication between several devices. Let's see how it works using examples:
 -->

I2Cプロトコルは複数のデバイス間の通信をサポートしなければならないため、シリアル通信プロトコルより複雑です。
例を使って、I2Cプロトコルがどのように動くか見ていきましょう。

<!-- ## Master -> Slave -->

## マスター -> スレーブ

<!-- If the master wants to send data to the slave: -->

マスターがスレーブにデータを送りたい場合、次のようになります。

<p align="center">
  <img class="white_bg" height=180 title="I2C bus" src="https://upload.wikimedia.org/wikipedia/commons/3/3e/I2C.svg">
</p>

<!-- 
1. Master: Broadcast START
2. M: Broadcast slave address (7 bits) + the R/W (8th) bit set to WRITE
3. Slave: Responds ACK (ACKnowledgement)
4. M: Send one byte
5. S: Responds ACK
6. Repeat steps 4 and 5 zero or more times
7. M: Broadcast STOP OR (broadcast RESTART and go back to (2))
 -->

1. マスター：STARTをブロードキャストします
2. M：スレーブアドレス（7ビット）＋R/W（第8の）ビットをWRITEに設定したものをブロードキャストします
3. スレーブ：ACK（ACKnowledgement）を返信します
4. M：1バイト送ります
5. S：ACKを返信します
6. 手順4と5を0回以上繰り返します
7. M：STOPブロードキャストします（または、RESTARTをブロードキャストし、手順2に戻ります）

<!-- 
> **NOTE** The slave address could have been 10 bits instead of 7 bits long. Nothing else would have
> changed.
 -->

> **注記** スレーブアドレスは7ビット長の代わりに10ビット長になる可能性があります。他には何も変わりません。

<!-- ## Master <- Slave -->

## マスター <- スレーブ

<!-- If the master wants to read data from the slave: -->

マスターがスレーブからデータを読みたい場合、次のようになります。

<p align="center">
<img class="white_bg" height=180 title="I2C bus" src="https://upload.wikimedia.org/wikipedia/commons/3/3e/I2C.svg">
</p>

<!-- 
1. M: Broadcast START
2. M: Broadcast slave address (7 bits) + the R/W (8th) bit set to READ
3. S: Responds with ACK
4. S: Send byte
5. M: Responds with ACK
6. Repeat steps 4 and 5 zero or more times
7. M: Broadcast STOP OR (broadcast RESTART and go back to (2))
 -->

1. M：STARTをブロードキャストします
2. M：スレーブアドレス（7ビット）＋R/W（第8の）ビットをREADに設定したものをブロードキャストします
3. S：ACKを返信します
4. S：1バイト送ります
5. M：ACKを返信します
6. 手順4と5を0回以上繰り返します
7. M：M：STOPブロードキャストします（または、RESTARTをブロードキャストし、手順2に戻ります）

<!-- 
> **NOTE** The slave address could have been 10 bits instead of 7 bits long. Nothing else would have
> changed.
 -->

> **注記** スレーブアドレスは7ビット長の代わりに10ビット長になる可能性があります。他には何も変わりません。
