# カーネル解析

## 重要ファイル

- カーネル用のリンカスクリプトは`kernel/src/arch/aarch64/kernel.ld`
- 開始コードは`kernel/src/arch/aarch64/start.s`
    - コア0とそれ以外で処理を分けているが、EL1に移行し、MMUを有効にして
      Rustの各々の開始コードにジャンプしている
- `start.s`からジャンプするのは
    - コア0は`kernel/src/lib/lib.rs#boot_core_start()`
    - それ以外はコア0は`kernel/src/lib/lib.rs#non_boot_core_start()`
- Rustの開始コードでは
    - コア0は`register_devices()` (config/raspberrypi3/src/main.rs)を実行して、初期化済みフラグをオンに、`start_multitasking()`(kernel/src/arch/aarch64/context.rs)を実行
    - 祖霊母初期化済みフラグが音になるまで待機して`arch::loop_forever()`を実行（現在は`wfe`しているだけ）

## 物理メモリマップ

```bash
0x0000_0000
0x0002_0000     core1 のスタックポインタ
0x0004_0000     core2 のスタックポインタ
0x0006_0000     core3 のスタックポインタ
0x0008_0000     カーネル開始アドレス
....
0x????_???0     カーネル終了アドレス (__KERNEL_END_ADDR)
0x??10_0000     コア0のスタックポイント (1MB)
```

## 仮想アドレスマップ

```bash
0x0xffff_0000_0000_0000     カーネルベースアドレス
0x0xffff_0000_0008_0000     カーネルの開始アドレス
```
