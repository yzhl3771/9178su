# 9178su

9178su 是一个 Android 内核级 Root 方案（上游为内核辅助 Root 开源项目），
已完成品牌与运行时身份改名。

## 改名说明

由于 Android 包名、Java/Kotlin/Rust/C 标识符、URI scheme 均不能以数字开头，
本仓库的命名规则如下：

| 场景 | 命名 |
| --- | --- |
| 产品名、应用名、菜单文案、文档标题 | `9178su` |
| Android 应用包名（applicationId） | `com.kxqw.hzvd` |
| Manager 类包（源码 namespace） | 保持 `me.weishu.kernelsu`（内部类名，不影响使用，可后续深改） |
| 内核模块 / `9178su.ko` / Kconfig | `9178su` / `CONFIG_N9178SU` |
| 守护进程与相关产物 | `n9178sud`（文件/进程），数据目录 `/data/adb/9178su` |
| 代码内部前缀（必须为合法标识符） | `n9178su` / `N9178SU` |

> 已改动的运行时可见位置：Manager 应用名与包名、UI 文案、日志 TAG、
> deep-link scheme `n9178su://`、守护进程名、`/data/adb` 与 `/metadata` 路径、
> SELinux domain/type（`n9178su` / `n9178su_file`）、内核模块名与配置项、
> CI/打包脚本、文档与网站文案。

## 仓库地址

`kernel/setup.sh` 默认从以下仓库拉取源码，上传前请改成你自己的仓库，或保留占位仓库：

```sh
https://github.com/9178su/9178su.git
```

## 如何让 Root Manager 正常工作

Manager 与内核模块通过 **APK 签名哈希**互相识别，不能用官方 KernelSU 的
Manager APK 配自定义内核，也不能用未带对应签名的 APK。

1. 用自己的 keystore 构建 Manager：

```sh
cd manager
./gradlew assembleRelease \
  -PN9178SU_PACKAGE_NAME=com.kxqw.hzvd \
  -PN9178SU_NAME=9178su \
  -PKEYSTORE_FILE=/path/to/your.keystore \
  -PKEYSTORE_PASSWORD=xxx \
  -PKEY_ALIAS=xxx \
  -PKEY_PASSWORD=xxx
```

2. 从签名证书计算内核要校验的哈希，并把以下参数传给内核模块构建
   （`N9178SU_EXPECTED_SIZE` 为十六进制字节数，`N9178SU_EXPECTED_HASH` 为
   SHA-256，构建日志会打印当前值）：

```sh
make CONFIG_N9178SU=m \
  N9178SU_MANAGER_PACKAGE=com.kxqw.hzvd \
  N9178SU_EXPECTED_SIZE=0x... \
   N9178SU_EXPECTED_HASH=...
```

3. 将构建得到的 `9178su.ko` / Manager APK 按官方 KernelSU 的 GKI 修补流程使用；
   LKM 模式下 n9178sud 会在启动时负责加载 `9178su.ko`。

### 仓库自带开发签名

为了方便首次测试，`kernel/Kbuild` 的默认 `N9178SU_EXPECTED_SIZE/HASH`
已指向随附的开发 keystore（见交付目录 `9178su-signing/`）。
只要使用该 keystore 的 secrets 构建 Manager，Actions 产出的 `9178su.ko`
即可直接配对使用。

正式对外发布前请**务必更换为自己生成的 keystore**，然后运行：

```sh
keytool -exportcert -alias <alias> -keystore <keystore> -storepass <pass> -file cert.der
scripts/update-expected-signature.sh cert.der
```

并同步更新 GitHub 仓库 secrets。

## 非 GKI / 老内核

当前上游只支持 GKI 2.0（kernel 5.10+）。本分支同步自上游 main，未改动内核
兼容性逻辑；如需旧版（v0.9.x）请自行基于对应 tag 操作。

## License

与上游一致：`kernel/` 为 GPL-2.0-only，其余为 GPL-3.0-or-later。
