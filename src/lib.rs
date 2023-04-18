#![allow(dead_code)]
mod bulb;
mod bulblibrary;
mod cli;
pub mod discovery;
mod errors;
mod models;
mod protocol;
mod push_manager;
mod rgbcw;
mod scenes;
mod utils;

pub use errors::{Result, WizError};
/// Macro for creating a [map](hashbrown::HashMap).
///
/// Equivalent to the [vec!] macro for [vectors](Vec).
///
/// **Example:**
///
/// ```rust
/// use wizlight_rs::map;
///
/// let goodbye = map! {
///     "en" => "Goodbye",
///     "de" => "Auf Wiedersehen",
///     "fr" => "Au revoir",
///     "es" => "Adios",
/// };
/// ```
///
#[macro_export]
macro_rules! map {
    {$($k: expr => $v: expr),* $(,)?} => {
        hashbrown::HashMap::from([$(($k, $v),)*])
    };
}

/// Macro for creating a [map](hashbrown::HashMap) inside [Lazy](once_cell::unsync::Lazy).
///
/// Equivalent to the [vec!] macro for [vectors](Vec).
///
/// **Example:**
///
/// ```rust
/// use wizlight_rs::lazy_map;
///
/// let goodbye = lazy_map! {
///     "en" => "Goodbye",
///     "de" => "Auf Wiedersehen",
///     "fr" => "Au revoir",
///     "es" => "Adios",
/// };
/// ```
///
#[macro_export]
macro_rules! lazy_map {
    {$($k: expr => $v: expr),* $(,)?} => {
        once_cell::unsync::Lazy::new(|| hashbrown::HashMap::from([$(($k, $v),)*]))
    };
}

/// Macro for creating a [map](hashbrown::HashMap) inside [Lazy](once_cell::sync::Lazy).
///
/// Equivalent to the [vec!] macro for [vectors](Vec).
///
/// **Example:**
///
/// ```rust
/// use wizlight_rs::lazy_sync_map;
///
/// let goodbye = lazy_sync_map! {
///     "en" => "Goodbye",
///     "de" => "Auf Wiedersehen",
///     "fr" => "Au revoir",
///     "es" => "Adios",
/// };
/// ```
///
#[macro_export]
macro_rules! lazy_sync_map {
    {$($k: expr => $v: expr),* $(,)?} => {
        once_cell::sync::Lazy::new(|| hashbrown::HashMap::from([$(($k, $v),)*]))
    };
}
