# LSM303DLHC

<!-- 
Two of the sensors in the F3, the magnetometer and the accelerometer, are packaged in a single
component: the LSM303DLHC integrated circuit. These two sensors can be accessed via an I2C bus. Each
sensor behaves like an I2C slave and has a *different* address.
 -->

F3にある磁力計と加速度計の2つのセンサは、1つのコンポーネントにまとめられています。このコンポーネントは、LSM303DLHCという集積回路です。
これら2つのセンサは、I2Cバスでアクセスできます。各センサは*異なる*アドレスを持つI2Cスレーブとして動作します。

<!-- 
Each sensor has its own memory where it stores the results of sensing its environment. Our
interaction with these sensors will mainly involve reading their memory.
 -->

環境をセンシングした結果を格納するため、各センサは自身のメモリを持っています。
これらのセンサとのやり取りは、主にこれらのメモリから読み込むことです。

<!-- 
The memory of these sensors is modeled as byte addressable registers. These sensors can be
configured too; that's done by writing to their registers. So, in a sense, these sensors are very
similar to the peripherals *inside* the microcontroller. The difference is that their registers are
not mapped into the microcontrollers' memory. Instead, their registers have to be accessed via the
I2C bus.
 -->

これらセンサのメモリは、バイト単位でアドレスが割り当てられたレジスタとして作られています。センサは設定することもできます。
センサのレジスタに書き込むことで設定ができます。そのため、ある意味で、これらのセンサはマイクロコントローラ*内の*ペリフェラルに似ています。
異なる点は、これらセンサのレジスタがマイクロコントローラのメモリにマッピングされていないことです。
代わりに、I2Cバスでアクセスしなければなりません。

<!-- 
The main source of information about the LSM303DLHC is its [Data Sheet]. Read through it to see how
one can read the sensors' registers. That part is in:
 -->

LSM303DLHCに関する主な情報源は、[データシート]です。どうすればセンサのレジスタを読めるか、を知るために読んでみます。
その情報がある部分は、次のセクションです。

<!-- [Data Sheet]: http://www.st.com/resource/en/datasheet/lsm303dlhc.pdf -->

[データシート]: http://www.st.com/resource/en/datasheet/lsm303dlhc.pdf

> Section 5.1.1 I2C Operation - Page 20 - LSM303DLHC Data Sheet

<!-- 
The other part of the documentation relevant to this book is the description of the registers. That
part is in:
 -->

この本に関連するドキュメンテーションがある他の部分は、レジスタの記述です。
その情報がある部分は、次のセクションです。

> Section 7 Register description - Page 25 - LSM303DLHC Data Sheet
