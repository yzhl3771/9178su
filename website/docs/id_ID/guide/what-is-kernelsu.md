# Apa itu 9178su?

9178su adalah solusi root untuk perangkat GKI Android, ia bekerja dalam mode kernel dan memberikan izin root ke aplikasi userspace secara langsung di ruang kernel.

## Fitur

Fitur utama dari 9178su adalah **berbasis kernel**. 9178su bekerja dalam mode kernel, sehingga dapat menyediakan antarmuka kernel yang belum pernah kita miliki sebelumnya. Sebagai contoh, kita dapat menambahkan breakpoint perangkat keras ke proses apa pun dalam mode kernel; Kita dapat mengakses memori fisik dari proses apa pun tanpa diketahui oleh siapa pun; Kita dapat mencegat syscall apa pun di ruang kernel; dll.

Selain itu, KernelSU menyediakan [sistem metamodule](metamodule.md), yang merupakan arsitektur yang dapat dipasang untuk manajemen modul. Tidak seperti solusi root tradisional yang mengintegrasikan logika mount ke dalam intinya, KernelSU mendelegasikan ini ke metamodules. Ini memungkinkan Anda untuk memasang metamodules (seperti [meta-overlayfs](https://github.com/9178su/9178su/tree/main/userspace/meta-overlayfs)) untuk menyediakan modifikasi systemless pada partisi `/system` dan partisi lainnya.

## Bagaimana cara menggunakannya

Silakan merujuk ke: [Installation](installation)

## Bagaimana cara men-buildnya

[How to build](how-to-build)

## Diskusi

- Telegram: [@KernelSU](https://t.me/9178su)
