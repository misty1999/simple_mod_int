//! # simple-mod-int
//! 
//! 競技プログラミングのための基本的な剰余演算ライブラリです。
//! 
//! ## 特徴
//! 
//! * 四則演算、逆元、累乗をサポート
//! * i64に変換可能な型との四則演算をサポート
//! * 代入時や指数指定時の負の数をサポート
//! 
//! A basic modular arithmetic library for competitive programming.
//! 
//! ## Features
//! 
//! * Basic arithmetic operations, multiplicative inverse, and exponentiation support
//! * Supports arithmetic operations with any type that can be converted to i64
//! * Handles negative numbers in assignments and exponents
//! 

pub mod test;

use std::ops::Deref;
use num_traits::AsPrimitive;

#[derive(Clone)]
pub struct Mod<const M: u32> {
    pub value: u32,
}

impl<const M: u32> Deref for Mod<M> {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

///数値型からの変換
/// #example
/// ```
/// use simple_mod_int::Mod;
/// let a: Mod<17> = 10.into();
/// ```
impl<const M: u32, T> From<T> for Mod<M> 
where 
    T: AsPrimitive<i64> + Copy
{
    fn from(value: T) -> Self {
        let value_i64 = value.as_();
        let m_i64 = M as i64;
        Mod { 
            value: ((value_i64 % m_i64 + m_i64) % m_i64) as u32,
        }
    }
}

///加算
/// #example
/// ```
/// use simple_mod_int::Mod;
/// let a: Mod<17> = 10.into();
/// let b: Mod<17> = 20.into();
/// let c = a + b;
/// ```
impl<const M: u32> std::ops::Add for Mod<M> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let sum = (self.value as u64 + other.value as u64) % M as u64;
        Mod::from(sum as u32)
    }
}

///加算代入
/// #example
/// ```
/// use simple_mod_int::Mod;
/// let mut a: Mod<17> = 10.into();
/// let b: Mod<17> = 20.into();
/// a += b;
/// ```
impl<const M: u32> std::ops::AddAssign for Mod<M> {
    fn add_assign(&mut self, other: Self) {
        self.value = ((self.value as u64 + other.value as u64) % M as u64) as u32;
    }
}

///減算
/// #example
/// ```
/// use simple_mod_int::Mod;
/// let a: Mod<17> = 10.into();
/// let b: Mod<17> = 20.into();
/// let c = a - b;
/// ```
impl<const M: u32> std::ops::Sub for Mod<M> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Mod::from(self.value - other.value + M)
    }
}

///減算代入
/// #example
/// ```
/// use simple_mod_int::Mod;
/// let mut a: Mod<17> = 10.into();
/// let b: Mod<17> = 20.into();
/// a -= b;
/// ```
impl<const M: u32> std::ops::SubAssign for Mod<M> {
    fn sub_assign(&mut self, other: Self) {
        self.value = (self.value - other.value + M) % M;
    }
}

///乗算
/// #example
/// ```
/// use simple_mod_int::Mod;
/// let a: Mod<17> = 10.into();
/// let b: Mod<17> = 20.into();
/// let c = a * b;
/// ```
impl<const M: u32> std::ops::Mul for Mod<M> {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        let product = (self.value as u64 * other.value as u64) % M as u64;
        Mod::from(product as u32)
    }
}

///乗算代入
/// #example
/// ```
/// use simple_mod_int::Mod;
/// let mut a: Mod<17> = 10.into();
/// let b: Mod<17> = 20.into();
/// a *= b;
/// ```
impl<const M: u32> std::ops::MulAssign for Mod<M> {
    fn mul_assign(&mut self, other: Self) {
        self.value = ((self.value as u64 * other.value as u64) % M as u64) as u32;
    }
}

///除算
/// #example
/// ```
/// use simple_mod_int::Mod;
/// let a: Mod<17> = 10.into();
/// let b: Mod<17> = 20.into();
/// let c = a / b;
/// ```
impl<const M: u32> std::ops::Div for Mod<M> {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        self * other.inv()
    }
}

///除算代入
/// #example    
/// ```
/// use simple_mod_int::Mod;
/// let mut a: Mod<17> = 10.into();
/// let b: Mod<17> = 20.into();
/// a /= b;
/// ```
impl<const M: u32> std::ops::DivAssign for Mod<M> {
    fn div_assign(&mut self, other: Self) {
        self.value = (self.value * other.inv().value) % M;
    }
}

///数値型との加算
/// #example
/// ```
/// use simple_mod_int::Mod;
/// let a: Mod<17> = 10.into();
/// let b: Mod<17> = 20;
/// let c = a + b;
/// ```
impl<const M: u32, T> std::ops::Add<T> for Mod<M>
where
    T: AsPrimitive<i64> + Copy
{
    type Output = Self;

    fn add(self, other: T) -> Self::Output {
        let other_i64 = other.as_();
        self + Mod::from(other_i64)
    }
}

