# Notes on windows-rs etc

[Official DOCS](https://microsoft.github.io/windows-docs-rs/)

[Official FAQ](https://github.com/microsoft/windows-rs/blob/master/docs/FAQ.md)


Super useful table:

[by Wesleywiser in Issue](https://github.com/microsoft/windows-rs/issues/1647#issuecomment-1085005439)

[Interesting thread which I spammed "Support WinRT and COM class authoring"](https://github.com/microsoft/windows-rs/issues/1094)


## CoIncrementMTAUsage vs CoInitialize

If I use `CoInitializeEx` with `COINIT_APARTMENTTHREADED` it works, but e.g. with `COINIT_MULTITHREADED` it fails or with just `CoIncrementMTAUsage` it fails in threading tests with illegal memory access.