# Usage
The basic usage is as follows:
- Press `q` to exit `lazy-etherscan`.
- Press `s` to focus on the search bar. You can search by the following words.
    - Address
    - Block Number
    - ENS ID
    - Transaction Hash
    - Ticker Name (`USDT`, `BNB`,`UNI`, ...)
- Press `1` to navigate the `Latest Blocks` panel. Use `j` to move to a block below and `k` to an above block.
    - Press `r` to refresh the `Latest Blocks`.
- Press `2` to navigate the `Latest Transactions` panel. Use `j` to move to a transaction below and `k` to move to a transaction above.
    - Press `r` to refresh the `Latest Transactions`.
- Press `<Ctrl+e>` to toggle the sidebar.
- Press `<Ctrl+p>` to move to a previous screen.

## Examples

### Searching by Tickers
`USDT`で検索する例を紹介する。
`s`を押して、検索バーにフォーカスする。そして、`i`を押して、編集モードに移行する。`USDT`とタイプして`Enter`を押す。
検索結果の画面にて、左側にコントラクトのソースコード、右側にコントラクトのABIが表示されており、左右の矢印キーで相互にフォーカスが可能である。`j`/`k`キーを押すことで、フォーカスされている要素をスクロールすることができる。

![demo](../resources/screenshots/ticker.png)
また、`<Ctrl+e>`を押すことで、サイドバーをトグルすることができ、以下の画像のようにソースコードとABIを両方同時に見ることができる。
![demo](../resources/screenshots/ticker_toggled.png)


### Exploring a Block
![demo](../resources/screenshots/block.png)
![demo](../resources/screenshots/block_toggled.png)
