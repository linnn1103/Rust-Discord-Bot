# CoolBot

一個非常荒唐的政治不正確Discord Bot
拿來做Rust的練習
提供給那些跟我一樣不知道為什麼要用Rust寫Bot的人做為參考。

>可能有敏感詞彙，麻煩對政治敏感的人移至[這裡](https://github.com/linnn1103/Rust-Discord-Bot/tree/harmonious)

## 機器人功能  
  
### 動態語音頻道  
  
機器人允許在伺服器中創建動態語音頻道，在`main.rs`中修改`TARGET_CHANNEL_ID`以設定用來創建動態語音頻道的房間、修改`CATEGORY_ID`設定動態語音頻道所屬的類別。  
  
完整邏輯於`main.rs``fn voice_state_update`  
  
### 指令  

**所有指令的邏輯可以在`/commands`下找到**  
  
`/重新命名 <name>` 允許動態頻道的擁有者修改動態頻道的名稱。  
  
`/求籤` 一個沒什麼用的運氣測試器。  
  
`/指令計數器 <command name>` 用來查看目標指令的使用次數。  
  
~~`/dirty_talk` 讓機器人跟你dirty_talk。~~  
  
~~`/习近平万岁` 我爱中国。~~  
  
~~emmmm... 好像有奇怪的東西混進來了~~  
  
### 訊息偵測  

允許Bot偵測用戶發送到伺服器的訊息，完整的邏輯位於`main.rs``fn message`。  
  
## 已知問題

- 當不同伺服器共用同一個機器人時同時也會共用data，倒置可能發生意外的錯誤，尤其是有重複使用者使用動態語音頻道時。  
  >目前在研究使用SQL建置能夠儲存更完整的資訊的解決方案，本人沒怎麼碰過SQL所以可能要很長一段時間  
