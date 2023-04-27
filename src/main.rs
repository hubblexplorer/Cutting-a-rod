use std::io;
use std::str::FromStr;

fn main() {
    // Ler a entrada
    let n = read_line::<usize>().unwrap();
    let m = read_line::<usize>().unwrap();

    let mut prices = vec![0; n + 1];

    for _ in 0..m {
        let input = read_line::<String>().unwrap();
        let mut iter = input.split_whitespace();
        let size = iter.next().unwrap().parse::<usize>().unwrap();
        let price = iter.next().unwrap().parse::<usize>().unwrap();
        prices[size] = price;
    }

    // Encontrar o lucro máximo usando programação dinâmica
    let max_profit = find_max_profit(n, &prices);

    // Exibir o lucro máximo
    println!("{}", max_profit);
}

fn read_line<T: FromStr>() -> Result<T, T::Err> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<T>()
}

fn find_max_profit(n: usize, prices: &[usize]) -> usize {
    let mut dp = vec![0; n + 1];

    for i in 1..=n {
        for j in 1..=i {
            dp[i] = std::cmp::max(dp[i], prices[j] + dp[i - j]);
        }
    }

    dp[n]
}
