# general-scraper
汎用クローラー

** **under development** **

## 概要

JSONのDSLを入力として受け取り、行指向のデータにフォーマットして返す

DSLのフォーマットは[WebScraper](https://webscraper.io/)のSitemapに基づく

### 例

<details>
<summary>input</summary>

```
{
  "_id": "tech_crunch",
  "startUrl": [
    "https://jp.techcrunch.com/"
  ],
  "selectors": [
    {
      "id": "link",
      "type": "SelectorLink",
      "parentSelectors": [
        "_root"
      ],
      "selector": ".post-title a",
      "multiple": true,
      "delay": 0
    },
    {
      "id": "title",
      "type": "SelectorText",
      "parentSelectors": [
        "link"
      ],
      "selector": "h1",
      "multiple": false,
      "regex": "",
      "delay": 0
    },
    {
      "id": "pub_date",
      "type": "SelectorText",
      "parentSelectors": [
        "link"
      ],
      "selector": ".title-left time",
      "multiple": false,
      "regex": "",
      "delay": 0
    }
  ]
}
```
</details>
↓ クロール実行＆結果フォーマット
<details>
<summary>output</summary>

```
[
  {
    "source_url": "https://jp.techcrunch.com/",
    "pub_date": "2020年7月29日",
    "link": "https://jp.techcrunch.com/2020/07/29/ziploc-recycle-program/",
    "title": "ジップロックをリサイクルした傘のシェアリングサービスが開始、7月29日よりテラサイクルが一般回収スタート"
  },
  {
    "pub_date": "2020年7月29日",
    "link": "https://jp.techcrunch.com/2020/07/29/2020-07-23-microsoft-showcases-gameplay-from-halo-infinite-and-other-other-xbox-series-x-titles/",
    "title": "Halo InfiniteやForzaなどXboxシリーズXゲームのデモビデオ一挙公開",
    "source_url": "https://jp.techcrunch.com/"
  },
  {
    "pub_date": "2020年7月29日",
    "link": "https://jp.techcrunch.com/2020/07/29/2020-07-28-climacell-raises-23m-series-c-for-its-weather-intelligence-platform/",
    "title": "天候の予報と関連情報を提供するClimaCellが24億円超を調達、建築や運送での的確な判断を支援する基礎研究と戦略部門を強化",
    "source_url": "https://jp.techcrunch.com/"
  },
  {
    "link": "https://jp.techcrunch.com/2020/07/29/2020-07-28-twitter-donald-trump-jr-frontline-doctors-viral-video-misinformation/",
    "source_url": "https://jp.techcrunch.com/",
    "title": "Twitterがトランプ大統領の息子のアカウントを制限、新型コロナ誤情報の共有で",
    "pub_date": "2020年7月29日"
  },
  {
    "source_url": "https://jp.techcrunch.com/",
    "title": "眺めを重視したVirgin Galactic観光宇宙船の内装に注目",
    "pub_date": "2020年7月29日",
    "link": "https://jp.techcrunch.com/2020/07/29/2020-07-28-take-a-first-look-inside-virgin-galactics-spacecraft-for-private-astronauts/"
  },
  {
    "link": "https://jp.techcrunch.com/2020/07/29/aiwell/",
    "title": "東京工業大学発ベンチャー認定企業aiwellが資金調達を実施",
    "pub_date": "2020年7月29日",
    "source_url": "https://jp.techcrunch.com/"
  }
]
```
</details>

## TODO

- DSL
  - [x] Linkセレクタに対応
  - [x] Textセレクタに対応
  - [x] Elementに対応
  - [ ] Imageセレクタに対応
- crawler
  - [x] 再帰的なDSLのクロールに対応
  - [x] 差分取得のための`accessed_urls`と`skip_urls`に対応
  - [ ] delayに対応
  - [ ] ループ検出
- formatter
  - [x] 行指向のフォーマットに対応
  - [ ] Treeのトラバース方法を可変にする
- other
  - [ ] SQLiteをデータストアとした参照実装を作る(WIP)
  - [ ] PyO3でPythonからライブラリとして呼び出せるようにする
