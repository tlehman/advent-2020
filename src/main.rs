mod day01;
mod day02;
use std::fs::File;
use std::path::Path;
use std::io;
use std::io::BufRead;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let day01_input: &[i32] = &[1583, 1295, 1747, 1628, 1756, 1992, 1984, 1990, 2006, 1626, 1292, 1561, 1697, 1249, 2001, 1822, 1715, 1951, 1600, 1615, 1769, 1825, 1335, 1987, 1745, 1660, 1952, 1437, 1348, 1968, 615 , 1847, 476 , 1346, 1357, 1838, 1955, 1750, 1831, 2003, 1730, 1696, 1257, 1581, 866 , 1765, 1691, 1995, 1977, 1988, 1713, 1599, 1300, 1892, 1550, 2002, 1694, 1930, 1998, 1564, 1704, 1398, 864 , 1480, 1578, 1946, 1850, 1964, 1914, 1860, 1979, 1857, 1969, 1675, 1967, 2009, 1950, 1834, 783 , 1935, 1963, 1659, 1314, 1647, 1671, 1706, 1734, 1965, 1666, 316 , 1657, 1663, 1373, 1719, 1778, 1559, 1869, 1958, 1986, 1845, 1643, 1783, 1670, 1445, 1758, 2008, 1680, 1251, 1982, 1420, 1621, 1997, 1785, 1994, 1376, 1944, 1771, 1844, 96  , 467 , 1954, 903 , 1368, 1305, 1589, 1970, 1980, 1521, 1775, 1629, 1796, 1985, 1957, 1669, 1637, 1606, 1766, 1972, 1956, 1685, 1235, 58  , 1996, 1959, 1788, 1273, 1378, 1233, 1470, 1584, 1741, 1327, 1763, 1989, 1665, 1667, 1975, 1862, 1791, 1229, 1873, 1761, 1754, 1882, 1642, 1971, 1777, 1580, 1648, 1678, 1266, 1645, 502 , 1717, 1723, 1244, 1370, 1898, 1755, 1708, 1983, 1901, 844 , 1239, 1290, 1879, 1656, 1966, 1929, 1993, 1743, 1909, 1451, 2000, 1978, 1938, 1707, 1337, 1362, 1263];
    println!(
        "day 1 sum 2 to 2020 =  {}",
        day01::sum2_to_2020_mult(day01_input)
    );
    println!(
        "day 1 sum 3 to 2020 =  {}",
        day01::sum3_to_2020_mult(day01_input)
    );
    // Day 02, part 1: read file input
    if let Ok(lines) = read_lines("./data/day02.txt") {
        // each line is divided into two parts: rule and password
        // we maintain an integer and use the parity (even vs odd)
        // to decide if it's a rule or password

        let mut valid_passwords = 0;
        let mut valid_passwords_part2 = 0;
        for line in lines {
            if let Ok(raw_line) = line {
                let mut iter = raw_line.split(": ");
                if let Some(rule) = iter.next() {
                    if let Some(password) = iter.next() {
                        let rule = day02::Rule::parse(rule);
                        if rule.valid(password) {
                            valid_passwords += 1;
                        }
                        if rule.valid_part2(password) {
                            valid_passwords_part2 += 1;
                        }
                    }
                }
            }
        }
        println!("day02.txt has {} valid passwords", valid_passwords);
        println!("day02.txt has {} valid passwords (part2)", valid_passwords_part2);
    }
}

