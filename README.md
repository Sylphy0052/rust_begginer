# rust_begginer

## Rustとは
- C++を元に作られたC++よりも安全性の高い関数型言語
- 安全性・速度・並行性にフォーカス

## 開発環境
- `curl https://sh.rustup.rs -sSf | sh`
- `mkdir -p projects/rust_tutorial`
- `cd projects/rust_tutorial`
- `mkdir hello_world`
- `cd hello_world`
- `touch main.rs`

## Uninstall
- `sudo /usr/local/lib/rustlib/uninstall.sh`
- `rustup self uninstall`

## コンパイル・実行
- `rustc main.rs`
- `./main`

## Rustについて
- `println!`: マクロの呼び出し(!がある)

## Cargo
- 以下の3つを行う
  - ソースコードのビルド
  - 依存ライブラリのダウンロード
  - 依存ライブラリのビルド
- プロジェクトの作成
  - `cargo new hello_world --bin`
    - `--bin`はバイナリを作成する
- Cargo.toml: Cargoプロジェクトの設定ファイル
- ビルド・実行
  - `cargo build`
  - `./target/debug/hello_world`
  - もしくは
    - `cargo run`
- Releaseのしかた
  - `cargo build --release`
  - `./target/release/hello_world`

## gitignoreを生成するgibo
- `gibo dump Rust macOS`
- `gibo list`
  ```
    === Languages ===

  Actionscript		Go			Processing
  Ada			Godot			PureScript
  Agda			Gradle			Python
  Android			Grails			Qooxdoo
  AppceleratorTitanium	GWT			Qt
  AppEngine		Haskell			R
  ArchLinuxPackages	Idris			Rails
  Autotools		IGORPro			RhodesRhomobile
  C++			Java			ROS
  C			Jboss			Ruby
  CakePHP			Jekyll			Rust
  CFWheels		Joomla			Sass
  ChefCookbook		Julia			Scala
  Clojure			KiCad			Scheme
  CMake			Kohana			SCons
  CodeIgniter		Kotlin			Scrivener
  CommonLisp		LabVIEW			Sdcc
  Composer		Laravel			SeamGen
  Concrete5		Leiningen		SketchUp
  Coq			LemonStand		Smalltalk
  CraftCMS		Lilypond		Stella
  CUDA			Lithium			SugarCRM
  D			Lua			Swift
  Dart			Magento			Symfony
  Delphi			Maven			SymphonyCMS
  DM			Mercury			Terraform
  Drupal			MetaProgrammingSystem	TeX
  Eagle			Nanoc			Textpattern
  Elisp			Nim			TurboGears2
  Elixir			Node			Typo3
  Elm			Objective-C		Umbraco
  EPiServer		OCaml			Unity
  Erlang			Opa			UnrealEngine
  ExpressionEngine	OpenCart		VisualStudio
  ExtJs			OracleForms		VVVV
  Fancy			Packer			Waf
  Finale			Perl			WordPress
  ForceDotCom		Perl6			Xojo
  Fortran			Phalcon			Yeoman
  FuelPHP			PlayFramework		Yii
  Gcov			Plone			ZendFramework
  GitBook			Prestashop		Zephir

  === Global ===

  Anjuta			JDeveloper		Patch
  Ansible			JEnv			Redcar
  Archives		JetBrains		Redis
  Backup			Kate			SBT
  Bazaar			KDevelop4		SlickEdit
  BricxCC			Lazarus			Stata
  Calabash		LibreOffice		SublimeText
  Cloud9			Linux			SVN
  CodeKit			LyX			SynopsysVCS
  CVS			macOS			Tags
  DartEditor		Matlab			TextMate
  Dreamweaver		Mercurial		TortoiseGit
  Dropbox			MicrosoftOffice		Vagrant
  Eclipse			ModelSim		Vim
  EiffelStudio		Momentics		VirtualEnv
  Emacs			MonoDevelop		VisualStudioCode
  Ensime			NetBeans		WebMethods
  Espresso		Ninja			Windows
  FlexBuilder		NotepadPP		Xcode
  GPG			Otto			XilinxISE
  ```

## Reference
1. [春休みだしRust入門しようぜ(1): Qiita](https://qiita.com/musaprg/items/97a72bb1ba85932ad161)
2. [プログラミング言語 Rust 日本語版](https://rust-lang-ja.github.io/the-rust-programming-language-ja/1.6/book/README.html)
3. [Rustのリンク集: Qiita](https://qiita.com/moshroom/items/7e327dafbe53b72ad99d)
4. [The Rust Programming Language](https://doc.rust-lang.org/book/second-edition/index.html)
