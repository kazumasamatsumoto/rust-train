use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

// use anyhow::Result;:

// anyhowクレートからResult型をインポートします。このResult型はエラーハンドリングを簡素化するために使用され、標準ライブラリのResult型と互換性があります。
// anyhow::Resultはanyhow::Errorをエラー型として使うため、様々なエラーを簡単に扱えます。
// use clap::Parser;:

// clapクレートからParserトレイトをインポートします。これにより、構造体に対してコマンドライン引数の解析機能を追加できます。
// Parserトレイトは、コマンドライン引数を解析し、それに基づいて構造体を生成するために使用されます。
// use std::fs::File;:

// 標準ライブラリのfsモジュールからFile型をインポートします。File型はファイルを開いたり作成したりするために使用されます。
// use std::io::{self, BufRead, BufReader};:

// 標準ライブラリのioモジュールからいくつかの型とトレイトをインポートします。
// io: std::ioモジュール自体をインポートします。これにより、io::Resultなどの完全修飾名でアクセスできます。
// BufRead: バッファ付きリーダーを提供するトレイトで、行単位の読み取りが可能になります。
// BufReader: バッファ付きリーダーの具象型で、任意のリーダーに対してバッファリングを行い、効率的な読み取りを可能にします。

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,

    #[arg(short('n'), long("number"), conflicts_with("number_nonblank_lines"))]
    number_lines: bool,

    #[arg(short('b'), long("number-nonblank"))]
    number_nonblank_lines: bool,
}

// #[derive(Debug, Parser)]
// Debugトレイト: このトレイトを派生させることで、構造体のインスタンスをデバッグ形式で表示できるようになります。例えば、println!("{:?}", args);といったコードで使います。
// Parserトレイト: clapクレートのトレイトで、コマンドライン引数を解析するために必要です。これにより、構造体がコマンドライン引数から自動的にデータを取得できるようになります。
// #[command(author, version, about)]
// author: プログラムの作者情報を表示します。
// version: プログラムのバージョン情報を表示します。
// about: プログラムの概要を表示します。
// これらの情報は、コマンドラインで--helpオプションを指定したときに表示されます。
// files: Vec<String>
// #[arg(value_name = "FILE", default_value = "-")]
// value_name: コマンドライン引数の名前を指定します。ここではFILEとして表示されます。
// default_value: 引数が指定されなかった場合のデフォルト値を指定します。ここではデフォルトで"-"（標準入力）を使用します。
// このフィールドは、複数のファイル名を受け取るためのベクタとして定義されています。
// number_lines: bool
// #[arg(short('n'), long("number"), conflicts_with("number_nonblank_lines"))]
// short('n'): 短い形式のオプションとして-nを指定します。
// long("number"): 長い形式のオプションとして--numberを指定します。
// conflicts_with("number_nonblank_lines"): number_nonblank_linesオプションと同時に使用することはできません。これにより、二つのオプションが同時に指定された場合にエラーが発生します。
// number_nonblank_lines: bool
// #[arg(short('b'), long("number-nonblank"))]
// short('b'): 短い形式のオプションとして-bを指定します。
// long("number-nonblank"): 長い形式のオプションとして--number-nonblankを指定します。

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

// main関数の解説
// Args::parse():

// これは、clapクレートの機能を使ってコマンドライン引数を解析し、その結果をArgs構造体にパースします。
// Args構造体は、前回のコード例で定義した通り、コマンドライン引数のオプションや引数を保持するための構造体です。
// run関数の呼び出し:

// run関数は、Args構造体を引数として受け取ります。この関数が、実際のプログラムのロジックを実行する部分です。
// run関数の戻り値はResult型であり、成功した場合はOk(())、失敗した場合はErr(e)を返します。
// if let Err(e) = run(Args::parse()):

// ここでは、run関数の呼び出し結果がErrであるかどうかをチェックしています。
// if let構文を使って、run関数の戻り値がErrであった場合、そのエラー値をeにバインドします。
// eprintln!("{e}");:

// エラーが発生した場合、eprintln!マクロを使ってエラーメッセージを標準エラー出力（stderr）に表示します。
// {e}は、エラー値をフォーマットして表示するためのプレースホルダーです。
// std::process::exit(1);:

// エラーが発生した場合、プログラムを終了させます。
// std::process::exit(1)は、終了ステータスコード1でプログラムを終了します。一般的に、0は正常終了、0以外の値はエラー終了を示します。

fn run(args: Args) -> Result<()> {
    for filename in args.files {
        match open(&filename) {
            Err(e) => eprintln!("{filename}: {e}"),
            Ok(file) => {
                let mut prev_num = 0;
                for (line_num, line_result) in file.lines().enumerate() {
                    let line = line_result?;
                    if args.number_lines {
                        println!("{:6}\t{line}", line_num + 1);
                    } else if args.number_nonblank_lines {
                        if line.is_empty() {
                            println!();
                        } else {
                            prev_num += 1;
                            println!("{prev_num:6}\t{line}");
                        }
                    } else {
                        println!("{line}");
                    }
                }
            }
        }
    }

    Ok(())
}

// run関数は、コマンドライン引数を保持するArgs構造体を引数に取り、anyhow::Result<()>を返します。Resultはエラーハンドリングのための型で、Ok(())が成功を示し、Err(e)がエラーを示します。
// args.filesに含まれる各ファイル名についてループ処理を行います。
// open関数を使用してファイルを開きます。ファイルのオープンに失敗した場合、エラーメッセージを標準エラー出力に表示します。成功した場合は、ファイルハンドルをfileに保持します。
// prev_numを初期化します。これは空でない行番号を管理するために使用されます。
// file.lines().enumerate()を使ってファイルの各行を読み取り、行番号と共に処理します。line_result?は、行の読み取りに失敗した場合にエラーを伝播させます。
// 行番号を付ける場合 (args.number_linesがtrueの場合)
// 各行に行番号を付けて表示します。
// 空行以外の行番号を付ける場合 (args.number_nonblank_linesがtrueの場合)
// 行が空である場合、単に空行を表示します。
// 行が空でない場合、prev_numをインクリメントし、その行にprev_numを行番号として付けて表示します。
// 行番号を付けない場合 (その他の場合)
// 単に行をそのまま表示します。
// 関数が正常に終了したことを示します。
/**
 * まとめ
このrun関数は、与えられたファイル名のリストに対して以下の処理を行います：

各ファイルを開く。
ファイルの内容を行ごとに読み取る。
指定されたオプションに従って行に番号を付けるか、またはそのまま表示する。
エラーが発生した場合はエラーメッセージを表示する。
この構造により、コマンドライン引数に基づいた柔軟なファイル処理が可能になります。
 *
 */

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

/**
 * この関数openは、指定されたファイル名に基づいて適切なバッファ付きリーダー（BufReadトレイトを実装したオブジェクト）を返します。ファイル名が"-"の場合は標準入力（stdin）を読み込み、それ以外の場合は指定されたファイルを読み込みます。戻り値は、動的ディスパッチを使用してBox<dyn BufRead>型として返されます。これにより、標準入力またはファイルのどちらのソースからもデータを読み取ることができます。
 */