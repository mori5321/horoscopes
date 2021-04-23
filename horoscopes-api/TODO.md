# Must
- [x] DBとの連携
  - [x] diesel
  - [x] mysql用意 まとめてDocker化してもいいなぁ...
  - [x] repository層 用意(DI & usecase層は別タスクでOK)
  - [x] OnMemoryのRepository用意
  - [ ] DBに接続したRepository用意

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
- [ ] ULIDの生成・注入
- [ ] Validation
  - [ ] Handler層でのValidation
    - [ ] Stringの文字数
    - [ ] 数値の不許可範囲(<- ほぼDomainErrorでは...?)
  - [x] Entity層でのValidation
- [ ] 認証との連携
- [x] Test
  - [x] 単体テスト
  - [x] 結合テスト
- [ ] ロギング
- [ ] Docker化
- [ ] デプロイ
- [ ] SqlXをdieselと比較検討する。
- [ ] Rust Docを活用したい

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

