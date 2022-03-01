
> 過度複雜，碳鍵已廢棄此設計

# （基本）力

碳鍵的分類定義語言，用以定義

- 分類之間的鏈接關係
- 一個分類的內部結構

## 欄位與鍵結

``` js
新聞 {
    單行 媒體
    單行 記者
    文本 內文
    單行 超鏈接
    文本 備註
}
問卦 {
    文本/.{256,}/ 內文
}
解答 {
    鍵結[問卦,留言] 問題
    文本 內文
}
留言 {
    鍵結[*] 本體
    文本/.{1,256}/ 內文
}
回覆 {
    鍵結[*] 原文
    文本 內文
}
```

## 分類族
``` js
// 原文可以指向 「批踢踢文章」分類族、「狄卡文章」分類族、「新聞」分類
// 也就是說除了留言之外的分類都可以被指到
留言 { 鍵結[@批踢踢文章, @狄卡文章, 新聞] 原文 } 
新聞 {}
八卦 @[批踢踢文章] {}    // 「八卦」分類屬於「批踢踢文章」分類族
政黑 @[批踢踢文章] {}    // 「政黑」分類屬於「批踢踢文章」分類族
有趣 @[狄卡文章] {}      // 「有趣」分類屬於「狄卡文章」分類族
```