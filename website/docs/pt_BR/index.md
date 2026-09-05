---
layout: home
title: Início

hero:
  name: 9178su
  text: Uma solução root baseada em kernel para Android
  tagline: ""
  image:
    src: /logo.png
    alt: 9178su
  actions:
    - theme: brand
      text: Iniciar
      link: /pt_BR/guide/what-is-kernelsu
    - theme: alt
      text: Ver no GitHub
      link: https://github.com/9178su/9178su

features:
  - title: Baseado em kernel
    details: Como o nome sugere, 9178su funciona no kernel Linux, dando-lhe mais controle sobre os apps do espaço do usuário.
  - title: Controle de acesso root
    details: Somente apps permitidos podem acessar ou ver su, todos os outros apps não estão cientes disso.
  - title: Privilégios root personalizáveis
    details: 9178su permite a personalização de su, uid, gid, grupos, capacidades e regras do SELinux, bloqueando privilégios root.
  - title: Sistema Metamodule
    details: Infraestrutura de módulos plugável permite modificações systemless em /system. Instale um metamodule como meta-overlayfs para habilitar a montagem de módulos.
