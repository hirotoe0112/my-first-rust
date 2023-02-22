// 定数はconst
// letのデフォルトと動き的には同じ
// 型は必ず明記しなければならない
// 関数の外でも任意の場所に定義できる
// 定数名は大文字で書くのが慣例
const A: i32 = 1;
// letは関数の外では定義できない
// let aaa = 1;
fn main() {
    println!("Hello, world!");
    print!("Hello, ");
    print!("Rust!");
    // 波カッコでプレースホルダーが使える
    println!("Hello, {}", "Rust");
    /*
     複数行コメント
     */

    // letは変更不可
    let a = 1;
    let b = 2;
    // let mutなら変更可
    let mut c = 3;
    println!("{}", c);
    c = 4;
    println!("{}", c);

    // 直接println!(a)のようには書けない
    println!("{}", a);
    println!("{}", b);

    const ABC: i32 = 1;

    // 数値型
    // アルファベット+サイズ
    // iは符号付整数、uは符号なし整数、fは浮動小数点数
    // i8とかu16とか
    // 型推論の場合はデフォルトの型が指定される
    // 明示的に別の型を指定したい場合は定義を記述する
    let a1: i8 = 1;
    let a2 = 1.1111;
    // 値の後に型を書くこともできる
    let a3 = 123i16;

    // キャスト
    let g = 1;
    let f = 1 as f64;

    // 論理型
    // true or false
    let flg = true;
    let h = 1 == 2;
    println!("{}", h);

    // タプル型
    let t1 = (1, false);
    // 上の変数と下の変数は異なるもの
    let t2 = (false, 1);
    // タプルをprintlnするには:?を記述する
    println!("{:?}", t1);
    // 要素を指定する場合はドットの後にインデックス番号を記述
    println!("{}", t1.0);

    let (x, y) = t2;
    println!("{}", x);

    // ユニット型
    // 値が何もない型のこと
    // 何も返さない関数の戻り値の型などで使用する
    let u = ();
}
