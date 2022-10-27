
// stdlib imports
use std::fmt;
use std::{fmt::Write, time::SystemTime};
use nom::multi::many0;
use nom::sequence::terminated;
use nom::character::complete::u64;
// external library imports
use rand::{Rng, thread_rng};
use nom::error::{Error};
use nom::bytes::streaming::tag;

/// Benchmarks parsing numbers with [`&str::split`] and [`nom`]
/// 
/// We parse textual numbers to a [`Vec<u64>`] with:
/// * [`&str::split()`] and [`&str::parse()`]
/// * [`nom`] (with [`nom::character::complete::u64`])
pub fn main( ) {
  let n = 10_000_000;
  let input = gen_str( n ).unwrap( );
  println!( "Generated string of size: {}", input.len( ) );

  // Benchmark .split()
  let now = SystemTime::now( );
  for _i in 0..10 {
    let lines = input.split( "\n" );
    let parsed: Vec< u64 > = lines.into_iter( ).filter_map( |x| x.parse::< u64 >( ).ok( ) ).collect( );
    debug_assert!( parsed.len( ) == n );
  }
  println!( "Time for split: {}ms", now.elapsed().unwrap().as_millis() );

  // Benchmark nom
  let now = SystemTime::now( );
  for _i in 0..10 {
    // Without the type annotation (of `u64()`), the error infers to:
    // `Error< &String >`, which fails.
    let (rem_input, parsed) = many0( terminated( u64::< _, Error< &str > >, tag( "\n" ) ) )( &input ).unwrap( );
    debug_assert!( rem_input == "" );
    debug_assert!( parsed.len( ) == n );
  }
  println!( "Time for nom:   {}ms", now.elapsed().unwrap().as_millis() );
}

/// Generate a string with many (textual) numbers on each line. Every number
/// fits in a [`u64`].
fn gen_str( n: usize ) -> Result< String, fmt::Error > {
  let mut rng = thread_rng();
  let mut out = String::new( );
  for _i in 0..n {
    out.write_fmt( format_args!( "{}\n", rng.gen::< u64 >( ) ) )?;
  }
  Ok( out )
}
