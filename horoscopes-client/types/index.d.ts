// Memo: file loaderで画像ファイルをJSにBundleするための型定義
// 参考文献:
//   Main:
//      https://qiita.com/gaaaaaasuuuufe/items/ef9fa15a8bc34b1296c7
//   Sub:
//      https://qiita.com/babie/items/25aa63e14c06e4a9a046
//      https://qiita.com/terrierscript/items/56d2cc15f76df50dfee7
//
declare module "*.png" {
  const src: string;
  export default src;
}

declare module "*.jpg" {
  const src: string;
  export default src;
}

declare module "*.gif" {
  const src: string;
  export default src;
}

declare module "*.svg" {
  const src: string;
  export default src;
}
