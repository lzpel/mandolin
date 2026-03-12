// ブランケット実装のサンプル
//
// trait A を実装した全ての型に trait B を自動実装する

trait A {
    fn hello(&self) -> String;
}

// メソッドなしのマーカー的なtrait
trait B: A {
    fn greet(&self) -> String {
        format!("B::greet -> {}", self.hello())
    }
}

// ブランケット実装: A を実装する全ての型が自動的に B を満たす
impl<T: A> B for T {}

// ---

struct Foo;
struct Bar;

impl A for Foo {
    fn hello(&self) -> String { "Foo".to_string() }
}

impl A for Bar {
    fn hello(&self) -> String { "Bar".to_string() }
}

// B を明示的に impl しなくても B を要求する関数に渡せる
fn use_b(x: &impl A) {
    println!("{}", x.greet());
}

fn main() {
    use_b(&Foo); // B::greet -> Foo
    use_b(&Bar); // B::greet -> Bar
}
