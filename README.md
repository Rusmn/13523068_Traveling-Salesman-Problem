# Traveling Salesman Problem Solver

<p align="center">
  <img src="img/contoh.gif" alt="Contoh" />
  <br>
  <br>
  Hasil
  <br>
  <img src = "img/hasil.png" alt = "hasil"/>
</p>

Mencari harga (bobot) minimum untuk mengunjungi semua kota (node) dari kota asal lalu kembali lagi ke kota asal.

---

## Requirement Program

Pastikan rust terinstall, contoh berikut.

```bash
rustc --version
rustc 1.87.0
```

```bash
cargo --version
cargo 1.87.0
```

## How To Run

1. Clone repository ini.

```bash
git clone https://github.com/Rusmn/13523068_Traveling-Salesman-Problem.
```

2. dari root ke directory `tsp_solution`.

```bash
cd tsp_solution
```

3. run program.

```bash
cargo run
```

---

## Petunjuk Penggunaan

1. `n` sebagai jumlah kota.
2. baris merepresentasikan kota asal.
3. kolom merepresentasikan kota tujuan.
4. input angka merepresentasikan harga.
5. input `0` merepresentasikan kota tidak terhubung serta jika baris dan kolom sama.

## Contoh Input Output

Input

```
Masukkan jumlah kota (n): 2
Masukkan elemen baris 1, kolom 1:
0
Masukkan elemen baris 1, kolom 2:
2
Masukkan elemen baris 2, kolom 1:
1
Masukkan elemen baris 2, kolom 2:
0
```

Output

```
==============================================
Representasi Graf dengan Matriks Ketetanggaan:

0 2
1 0

==============================================

Bobot minimum yang dibutuhkan dari kota 1
hingga kembali ke kota 1 adalah 3.

==============================================
```

## Author

<p>
Muh. Rusmin Nurwadin
<br>
13523068
</p>
