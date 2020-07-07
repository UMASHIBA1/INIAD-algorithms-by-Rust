// # bit全探索を理解しよう！
// # 部分集合を全パターン列挙できる

// # この問題とかで使えるよ！
// # https://atcoder.jp/contests/abc045/tasks/arc061_a

const search = () => {
  const n = 8;

  // 1<<nは実質2^n, 2^nは全パターン数に一致
  // 2^n回繰り返し
  for (let bit = 0; bit < 1 << n; bit++) {
    const ss = [];
    // n回繰り返し
    for (let i = 0; i < n; i++) {
      // bitが7だった場合、0,1,2どれでも&したら1になるので全パターン列挙
      if (bit & (1 << i)) {
        ss.push(i);
      }
    }

    let result = bit + ": {";
    for (let i = 0; i < ss.length; i++) {
      result += `${ss[i]} `;
    }
    result += "}";
    console.log(result);
  }
};

search();
