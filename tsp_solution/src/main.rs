use std::io;

fn solver() {
    println!("\n==============================================");
    println!("                    PETUNJUK");
    println!("==============================================\n");
    println!("1. n sebagai jumlah kota");
    println!("2. baris merepresentasikan kota asal");
    println!("3. kolom merepresentasikan kota tujuan");
    println!("4. input angka merepresentasikan harga");
    println!("5. input 0 merepresentasikan kota");
    println!("   tidak terhubung, serta jika baris");
    println!("   dan kolom sama");
    println!("\n==============================================\n");

    print!("Masukkan jumlah kota (n): ");
    io::Write::flush(&mut io::stdout()).unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let n: usize = input.trim().parse().unwrap();

    let mut matrix = vec![vec![0; n]; n];

    for i in 0..n {
        for j in 0..n {
            let mut elem = String::new();
            println!("Masukkan elemen baris {}, kolom {}:", i + 1, j + 1);
            io::stdin().read_line(&mut elem).unwrap();
            matrix[i][j] = elem.trim().parse().unwrap();
        }
    }

    println!("\n==============================================");
    println!("Representasi Graf dengan Matriks Ketetanggaan:\n");
    for i in 0..n {
        for j in 0..n {
            print!("{} ", matrix[i][j]);
        }
        println!();
    }
    println!("\n==============================================\n");

    // TSP dengan DP + bitmasking
    // dp[mask][pos] = biaya minimum untuk mengunjungi kota dengan bitmask mask, dan berada di kota pos sekarang

    let full_mask = (1 << n) - 1;
    let inf = 1_000_000_000;

    let mut dp = vec![vec![inf; n]; 1 << n];

    dp[1][0] = 0;

    for mask in 1..=full_mask {
        for u in 0..n {
            if dp[mask][u] == inf {
                continue;
            }
            // coba ke kota v yang belum dikunjungi
            for v in 0..n {
                // cek edge dari u ke v ada (bobot > 0) dan v belum dikunjungi
                if matrix[u][v] > 0 && (mask & (1 << v)) == 0 {
                    let next_mask = mask | (1 << v);
                    let cost = dp[mask][u] + matrix[u][v];
                    if cost < dp[next_mask][v] {
                        dp[next_mask][v] = cost;
                    }
                }
            }
        }
    }

    // cari hasil minimum kembali ke kota 0 setelah mengunjungi semua kota
    let mut ans = inf;
    for u in 1..n {
        if matrix[u][0] > 0 {
            let cost = dp[full_mask][u] + matrix[u][0];
            if cost < ans {
                ans = cost;
            }
        }
    }

    if ans == inf {
        println!("Tidak ada rute yang mengunjungi semua kota"); 
        println!("dan kembali ke kota asal.");
    } else {
        println!("Bobot minimum yang dibutuhkan dari kota 1"); 
        println!("hingga kembali ke kota 1 adalah {}.", ans);
    }

    println!("\n==============================================");
}


fn main() {
    println!("==============================================");
    println!("    SELAMAT DATANG DI TSP SOLVER SEDERHANA");
    println!("==============================================\n");

    loop {
        println!("1. Mencari solusi TSP");
        println!("2. Keluar");

        print!("Masukkan angka: ");

        io::Write::flush(&mut io::stdout()).unwrap(); 

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let choice: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Input tidak valid, silakan masukkan angka 1 atau 2.\n");
                continue;
            }
        };

        if choice == 1 {
            solver();
            println!();
        } else if choice == 2 {
            println!("\n==============================================");
            println!("                  TERIMA KASIH");
            println!("==============================================");
            break;
        } else {
            println!("Pilihan tidak tersedia, silakan coba lagi.\n");
        }
    }
}