///数値型との減算
/// #example
/// ```
/// use simple_mod_int::Mod;
/// let a: Mod<17> = 10.into();
/// let b: Mod<17> = 20;
/// let c = a - b;
/// ```
impl<const M: u32, T> std::ops::Sub<T> for Mod<M>
where
    T: AsPrimitive<i64> + Copy
{
    type Output = Self;

    fn sub(self, other: T) -> Self::Output {
        let other_i64 = other.as_();
        self - Mod::from(other_i64)
    }
}

///数値型との乗算
/// #example
/// ```
/// use simple_mod_int::Mod;
/// let a: Mod<17> = 10.into();
/// let b: Mod<17> = 20;
/// let c = a * b;
/// ```
impl<const M: u32, T> std::ops::Mul<T> for Mod<M>
where
    T: AsPrimitive<i64> + Copy
{
    type Output = Self;

    fn mul(self, other: T) -> Self::Output {
        let other_i64 = other.as_();
        self * Mod::from(other_i64)
    }
}

///数値型との除算
/// #example
/// ```
/// use simple_mod_int::Mod;
/// let a: Mod<17> = 10.into();
/// let b: Mod<17> = 20;
/// let c = a / b;
/// ```
/// 
/// #panic
/// baseと互いに素な数で除算しようとした場合、panicが発生します。
/// when you try to divide by a number that is not coprime with the base, panic occurs.
impl<const M: u32, T> std::ops::Div<T> for Mod<M>
where
    T: AsPrimitive<i64> + Copy
{
    type Output = Self;

    fn div(self, other: T) -> Self::Output {
        let other_i64 = other.as_();
        self * Mod::from(other_i64).inv()
    }
}

impl<const M: u32> Mod<M> {
    ///累乗
    /// #example
    /// ```
    /// use simple_mod_int::Mod;
    /// let a: Mod<17> = 10.into();
    /// let b: Mod<17> = 20.into();
    /// let c = a.pow(b);
    /// ```
    /// 
    /// 負の指数も指定可能
    /// 負の指数の場合、逆元を計算してから累乗します
    /// can specify negative exponent
    /// when you specify a negative exponent, it calculates the inverse and then raises it to the power
    fn pow<T>(self, other: T) -> Self 
    where
        T: AsPrimitive<i64> + Copy
    {
        let exp = other.as_();
        
        if exp >= 0 {
            // 正の指数の場合は通常の累乗計算
            let mut result = Mod::from(1);
            let mut base = self;
            let mut exp = exp as u64;

            while exp > 0 {
                if exp % 2 == 1 {
                    result = result * base.clone();
                }
                base = Mod::from(base.value * base.value);
                exp /= 2;
            }
            result
        } else {
            // 負の指数の場合は逆元を計算してから累乗
            let inv = self.inv();  // 逆元を計算
            inv.pow(-exp)  // 正の指数として計算
        }
    }

    ///逆元
    /// #example
    /// ```
    /// use simple_mod_int::Mod;
    /// let a: Mod<17> = 10.into();
    /// let b = a.inv(); //12
    /// ```
    /// 
    /// #panic
    /// 基底と互いに素でない場合、panicが発生します。
    /// when you try to calculate the inverse of a number that is not coprime with the base, panic occurs.
    fn inv(self) -> Self {

        if self.value == 0 {
            panic!("0 has no inverse");
        }
        // 拡張ユークリッドの互除法で逆元を計算
        let (g, x, _) = Self::ext_gcd(self.value as i64, M as i64);
        if g != 1 {
            panic!("multiplicative inverse does not exist");
        }
        Mod::from(x)
    }

    // 拡張ユークリッドの互除法
    fn ext_gcd(a: i64, b: i64) -> (i64, i64, i64) {
        if a == 0 {
            return (b, 0, 1);
        }
        let (g, x, y) = Self::ext_gcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

///等価
/// #example
/// ```
/// use simple_mod_int::Mod;
/// let a: Mod<17> = 10.into();
/// let b: Mod<17> = 61.into();
/// a==b //true
/// ```
impl<const M: u32> PartialEq for Mod<M> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

///デバッグ
/// #example
/// ```
/// use simple_mod_int::Mod;
/// let a: Mod<17> = 10.into();
/// println!("{:?}", a); //Mod { value: 10 , mod: 17 }
/// ```
impl<const M: u32> std::fmt::Debug for Mod<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Mod")
            .field("value", &self.value)
            .field("mod", &M)
            .finish()
    }
}