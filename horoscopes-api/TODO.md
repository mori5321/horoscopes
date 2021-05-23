# Want
- [ ] cli作成
  - [ ] filter生成コマンド 

- [ ] Error系
  - [ ] 生のコンストラクタを直で呼んでいるので、ラップしたコンストラクタを定義したい。
  - [ ] warpの組み込みのエラーをJSONに変換してResponseにしたい。
    - [ ] Method Not Allowd とか Invalid Field的なやつ
  - [ ] SystemError系はログを残したい
    - [ ] 通知もしたい
  - [ ] ログを取る際にエラーのバックトレースを残したい。(大本はどこのエラーなのか)
    - [ ] エラーツリーの末端を取得する
    - [ ] エラーツリーをVecで表現する
  - [ ] Repository系のエラーも実装する
  - [ ] Error Messageなどのリテラルをyamlで管理したい、i18n的なやつ。

- [ ] linterほしい
  - [ ] できればprecommitとCIで動かしたい

- [ ] commit hookでcargo fmt(rustfmt, lint = clippy)動かす。
