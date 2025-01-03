//和を正しく計算できるか
#[cfg(test)]
mod test_add {
    use crate::Mod;
    #[test]
    fn test_mod_add() {
        let m: Mod<17> = 100.into();
        let n: Mod::<17> = 300.into();
        assert_eq!((m + n).value, 9); // 400 % 17 = 9
    }
}

//差を正しく計算できるか
#[cfg(test)]
mod test_sub {
    use crate::Mod;
    #[test]
    fn test_mod_sub() {
        let m: Mod<17> = 100.into();
        let n: Mod::<17> = 300.into();
        assert_eq!((m - n).value, 4); // -200 % 17 = 4
    }
}

//積を正しく計算できるか
#[cfg(test)]
mod test_mul {
    use crate::Mod;
    #[test]
    fn test_mod_mul() {
        let m: Mod<17> = 100.into();
        let n: Mod::<17> = 300.into();
        assert_eq!((m * n).value, 12); // 30000 % 17 = 12
    }
}

//累乗を正しく計算できるか
#[cfg(test)]
mod test_pow {
    use crate::Mod;
    #[test]
    fn test_mod_pow() {
        let m: Mod<17> = 100.into();
        assert_eq!(m.pow(2).value, 4); // 100^2 % 17 = 4
    }
}

//逆元を正しく計算できるか
#[cfg(test)]
mod test_inv {
    use crate::Mod;
    #[test]
    fn test_mod_inv() {
        let m: Mod<17> = 100.into();
        assert_eq!(m.inv().value, 8); // 100^-1 % 17 = 8
    }

    #[test]
    fn test_mod_inv2() {
        let m: Mod<18> = 5.into();
        let n = m.inv();
        assert_eq!(n.value, 11);
    }

    //基底と互いに素でない場合はpanic
    #[test]
    #[should_panic]
    fn test_mod_inv_not_coprime() {
        let m: Mod<18> = 100.into();
        m.inv();
    }
}

//除算を正しく計算できるか
#[cfg(test)]
mod test_div {
    use crate::Mod;
    #[test]
    fn test_mod_div() {
        let m: Mod<227> = 100.into();
        let n: Mod::<227> = 300.into();
        assert_eq!((m / n).value, 76); // (100 / 300) % 227 = 76
    }
}

//負の数の代入
#[cfg(test)]
mod test_neg {
    use crate::Mod;
    #[test]
    fn test_mod_neg() {
        const NEG: i64 = -100;
        let m: Mod<17> = NEG.into();
        assert_eq!(m.value, 2);
    }
}

//簡略表記
#[cfg(test)]
mod test_simplified_notation {
    use crate::Mod;
    #[test]
    fn test_add_simplified_notation() {
        let mut m: Mod<17> = 100.into();
        let n: Mod::<17> = 300.into();
        m += n;
        assert_eq!(m.value, 9);
    }

    #[test]
    fn test_sub_simplified_notation() {
        let mut m: Mod<17> = 100.into();
        let n: Mod::<17> = 300.into();
        m -= n;
        assert_eq!(m.value, 4);
    }

    #[test]
    fn test_mul_simplified_notation() {
        let mut m: Mod<17> = 100.into();
        let n: Mod::<17> = 300.into();
        m *= n;
        assert_eq!(m.value, 12);
    }

    #[test]
    fn test_div_simplified_notation() {
        let mut m: Mod<17> = 100.into();
        let n: Mod::<17> = 300.into();
        m /= n;
        assert_eq!(m.value, 6);
    }
}

#[cfg(test)]
mod test_u32 {
    use crate::Mod;
    #[test]
    fn test_add_u32() {
        let m: Mod<17> = 100.into();
        assert_eq!((m + 3).value, 103 % 17);
    }

    #[test]
    fn test_sub_u32() {
        let m: Mod<17> = 100.into();
        assert_eq!((m - 3).value, 97 % 17);
    }

    #[test]
    fn test_mul_u32() {
        let m: Mod<17> = 100.into();
        assert_eq!((m * 3).value, 300 % 17);
    }

    #[test]
    fn test_big_number() {
        let m: Mod<1000000007> = 1000000000.into();
        let n: i64 = 1000000000000000000;
        assert_eq!((m * n).value, 999999664);
    }
}
