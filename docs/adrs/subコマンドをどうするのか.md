# subcommand をどうするのか

## 言語ごとにする

```
tg rs -po -d "Clone,Copy"
tg go -poP
```

- 利点はその言語ごとのオプションを柔軟に設定できる
- 想像としては，Command の Config と BuildTrait を言語ごとに用意して，やる
- 欠点は，全言語共通のところはかぶる
  - 最悪，CommonConfigTrait みたいなやつを作っておいて，そこは共通のマクロで設定する
  - trait 境界を Common と言語独自のものにする
  - この時の Common は言語の仕様と関係ない WhiteList とか Pub(Python みたいな言語だとこれはから実装になってしまうが．．．),
  - ただこの共通のやつやると，langs がこの CLI コマンドの仕様に依存するようになる
    - それでもいいのかどうか
    - 最悪，langs の方でよしなに作ったやつを wrap する形で cli との関係を疎にすることも可能
    - そういう意味だと，cli の方で言語仕様に関係ないやつは実装する?
    - ただそうなると pub とかは cli の方では実装不可
- サブコマンドは動詞ややることの気がするが，それが言語だと，少し感覚と合わない
- 言語は設定であってサブコマンドではない気もする

## 生成方法にする

```
tg req -po -s url-config.json -l rs
tg convert -s src -l go -poP
```

- 利点は意味がわかりやすい
- 依存関係はどうなる?
- 言語共通の設定と，言語独自の設定はどうやってホストするか
