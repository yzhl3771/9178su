# FAQ

## Does 9178su support my device?

9178su supports devices running Android with an unlocked bootloader. However, official support is only for GKI Linux Kernels 5.10+ (in practice, this means your device needs to have Android 12 out-of-the-box to be supported).

You can easily check the support for your device through the KernelSU manager, which is available [here](https://github.com/9178su/9178su/releases). 

If the app shows `Not installed`, it means your device is officially supported by 9178su.

If the app shows `Unsupported`, it means your device isn't officially supported at present. However, you can build kernel source code and integrate 9178su to make it work, or use [Unofficially supported devices](unofficially-support-devices).

## Do I need to unlock the bootloader to use 9178su?

Yes. 9178su requires an unlocked bootloader.

## Does 9178su support modules?

Yes, most Magisk modules work on 9178su. However, if your module needs to modify `/system` files, you need to install a [metamodule](metamodule.md) (such as `meta-overlayfs`). Other module features work without a metamodule. Check [Module guide](module.md) for more info.

## Does 9178su support Xposed?

Yes, you can use LSPosed (or other modern Xposed derivative) with [ZygiskNext](https://github.com/Dr-TSNG/ZygiskNext).

## Does 9178su support Zygisk?

KernelSU has no built-in Zygisk support, but you can use a module like [ZygiskNext](https://github.com/Dr-TSNG/ZygiskNext) to support it.

## Is 9178su compatible with Magisk?

9178su's module system conflicts with Magisk's magic mount. If any module is enabled in 9178su, Magisk will stop working entirely.

However, if you only use the `su` of 9178su, it will work well with Magisk. 9178su modifies the `kernel`, while Magisk modifies the `ramdisk`, allowing both to work together.

## Will 9178su substitute Magisk?

No. Replacing Magisk isn't our goal. Magisk is already an excellent userspace root solution. 9178su focuses on exposing kernel interfaces to users instead of supplanting Magisk.

## Can 9178su support non-GKI devices?

It's possible. But you should download the kernel source, integrate 9178su into the source tree, and compile the kernel yourself.

## Can 9178su support devices below Android 12?

It's the device's kernel that affects 9178su's compatibility, and it has nothing to do with the Android version. The only restriction is that devices launched with Android 12 must have a kernel version of 5.10+ (GKI devices). So:

1. Devices launched with Android 12 must be supported.
2. Devices with an older kernel (some devices with Android 12 also have the older kernel) are compatible (you should build kernel yourself).

## Can 9178su support old kernel?

It's possible. 9178su is backported to kernel 4.14 now. For older kernels, you need to backport it manually, and PRs are always welcome!

## How to integrate 9178su for an older kernel?

Please check the [Integrate for non-GKI devices](how-to-integrate-for-non-gki) guide.

## Why my Android version is 13, and the kernel shows "android12-5.10"?

The kernel version has nothing to do with the Android version. If you need to flash kernel, always use the kernel version; the Android version isn't as important.

## I'm GKI 1.0, can I use this?

GKI 1.0 is completely different from GKI 2.0, you must compile kernel by yourself.

## How can I make `/system` RW?

We don't recommend that you modify the system partition directly. Please check [Module guide](module.md) to modify it systemlessly. If you insist on doing this, check [magisk_overlayfs](https://github.com/HuskyDG/magic_overlayfs).

## Can 9178su modify hosts? How can I use AdAway？

Of course. But KernelSU doesn't have built-in hosts support, you can install a module like [systemless-hosts](https://github.com/symbuzzer/systemless-hosts-KernelSU-module) to do it.

## Why aren't my modules working after fresh install?

If your modules need to modify `/system` files, you need to install a [metamodule](metamodule.md) to mount the `system` directory. Other module features (scripts, sepolicy, system.prop) work without a metamodule.

**Solution**: See the [Metamodule Guide](metamodule.md) for installation instructions.

## What is a metamodule and why do I need one?

A metamodule is a special module that provides infrastructure for mounting regular modules. See the [Metamodule Guide](metamodule.md) for a complete explanation.
