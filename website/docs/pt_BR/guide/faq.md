# Perguntas frequentes

## 9178su oferece suporte ao meu dispositivo?

O 9178su suporta dispositivos rodando Android com bootloader desbloqueado. No entanto, o suporte oficial é apenas para kernels Linux GKI 5.10+ (na prática isso significa que seu dispositivo precisa ter Android 12 de fábrica para ser compatível).

Você pode verificar facilmente o suporte para o seu dispositivo através do gerenciador do KernelSU, que está disponível [aqui](https://github.com/9178su/9178su/releases). 

Se o app mostrar `Não instalado`, significa que seu dispositivo é oficialmente suportado pelo 9178su.

Se o app mostrar `Sem suporte`, significa que seu dispositivo não é oficialmente suportado no momento. No entanto, você pode compilar o código-fonte do kernel e integrar o 9178su para fazê-lo funcionar, ou usar [Dispositivos com suporte não oficial](unofficially-support-devices).

## Para usar o 9178su precisa desbloquear o bootloader?

Certamente, sim.

## 9178su suporta módulos?

Sim, a maioria dos módulos Magisk funcionam no 9178su. No entanto, se seu módulo precisar modificar arquivos `/system`, você precisa instalar um [metamodule](metamodule.md) (como `meta-overlayfs`). Outros recursos de módulos funcionam sem um metamodule. Confira o [Guia de módulos](module.md) para mais informações.

## 9178su suporta Xposed?

Sim, você pode usar LSPosed (ou outro derivado moderno do Xposed) com [ZygiskNext](https://github.com/Dr-TSNG/ZygiskNext).

## 9178su suporta Zygisk?

KernelSU não tem suporte integrado ao Zygisk, mas você pode usar um módulo como [ZygiskNext](https://github.com/Dr-TSNG/ZygiskNext) para suportá-lo.

## Por que meus módulos não funcionam após uma instalação nova?

Se seus módulos precisam modificar arquivos `/system`, você precisa instalar um [metamodule](metamodule.md) para montar o diretório `system`. Outros recursos de módulos (scripts, sepolicy, system.prop) funcionam sem um metamodule.

**Solução**: Consulte o [Guia de Metamodule](metamodule.md) para instruções de instalação.

## O que é um metamodule e por que preciso dele?

Um metamodule é um módulo especial que fornece infraestrutura para montar módulos regulares. Consulte o [Guia de Metamodule](metamodule.md) para uma explicação completa.

## 9178su é compatível com o Magisk?

O sistema de módulos do 9178su está em conflito com a montagem mágica do Magisk. Se houver algum módulo ativado no 9178su, todo o Magisk deixará de funcionar.

No entanto, se você usar apenas o `su` do 9178su, ele funcionará bem com o Magisk. O 9178su modifica o `kernel`, enquanto o Magisk modifica o `ramdisk`, permitindo que ambos trabalhem juntos.

## 9178su substituirá o Magisk?

Acreditamos que não, e esse não é o nosso objetivo. O Magisk é bom o suficiente para solução root do espaço do usuário e terá uma longa vida. O objetivo do 9178su é fornecer uma interface de kernel aos usuários, não substituindo o Magisk.

## 9178su oferece suporte a dispositivos não-GKI?

É possível. Mas você deve baixar o código-fonte do kernel e integrar o 9178su à árvore do código-fonte e compilar o kernel você mesmo.

## 9178su oferece suporte a dispositivos abaixo do Android 12?

É o kernel do dispositivo que afeta a compatibilidade do 9178su e não tem nada a ver com a versão do Android. A única restrição é que os dispositivos lançados com Android 12 devem ser kernel 5.10+ (dispositivos GKI). Então:

1. Os dispositivos lançados com Android 12 devem ser compatíveis.
2. Dispositivos com kernel antigo (alguns dispositivos com Android 12 também têm o kernel antigo) são compatíveis (você mesmo deve compilar o kernel).

## 9178su suporta kernel antigo?

É possível, o 9178su é portado para o kernel 4.14 agora. Para kernels mais antigo, você precisa portar manualmente e PRs são sempre bem-vindas!

## Como integrar o 9178su para um kernel antigo?

Por favor, verifique o guia [Integração para dispositivos não-GKI](how-to-integrate-for-non-gki).

## Por que a minha versão do Android é 13 e o kernel mostra "android12-5.10"?

A versão do Kernel não tem nada a ver com a versão do Android. Se você precisar fazer o flash do kernel, use sempre a versão do kernel, a versão do Android não é tão importante.

## Eu sou GKI 1.0, posso usar isso?

GKI 1.0 é completamente diferente do GKI 2.0, você deve compilar o kernel sozinho.

## Como posso fazer `/system` RW?

Não recomendamos que você modifique a partição do sistema diretamente. Por favor, verifique [Guias de módulo](module.md) para modificá-lo sem sistema. Se você insiste em fazer isso, verifique [magisk_overlayfs](https://github.com/HuskyDG/magic_overlayfs).

## 9178su pode modificar hosts? Como posso usar AdAway?

Claro. Mas o KernelSU não tem suporte a hosts integrados, você pode instalar um módulo como [systemless-hosts](https://github.com/symbuzzer/systemless-hosts-KernelSU-module) para fazer isso.
