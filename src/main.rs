// フォーマットするにはcargo fmt

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
    // これをデバッグフォーマットという
    println!("{:?}", t1);
    // 要素を指定する場合はドットの後にインデックス番号を記述
    println!("{}", t1.0);

    let (x, y) = t2;
    println!("{}", x);

    // ユニット型
    // 値が何もない型のこと
    // 何も返さない関数の戻り値の型などで使用する
    let u = ();

    // 配列
    // 型に要素の型と数が含まれているため、一度作成すると要素数が変更できない
    let arr1 = [1, 2, 3];
    let arr2 = ["a", "b", "c"];
    // "aaa"が100個並んだ配列
    let arr3 = ["aaa"; 100];
    // 配列をそのままprintlnする場合は:?を記述する
    println!("{:?}", arr1);
    println!("{}", arr1[2]);
    let [_, yy, _] = arr1;
    println!("{}", yy);
    // 配列の一部分を切り出すことができる
    // 切り出した配列をスライスという
    // &をつけて開始位置と終了位置を指定（終了位置の要素は含まれない）
    // 下の例では 1 ≦ x ＜ 2
    // &は「参照」を表している（後に学習）
    let slice = &arr2[1..2];
    println!("{:?}", slice);
    // 終了位置を含める場合は下のように記述する
    let slice2 = &arr2[1..=2];
    println!("{:?}", slice2);
    // 開始位置、終了位置は省略できる
    // 省略した場合はそれぞれ初めから、最後までとなる
    // 両方省略した場合は配列全体となる
    let slice3 = &arr2[..=1];
    let slice4 = &arr2[1..];
    println!("{:?}", slice3);
    println!("{:?}", slice4);

    // 後から要素数を変更できる配列＝ベクタ
    let vec1 = vec![1, 2, 3, 4, 5, 6];
    println!("{:?}", vec1);
    println!("{:?}", &vec1[2..5]);
    // デフォルトでは変更できないためmutを使う
    // 下の例は空のベクタを作成している
    let mut vec2 = Vec::new();
    // 要素を追加
    vec2.push("aaa");
    println!("{:?}", vec2);
    vec2.push("bbb");
    println!("{:?}", vec2);
    vec2.push("ccc");
    vec2.push("ddd");
    vec2.push("eee");
    vec2.push("fff");
    println!("{:?}", vec2);
    // 末尾から1つ取得してベクタから消す
    let pop = vec2.pop();
    println!("{:?}", vec2);
    println!("{:?}", pop);
    // getを使用して値を取得できる
    let get = vec2.get(2);
    println!("{:?}", vec2);
    println!("{:?}", get);
    // Rustにはnullがなく代わりにOption型がある
    // 存在しないインデックスを指定してもエラーにならずNoneが返る
    let get2 = vec2.get(10000);
    println!("{:?}", get2);

    // 文字型
    // 1文字だけを表す
    let char = 'a';
    let char2 = '§';
    // 文字列型
    // 文字列リテラル（文字列スライスとも呼ばれる）
    // 値が変更できない
    let str = "aaaa";
    // string型
    // 値を変更したい場合に使う
    let str2 = String::from("aaaaa");
    let str3 = str.to_string();
    // string型は文字列の長さを変更できるので、push_strで末尾に文字列を追加できる
    let mut str4 = String::from("abcdefg, ");
    println!("{}", str4);
    str4.push_str("abcdef");
    println!("{}", str4);
    // +で結合することができる
    // 左がstring型、右が文字列スライスである必要がある
    println!("{}", String::from("abc") + "aaaaa");
    // formatマクロ
    // printlnは標準出力に出力するが、formatは変数に束縛させることができる
    let str5 = format!("{}{}", "aaaa", "bbbb");
    println!("{}", str5);
    let str6 = format!("{}{}", String::from("ssss"), "bbbb");
    println!("{}", str6);
    let str7 = format!("{}{}", "ssss", String::from("bbbb"));
    println!("{}", str7);
    let str8 = format!("{}{}", String::from("ttttt"), String::from("bbbb"));
    println!("{}", str8);

    // 関数の呼び出し
    sample_function();
    let result = sample_function2(1, 2);
    println!("{}", result);

    println!("{}", "block() start");
    block();
    println!("{}", "block() end");
}

// このように値を返さない関数は、戻り値の型が省略されているだけで、
// 実際はユニット型となっている
fn sample_function() {
    println!("{}", "sample function");
}

// 引数と戻り値がある関数
fn sample_function2(a: i32, b: i32) -> i32{
    // returnを書かず、セミコロンなしで書くことで戻り値になる
    // Rustではこちらの方が一般的
    // returnは条件分岐などで値を返すときに使う
    // 式にはセミコロンをつけない
    // セミコロンをつけないことで式と認識されて戻り値となる
    a + b
}

fn block() {
    // {}で囲まれたものをブロックという
    // ブロック単体で使うこともできる
    println!("{}", "a");
    // ブロック内で宣言された変数はブロック内でしか使えない
    {
        println!("{}", "aaaa");
    }
    println!("{}", "b");

    // シャドーイング（同じ変数を複数宣言すること）
    // ブロック内でシャドーイングする
    let y = 10;
    println!("{}", y);
    {
        let y = 19;
        println!("{}", y);
    }
    // ブロック内の宣言は影響しないため10のまま
    println!("{}", y);

    // ブロック内でセミコロンをつけずに書くことで値として返却される
    let z = {
        200
    };
    println!("{}", z);
}