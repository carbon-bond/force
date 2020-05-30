use force::lexer::lexer;
use force::parser::Parser;

fn main() {
    let source = "
分類 新聞 {
    單行 媒體
    單行 記者
    文本 內文
    單行 超鏈接
    文本 備註
}
分類 問卦[非根] {
    文本 內文
}
分類 解答[非根] {
    文本 內文
}
分類 留言 {
    文本/.{1,256}/ 內文
}

鍵結 {
   解答 --> 問卦

   回覆 -[挺,戰,回]-> * {
       挺 {
            輸能: [1]
       }
       戰 {
            輸能: [-1]
       }
       回 {
            輸能: [0]
       }
   }

   留言 --> *
}
";
    let tokens = lexer(&source);
    for token in &tokens {
        println!("{:?}", token);
    }
    let mut parser = Parser::new(tokens);
    let force = parser.parse();
    println!("{:?}", force);
}
