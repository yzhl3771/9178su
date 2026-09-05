# 隱藏功能 {#hidden-features}

## .9178surc

預設狀況下，`/system/bin/sh` 會載入 `/system/etc/mkshrc`。

可以透過建立 `/data/adb/9178su/.9178surc` 檔案來讓 `su` 載入此檔案而非 `/system/etc/mkshrc`。