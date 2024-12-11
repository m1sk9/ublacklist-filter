# ublacklist-filter

[英語版](./README.md)

特定ドメインの検索結果を Google を始めとする各種検索エンジンから除外する uBlacklist 用のフィルタです。

購読には uBlacklist の導入が必要です．

- Firefox: [uBlacklist](https://addons.mozilla.org/ja/firefox/addon/ublacklist/)
  - Firefox for Android を使用している場合でも購読可能です.
- Safari: [uBlacklist for Safari](https://apps.apple.com/jp/app/ublacklist-for-safari/id1547912640)

> [!WARNING]
>
> このリストは Firefox の uBlacklist でのみ動作確認を行っています．メンテナンスは Firefox, Safari でのみ行います．

## 使い方

uBlacklist 導入後，設定ページから購読リストに以下の URL を追加してください．

```
https://raw.githubusercontent.com/m1sk9/ublacklist-filter/main/build/ublacklist-filter.txt
```

詳しい購読方法は [公式ドキュメント](https://iorate.github.io/ublacklist/ja/docs/advanced-features#subscription) を参照してください．

## ブロック基準について

> [!IMPORTANT]
>
> このフィルタに対してブロック解除の Issue を送ろうとしているサイト管理者へ:
>
> **送らないでください**．私は自分の意志であなたのサイトを検索結果から除外するためにこのリストを作成しています．
> あなたがブロック解除の Issue を送信しても私はクローズするか，無視します．無駄な労力を使わせないでください．

- このフィルターにブロック基準は存在しません．
- 前述の通り，このフィルターは私の意志で特定のドメインを検索結果から除外するために作成されています．
  - GitHub で公開している理由は，他のユーザーが同様の目的で利用できるようにするためです．
- ただし以下のサイトについては例外なく **すべて** ブロックします．
  - プログラミングスクール: 卒業生が書いた低品質なブログ記事は何の参考にもならない
  - GitHub や Stack Overflow を機械翻訳したサイト: 何のためのサイトですか?

uBlacklist を本格的に利用したい場合は他のユーザーが作成したフィルタも一緒に購読することをお勧めします．**このフィルタはかなり極端なフィルタです．**

詳しくは [こちら](#推奨フィルタ) を参照してください

## フィルターの追加・削除

**前述した通り削除申請には応じません．**

- フィルタの追加については Issue またはプルリクエストを受け付けます．
  - ただし受け入れるかどうかは私の判断によります．
  - プルリクエストを提出するには Rust の環境が必要になるので非推奨です．
- ルールは `assets/rule.toml` に記述します．
  - このフィルタは **ドメイン** に対するフィルタです．ドメインを指定するようにします． (サイト全体をブロックするため)
  - `domain` にドメイン, `comment` にコメントを記述します．

```toml
[[rule]]
domain = "example.com"
comment = "This is an example rule."
```

- ルールが追加できたら `make build` を実行して `build/ublacklist-filter.txt` を生成します．
  - `make test` でテストを実行できます．

## 購読推奨フィルタ

購読できるフィルタ一覧は [公式ドキュメント](https://iorate.github.io/ublacklist/ja/subscriptions) にもあります．ここはほんの一例です．

私の [`👶 ublacklist`](https://github.com/stars/m1sk9/lists/ublacklist) というリストには実際に私が使用している uBlacklist のフィルタが含まれています．

- [ncaq](https://github.com/ncaq)'s [uBlacklistRule](https://github.com/ncaq/uBlacklistRule)
- [108EAA0A](https://github.com/108EAA0A)'s [ublacklist-programming-school](https://github.com/108EAA0A/ublacklist-programming-school)
- [arosh](https://github.com/arosh)'s [ublacklist-stackoverflow-translation](https://github.com/arosh/ublacklist-stackoverflow-translation)
