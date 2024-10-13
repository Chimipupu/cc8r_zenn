# CC8R（ちみ's CPU 8bit RISC）
Zennで記載した内容のRust実装です！
テストで **`(5+3)x2 = 16`をCPUで計算** させています🥳

🔗Zenn「RustでCPUを自作して動くまで📝」
- https://zenn.dev/chimipupu/articles/e0af6451e0cab9


## 特徴
- アーキテクチャ ... 8bit
- メモリ空間 ... 256Byte
- 汎用レジスタ ... 8本(R0はアキュムレータ)
- フラグレジスタ ... ゼロ、キャリー、オーバーフロー、ネガティブ
- 命令セット ... 18（転送、算術論理演算、ジャンプ命令）

## 命令セット

- `LDI`: レジスタに即値をロード
- `MV`: レジスタ間のデータ転送
- `ADD`, `SUB`, `MUL`, `DIV`: 四則演算
- `AND`, `OR`, `XOR`: 論理演算
- `SHL`, `SHR`: シフト操作
- `PUSH`, `POP`: スタック操作
- `JMP`, `JZ`, `JNZ`: ジャンプ命令
- `HALT`: 実行停止
- `NOP`: なにもしない

## アセンブラ

命令セットを元に、**CC8Rで`(5+3)x2`をCPUで計算させる**アセンブラです🥳

```asm
ORG 0x0000    ; プログラムの開始アドレスを0x0000に設定

LDI R1, 5     ; R1に5をロード
LDI R2, 3     ; R2に3をロード
LDI R3, 2     ; R3に2をロード
ADD R1, R2    ; R1とR2を加算
MV R4, R0     ; R0の値をR4に移動
MUL R3, R4    ; R3とR4を掛け算
HALT           ; プログラムを終了s
```

## 機械語
アセンブラを機械語にしたものです🥳

```hex
0x14 0x01 0x05 // LDI R1, 5
0x14 0x02 0x03 // LDI R2, 3
0x14 0x03 0x02 // LDI R3, 2
0x20 0x01 0x02 // ADD R1, R2
0x18 0x04 0x00 // MV R4, R0
0x40 0x03 0x04 // MUL R3, R4
0x10           // HALT
```

# テスト
CPUに`(5+3)x2 `をさせるRustのテストコードです🛠️

```rust
#[cfg(test)]
mod tests {
    use super::*; // 現在のモジュールをインポート

    #[test]
    fn test_cpu_program() {
        let mut cpu = CC8R::new();

        // (5+3)x2 のプログラム
        let program = [
            0x14, 0x01, 0x05, // LDI R1, 5
            0x14, 0x02, 0x03, // LDI R2, 3
            0x14, 0x03, 0x02, // LDI R3, 2
            0x20, 0x01, 0x02, // ADD R1, R2
            0x18, 0x04, 0x00, // MV R4, R0
            0x40, 0x03, 0x04, // MUL R3, R4
            0x10,             // HALT
        ];

        cpu.load_program(&program);
        cpu.run();
        println!("{}", cpu);

        assert_eq!(cpu.registers[0], 16);
    }
}
```

## 期待値
テストコードで出力される文字と期待値です🥳
自作CPUで`(5+3)x2=16`を計算できていると**R0が16**になる
- 期待値: R0 ... 16

```shell
LDI R1, 5
LDI R2, 3
LDI R3, 2
ADD R1, R2
MV R4, R0
MUL R3, R4
HALT
Register: [16, 5, 3, 2, 8, 0, 0, 0]
Flag: 0x00
SP: 0xF0
PC: 0x13
```

**RustRover**で期待値で出力されている画面🥳
![](https://storage.googleapis.com/zenn-user-upload/2cf53e738457-20241014.png)
