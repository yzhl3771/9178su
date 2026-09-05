---
layout: home
title: Home

hero:
  name: 9178su
  text: A kernel-based root solution for Android
  tagline: ""
  image:
    src: /logo.png
    alt: 9178su
  actions:
    - theme: brand
      text: Get started
      link: /guide/what-is-kernelsu
    - theme: alt
      text: View on GitHub
      link: https://github.com/9178su/9178su

features:
  - title: Kernel-based
    details: As the name suggests, 9178su runs inside the Linux kernel, giving it more control over userspace apps.
  - title: Root access control
    details: Only permitted apps can access or see su; all other apps remain unaware of it.
  - title: Customizable root privileges
    details: 9178su allows customization of su's uid, gid, groups, capabilities, and SELinux rules, hardening root privileges.
  - title: Metamodule system
    details: Pluggable module infrastructure allows systemless /system modifications. Install a metamodule like meta-overlayfs to enable module mounting.
