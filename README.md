<p align="center">
    <img src="https://github.com/user-attachments/assets/efd0470a-f5cb-4c1d-a0c3-3f5c39113933" style="align-self: center"/>
</p>


# 📚 Hitomi Downloader

A GUI-based multi-threaded downloader for hitomi hitomi.la

English / [简体中文](./README.zh-CN.md)

## 📥 Download

Pre-compiled packages are available on the [Releases page](https://github.com/Transwarpcom/hitomi-downloader/releases). Just download and use.

**Enjoying this project? feel free to support it with a GitHub Star⭐! Your support motivates me to keep updating and maintaining🙏**

## ✨ Features

| Feature                            | Description                                                  |
| ---------------------------------- | ------------------------------------------------------------ |
| 🖼️ GUI                              | Built with [Tauri](https://v2.tauri.app/start/), lightweight, clean, and easy to use. |
| ⚡ Multi-threaded Downloading       | Maximize download speed.                                     |
| 📂 Export                           | One-click export to universal PDF or CBZ formats.            |
| 🌐 Internationalization             | Built-in multi-language support system (i18n).               |
| 🗂️ Customizable Directory Structure | Highly customizable directory structure and naming rules, supporting fields like type, author, language, etc. Say goodbye to the hassle of manual organization. |

## 🖥️ GUI

![image](https://github.com/user-attachments/assets/fd93fd2f-db16-43b6-86cf-aa643eb572c8)
![image](https://github.com/user-attachments/assets/81a859f2-2a06-4eca-b45f-4f6555cc62c0)


## 📖 How to Use

1.  In `Search` tab search for keywords.
2.  Click the `Download` button directly on the comic card, or click the cover/title to go to the `Comic` tab, where you'll also find a `Download` button.
3.  After downloading, click the `Open Folder` button to check the results.

**By the way, you can export to PDF/CBZ(ZIP) in the `Local` tab.**

📹 The video below demonstrates the full usage process. **It's Safe For Work, so feel free to watch.**

https://github.com/user-attachments/assets/d2d0e577-c074-41ca-996f-445d52e2cce5



## ⚠️ About Antivirus False Positives

For individually developed projects, this issue is almost unavoidable (~~because it requires purchasing a digital certificate for software signing, or even paying protection fees to antivirus companies~~).
The only solutions I can think of are:

1.  Compile it yourself according to the **How to Build** instructions below.
2.  Trust my promise that everything you download from the [Release page](https://github.com/Transwarpcom/hitomi-downloader/releases) is safe.

## 🛠️ How to Build

Building is very simple, just 3 commands.
~~Prerequisite you have Rust, Node, and pnpm installed.~~

#### 📋 Prerequisites

-   [Rust](https://www.rust-lang.org/tools/install)
-   [Node](https://nodejs.org/en)
-   [pnpm](https://pnpm.io/installation)

#### 📝 Steps

#### 1. Clone this repository

```
git clone https://github.com/Transwarpcom/hitomi-downloader.git
```

#### 2. Install dependencies

```
cd hitomi-downloader
pnpm install
```

#### 3. Build

```
pnpm tauri build
```

## 🌐 Adding a New Language

Help with translating this project is welcome! If you want to add a new language, please refer to the implementation in [PR #1](https://github.com/Transwarpcom/hitomi-downloader/pull/1). This PR shows how to add the localization files for `en-us`.

Main steps for adding a new language:

1.  Create a new language file in the `src/locales` directory.
2.  Translate the key-value pairs, following the format of the existing language files.
3.  Register the new language in `src/locales/index.ts`.
4.  Submit a PR.

## 🤝 Submitting PR

**Please submit Pull Requests to the `develop` branch.**

**If you want to add a new feature, please open an `issue` or `discussion` first to talk about it. This helps avoid wasted effort.**

For other cases, feel free to submit a PR directly, for example:

1.  🔧 Improvements to existing features.
2.  🐛 Bug fixes.
3.  🌐 Adding new language support.
4.  ⚡ Using a more lightweight library to implement existing features.
5.  📝 Documentation revisions.
6.  ⬆️  Pull Request for upgrading/updating dependencies will also be accepted.

## ⚠️ Disclaimer

-   This tool is intended for learning, research, and communication purposes only. Users should assume all risks associated with its use.
-   The author is not responsible for any losses, legal disputes, or other consequences resulting from the use of this tool.
-   The author is not responsible for the user's actions while using this tool, including but not limited to actions that violate laws or the rights of any third party.

## Thanks

[Pupil](https://github.com/tom5079/Pupil)
[original hitomi-downloader](https://github.com/lanyeeee/hitomi-downloader)

## 💬 Other

Any problems encountered during use or any features you would like to add, welcome to open a `issue` or `discussion`. I will do my best to address them.
