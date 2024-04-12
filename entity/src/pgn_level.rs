use std::{
    fmt::Display,
    ops::{Add, AddAssign, Sub, SubAssign***REMOVED***,
    str::FromStr,
***REMOVED***

use num_derive::FromPrimitive;
use saturating_cast::SaturatingCast;

/// Pgn上でのレベルを表す
#[derive(FromPrimitive, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i8)]
pub enum PgnLevel {
    Iron = 0,
    Bronze = 1,
    Silver = 2,
    Gold = 3,
    Platinum = 4,
    Diamond = 5,
    Master = 6,
    GrandMaster = 7,
***REMOVED***

impl AsRef<str> for PgnLevel {
    fn as_ref(&self) -> &str {
        match self {
            PgnLevel::Iron => "Iron",
            PgnLevel::Bronze => "Bronze",
            PgnLevel::Silver => "Silver",
            PgnLevel::Gold => "Gold",
            PgnLevel::Platinum => "Platinum",
            PgnLevel::Diamond => "Diamond",
            PgnLevel::Master => "Master",
            PgnLevel::GrandMaster => "GrandMaster",
    ***REMOVED***
***REMOVED***
***REMOVED***

impl Display for PgnLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{***REMOVED***", self.as_ref())
***REMOVED***
***REMOVED***

impl FromStr for PgnLevel {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Iron" => Ok(PgnLevel::Iron),
            "Bronze" => Ok(PgnLevel::Bronze),
            "Silver" => Ok(PgnLevel::Silver),
            "Gold" => Ok(PgnLevel::Gold),
            "Platinum" => Ok(PgnLevel::Platinum),
            "Diamond" => Ok(PgnLevel::Diamond),
            "Master" => Ok(PgnLevel::Master),
            "GrandMaster" => Ok(PgnLevel::GrandMaster),
            _ => Err("invalid value"),
    ***REMOVED***
***REMOVED***
***REMOVED***

fn cast(step: i8) -> PgnLevel {
    match step {
        step if step < PgnLevel::Iron as i8 => PgnLevel::Iron,
        step if step > PgnLevel::GrandMaster as i8 => PgnLevel::GrandMaster,
        step => unsafe { std::mem::transmute(step) ***REMOVED***,
***REMOVED***
***REMOVED***

impl<T: SaturatingCast + saturating_cast::SaturatingElement<i8>> Add<T> for PgnLevel {
    type Output = Self;
    fn add(self, step: T) -> Self::Output {
        cast((self as i8).saturating_add(step.saturating_cast()))
***REMOVED***
***REMOVED***

impl<T: SaturatingCast + saturating_cast::SaturatingElement<i8>> AddAssign<T> for PgnLevel {
    fn add_assign(&mut self, step: T) {
        *self = *self + step;
***REMOVED***
***REMOVED***

impl<T: SaturatingCast + saturating_cast::SaturatingElement<i8>> Sub<T> for PgnLevel {
    type Output = Self;
    fn sub(self, step: T) -> Self::Output {
        cast((self as i8).saturating_sub(step.saturating_cast()))
***REMOVED***
***REMOVED***

impl<T: SaturatingCast + saturating_cast::SaturatingElement<i8>> SubAssign<T> for PgnLevel {
    fn sub_assign(&mut self, step: T) {
        *self = *self - step;
***REMOVED***
***REMOVED***

impl PgnLevel {
    pub fn max_pix(self) -> u32 {
        use max_values::*;
        match self {
            PgnLevel::Iron => IRON_MAX,
            PgnLevel::Bronze => BRONZE_MAX,
            PgnLevel::Silver => SILVER_MAX,
            PgnLevel::Gold => GOLD_MAX,
            PgnLevel::Platinum => PLATINUM_MAX,
            PgnLevel::Diamond => DIAMOND_MAX,
            PgnLevel::Master => MASTER_MAX,
            PgnLevel::GrandMaster => u32::MAX,
    ***REMOVED***
***REMOVED***
***REMOVED***

pub mod max_values {
    pub const IRON_MAX: u32 = 500;
    pub const BRONZE_MAX: u32 = 1000;
    pub const SILVER_MAX: u32 = 2500;
    pub const GOLD_MAX: u32 = 5000;
    pub const PLATINUM_MAX: u32 = 10000;
    pub const DIAMOND_MAX: u32 = 20000;
    pub const MASTER_MAX: u32 = 35000;
***REMOVED***

impl From<u32> for PgnLevel {
    fn from(pix_monthly: u32) -> Self {
        use max_values::*;
        use PgnLevel::*;
        match pix_monthly {
            pix if pix < IRON_MAX => Iron,
            pix if pix < BRONZE_MAX => Bronze,
            pix if pix < SILVER_MAX => Silver,
            pix if pix < GOLD_MAX => Gold,
            pix if pix < PLATINUM_MAX => Platinum,
            pix if pix < DIAMOND_MAX => Diamond,
            pix if pix < MASTER_MAX => Master,
            _ => GrandMaster,
    ***REMOVED***
***REMOVED***
***REMOVED***
