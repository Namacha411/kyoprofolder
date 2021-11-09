# kyoprofolder

テンプレートを書いたプログラムフォルダを任意の数作ります。

設定ファイルを持っているため、他ユーザにおすすめのテンプレートを共有したり、PCを変えた際にそのままテンプレートを移行できるメリットがあります。

atcoder, codefoces, バーチャルコンテストなど競技プログラミングで使用することを目的としています。

## インストール

以下のコマンドでインストールできます。
インストールにはcargoが必要になります。

```sh
cargo install --git https://github.com/Namacha411/kyoprofolder
```

## 使用例

powershell

```powershell
kf abc226 Python @(1..8)
```

bash

```bash
kf abc226 Python {a..h}
```

コマンドを実行すると実行したディレクトリ下で以下のようなフォルダを生成します。
それぞれのプログラムファイルの中身は[設定ファイル](#設定ファイル)で設定した
`template`の値です。

powershell

```powershell
 abc226
  ├── 1.py
  ├── 2.py
  ├── 3.py
  ├── 4.py
  ├── 5.py
  ├── 6.py
  ├── 7.py
  └── 8.py
```

bash

```bash
 abc226
  ├── a.py
  ├── b.py
  ├── c.py
  ├── d.py
  ├── e.py
  ├── f.py
  ├── g.py
  └── h.py
```

## 使い方

```sh
USAGE:
    kf <folder-name> <lang> [srcs]...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <folder-name>    Folder name
    <lang>           Programing language
    <srcs>...        File names
```

このコマンドを実行したディレクトリ下にfolder-nameに指定した名前のフォルダを作成し、
そのフォルダ内にlangで指定した言語のテンプレートを設定ファイルから読み込みファイルを作成します。

## 設定ファイル

設定ファイルの保存場所は[dirs-rs](https://github.com/dirs-dev/dirs-rs)によって決めています。

それぞれのOSでの設定ファイルの位置は以下のようになっています。

| OS      | path                                                        |
| ------- | ----------------------------------------------------------- |
| linux   | /home/alice/.config/kyopro_folder.toml                      |
| windows | C:\Users\Alice\AppData\Roaming\kyopro_folder.toml           |
| max     | /Users/Alice/Library/Application Support/kyopro_folder.toml |

また、初期設定では内容は以下のようになっています。

```toml
[[languages]]
language = "Python"
extention = "py"
template = """
def main():
    pass


if __name__ == "__main__":
    main()
"""
```

このテーブルを書き替えることによって複数の言語のテンプレートを作ることができます。

また、`language`を`cpp-ac`、`cpp-cf`のように変えることによって１種類の言語でもatcoder用、codefoces用と複数作ることができます。

```toml
[[languages]]
language = ""
extention = ""
template = """

"""
```
