mod network;//これでnetwork.rsがコンパイル対象
fn main() {
    println!("Hello, world!");
    let c = Circle {radius:10};
    let c2 = Circle::new(32);
    println!("{}", 7 as f64 + 8.9);
    // クロージャー
    let mut one = 1;
    let plus_one = move |x| {x + one};
    one += 1;
    let eleven = plus_one(10);
    println!("{} {}",one,eleven);
    // 別ファイルに切り出した関数の実行
    crate::network::ping();
}

struct Circle {
    radius: i32
}
impl Circle {
    // Method
    fn area(&self) -> i32 {
        self.radius * self.radius
    }
    // 関連関数
    fn new(radius: i32) -> Circle {
        Circle {
            radius :radius
        }
    }
}
