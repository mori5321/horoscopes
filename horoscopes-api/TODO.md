# Must
- [x] DBとの連携
  - [x] diesel
  - [x] mysql用意 まとめてDocker化してもいいなぁ...
  - [x] repository層 用意(DI & usecase層は別タスクでOK)
  - [x] OnMemoryのRepository用意
  - [ ] DBに接続したRepository用意
  - [ ] Organization/User概念を早いとこつくってしまう。

- [ ] AuthMiddlewareつくってしまう

- [z] Todo系の機能を一通りつくる(永続化層はMockでOK)
  - [x] List
  - [x] Get
  - [x] Create
  - [x] Update
  - [x] Delete

- [x] DTO周り
- [x] エラーハンドリング
  - [x] 試実装してみる。
  - [x]各階層にエラーをもたせる
  - [x] 各階層で発生したエラーを最終的にどのように捌くべき?
- [x] ULIDの生成・注入
- [x] Validation
  - [x] Usecase層でのValidation
  - [x] Entity層でのValidation
- [ ] 認証との連携
- [x] Test
  - [x] 単体テスト
  - [x] 結合テスト
- [x] ロギング
- [ ] Docker化
  - [x] 開発環境
  - [x] build環境
  - [ ] production環境
- [ ] デプロイ
- [x] SqlXをdieselと比較検討する。
- [x] Rust Docを活用したい

- [ ] commit hookでcargo fmt(rustfmt)動かす。


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
