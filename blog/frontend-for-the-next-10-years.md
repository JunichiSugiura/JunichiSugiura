# 今後10年のフロントエンドを模索し続けた1年

## はじめに

こんにちは、Junです。普段はパリの[Ledger](https://www.ledger.com/)で働いています。

業務では主にTypeScriptとElectron, React Nativeなどを用いてクロスプラットフォームなアプリの開発を担当しています。

## Reactとの出会い

Reactは自分のエンジニアキャリアにとって切っても切り離せない存在です。

僕がReactを書き始めたのは2015年のちょうどこの時期です。当時はまだRailsエンジニアとして働いていましたが、趣味で色々なJavaScriptベースのUIライブラリを触っていました。

そんな過程でReactと出会います。会社に内緒で勝手にReactで一部の機能を実装し、冗談半分でクリスマスにPRを投げました。そしたら案外すんなりとマージされてしまい、以降はReactエンジニアとして今に至ります。 

この経験が今のワークスタイルに繋がっていて、今までフロントエンドだけではない様々な分野で新たなライブラリの導入を行ってきました。

## 現職とJavaScript

現在は[Ledger Live](https://www.ledger.com/ledger-live)というハードウェアウォレットとシンクさせて使うアプリの開発を担当してます。

Ledger LiveではElectron + React (Native)という構成でLinux, macOS, Windows, iOS, Androidと計5つのプラットフォーム向けにアプリを提供しています。

https://github.com/LedgerHQ/ledger-live

ほぼ全てJS (TS)で実装されているので、デスクトップとモバイルのUI以外は[別ライブラリ](https://github.com/LedgerHQ/ledger-live/tree/develop/libs)としてロジックがシェアされています。また採用面でも(JSに極端に初心者が多いということを除いては)、比較的見つけやすいというメリットもありました。npmも充実していていいですよね。

ただアプリが大きくなるにつれ、パフォーマンスの低下やバンドルサイズの増加など様々な問題も出てきました。またバグの数も増え、火消しモードに入る回数も多くなっていきました。

また長い目で今後の数年を想像したときに、例えばデスクトップからモバイルにユーザーが移行したように、今後はメタバースのような新しいプラットフォームにも一部のユーザーは移行していくでしょう。そんな中今のツールセットで満足なUXを生み出す姿が僕には想像ができませんでした。

そんなこんなでJavaScriptに対して個人的にそこまで未来をあまり感じなくなり、個人開発でTSを書くことは徐々になくなりました。

## Rustの到来

代わりに書きはじめたのはRustです。むしろ本格的にRustを書き始めたからJSを書かなくなったといってもいいかもしれません。

正直いって僕はこれまで２回Rustにトライしてます。最初の頃はまだv1.0以前だったこともありwasmにコンパイルしてみて遊ぶ程度でした。当時は所有権やライフタイムなど新しいコンセプトを勉強してみたい、くらいの感覚で特に業務に採用するまでは考えてませんでした。

2回目はasync/awaitが実装されてからです。この頃にはもうサーバーを簡単に実装できるライブラリなどがすでに登場していたので、個人開発の一部で使ったりしてました。ただLedgerに入ってからはほぼサーバーを触ることがなくなったので業務では使い所を見出せずに終わりました。

ただ現時点でRustの

- 安全性
- パフォーマンス
- サイズ
- 互換性
- モダンなツール (version manager, test runner, formatter docs, etc.)
- 親切なコンパイラー
- コミュニティ

これらを備えた言語は他にあるでしょうか？

今後数十年使っていけるように12年間かけて今の形になった言語です。僕にもいずれ書く時が来るだろうと思っていました。

そして去年、ようやく僕にもRustを活用できそうな状況になってきました。

ある時フランスでも”どうぶつの森”がヒットしているのをみて、もしかしたらゲーム内で出てくる”カブ”のように現実世界では少々小難しく感じる概念でもゲームならみんな抵抗なくプレイするじゃないか？Ledger Liveの機能もゆるーいどうぶつたちとの対話式なら、みんな抵抗なくクリプトを使い始めるんじゃないか？と思い始めました。

## Rust製のゲームエンジン

そこで今まで無知だったゲームエンジンについて色々調べ始めます。

まずRust製で見つけたのは[Amethyst](https://amethyst.rs/)というゲームエンジンです。

このエンジンの売りは[Entity Comonent System](https://en.wikipedia.org/wiki/Entity_component_system)を採用していてデータドリブンな開発を行えるというところでした。自分はECSについて全く知識がなく、調べていくうちに[Unity](https://unity.com/ecs)をはじめ、ゲーム開発者の中では割と普及しつつあるデザインパターンだということがわかりました。ロジックを細かいパーツに分けてそれを組み合わせることでシステムを構築していくという概念だったり、データドリブンな開発にフォーカスしている点はReactに通じるものがあり興味が湧きました。この時は何でゲーム開発者のコミュニティだけで、webの世界ではあまり聞かない概念なんだろうとぼんやり思っていました。

また、Amethystだけではなくその他にもいくつかのRust実装があることがわかり、それらのパフォーマンスを比較した[レポジトリ](https://github.com/rust-gamedev/ecs_bench_suite)に辿り着きました。

### Bevy

そこで目に留まったのが[Bevy](https://bevyengine.org/)です。

調べてみるとAmethystのECSより新しい実装で使い方もかなりシンプルだということがわかりました。

またBevyはコアも含めほぼ一貫してECSで設計されており、レンダラーなども全てプラグインとして提供されている点に興味が湧きました。

systemとしてのfunctionの引数の型を見て自動でクエリーしてきてくれるのも簡単でいいです。

ライフタイムや所有権といったRust特有のコンセプトも登場機会は少なく、初心者でも比較的簡単にRustを書き始める事ができます。

ドキュメントはあまりありませんでしたが、[`examples/`](https://github.com/bevyengine/bevy/tree/main/examples)が結構充実していて、それを一つ一つ走らせていくだけである程度機能を把握することができました。

大抵のRustのライブラリはこのようにexamplesとあとはソースコード内の[doc comments](https://doc.rust-lang.org/reference/comments.html#doc-comments)から自動生成される[Docs.rs](https://docs.rs/)があるのでそれを見れば大抵謎解きできます。

中でも興味深かったのは[minimum example](https://github.com/bevyengine/bevy/tree/main/examples#the-bare-minimum)でレンダラーも何もなし、コンソールでただ起動してstdoutして終了するという非常にシンプルな例です。僕はここに一番の可能性を感じました。なぜならこの時初めて、もしかするとESCってゲーム以外でも使えるかもしれない？とアイディアが生まれたからです。そこから１年をかけて週末や朝の時間を使って色々なPoCを作っていきました。

Rust製の[Tonic](https://github.com/hyperium/tonic)を使ってgRPCサーバーを作ってみたり、[clap](https://github.com/clap-rs/clap)と組み合わせてCLIツールを書いたり、３Dゲーム空間とLedgerのハードウェアウォレットを繋いでみたりと色々な技術との相性を見ていきました。

その頃ちょうど[Tauri](https://tauri.app/)が巷で盛り上がってきたという事もあってフロントエンドとしての活用も視野に入ってきました。ただやはりReactディベロッパーとしては宣言的UIが欲しいところ。また、他の言語で書いた場合、それぞれの言語で型を２つ用意しないといけなかったり、いちいちデータを(de)serializeしてRPCで送るのもなぁとあまり積極的にはなりませんでした。どうせなら全部Rustで書きたいじゃないですか。

## Rustで宣言的UI

[Yew](https://yew.rs/)は以前使ったことあるけど、他にも何か新しく出てないかな。そんなことを思っていたら[Dioxus](https://dioxuslabs.com/)というライブラリを見つけました。

Reactとよく似たAPIを提供していて、component, state, props, hooks, global context, routerと何でもござれでございます。

そしてVirtual DOMがYewのようにWASM縛りの実装ではなく、ネイティブのRustとして動くように実装されていて、TauriベースのwebviewにDOMの編集のみを行う薄いJavaScriptがinjectされているといった点が腑に落ちました。

またReact Nativeで言うところのYogaに該当するflexbox[ライブラリ](https://github.com/DioxusLabs/taffy)は拡張性も高く、のちにTUI(Text User Interface)も作れるようになったり、Bevyの方でも使われるようになったりとその柔軟性に可能性を感じました。

もう一つ大きな点としてTauriのwindow managerとして使われているのはBevyに使われている[winit](https://github.com/rust-windowing/winit)というライブラリのフォークだということが分かり、これならBevyのプラグインにできるんじゃないか？と徐々に確信が生まれてきました。　

## 結論としてのdip

そこでDioxusのレポジトリをフォークして[DesktopPlugin](https://github.com/diptools/dip/tree/main/plugins/ui/desktop)の開発を始めました。結果としてこれをbevy_dioxusとして今年の5月にGitHubとcrates.ioの方にパブリッシュしました。名前はBevyのサードパーティープラグインの命名規則に従ったんですが、長くて人に伝えづらいのと、CLIコマンドとしてタイプしやすいものが欲しかったので今は[dip](https://dip.tools/)という名前で開発しています。

dipには２つの目的があり、一つはBevy ECSをゲーム以外のユースケースに応用すること。

もう一つは僕の本業でもあるweb3分野のインフラ(ウォレット、ブロックチェーンエクスプローラーなど)をプラグインとしてゲームの世界でも使えるようにすることです。

そしてデスクトップアプリからサーバー、CLI、web、メタバースのようなゲームまでプラグインの組み合わせ次第でどんなアプリケーションにもできるツールキットを目指しています。

まだdocsの用意はないのですが、興味ある方はぜひ

- [GitHub](https://github.com/diptools/dip)
- [Discord](https://discord.gg/4R8AtxAxk3)
- Twitter ([@JunichiSugiura](https://twitter.com/JunichiSugiura))

などで絡んでいただければご案内します。

それではみなさん来年もいいお年を！
