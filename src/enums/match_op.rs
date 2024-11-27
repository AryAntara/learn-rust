enum Suku {
    Jawa,
    Bali,
}

impl Suku {
    fn power(&self) -> i32 {
        match self {
            Suku::Jawa => 2,
            Suku::Bali => 3
        }
    }
}

pub fn main() {
    let orang_bali = Suku::Bali; 
    let orang_jawa = Suku::Jawa; 


    adu_suku(orang_bali, orang_jawa);
}

fn adu_suku(s: Suku, other: Suku) -> Suku {
    if s.power() > other.power() {
        return s 
    }

    return other
}

