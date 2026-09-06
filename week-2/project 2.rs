fn main () {
println! ("Sales Record for P.M Okeke and Sons Ltd");
Let t_amt: 132 = 450_000; // t_amt is amount of toshiba sold
Let m_amt: 132 = 1_500_000;// m_amt is amount of mac
Let hp_amt: 132 = 750_000; // amount of hp sold
Let d_amt: 132 = 2_850_000; // amount of dell sold
Let a_amt: 132 = 250_000; //amount of acer sold
Let t_qty:132 = 2; // quantity of toshiba
Let m_qty: 132 = 1; // " mac
Let hp_qty: 132 = 3; // " hp
Let d_qty:132 = 3; // " dell
Let a_qty:132 = 1; // " acer
Let s:i32 = t_amt + m_amt + hp_amt + d_amt + a_amt; // sum of sales
Let a:132 = s / (t_qty + m_qty + hp_qty + d_qty + a_qty); // average of sales
println! ("The sum of the sales record is {}",s);
println! ("The average of the sales record is {}",a);
}