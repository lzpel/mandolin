https://qiita.com/rchaser53/items/0b23ddbfaa13aa6ffb19

# console_error_panic_hook
これがないとwasm内でこけても「RuntimeError: Unreachable executed」しか出ない
これを使うとrustのエラーメッセージがconsole.errorとして出力される
開発中だけでも使うべき