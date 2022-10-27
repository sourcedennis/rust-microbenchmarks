
// stdlib imports
use std::{fmt::Write, time::SystemTime};
use std::fmt;
use nom::multi::many0;
use nom::sequence::{terminated};
// external library imports
use rand::{Rng, thread_rng};
use nom::{IResult, Parser};
use nom::error::{Error};
use nom::bytes::streaming::tag;

/// Benchmarks parsing numbers with [`&str::split`] and [`nom`]
/// 
/// We parse textual numbers to a [`Vec<u64>`] with:
/// * [`&str::split()`] and [`&str::parse()`]
/// * [`nom`] (with [`nom::character::complete::u64`])
pub fn main( ) {
  let n = 10_000_000;
  let str = gen_str( n ).unwrap( );
  println!( "Generated string of size: {}", str.len( ) );

  // Benchmark .split()
  let now = SystemTime::now( );
  for _i in 0..10 {
    let lines = str.split( "\n" );
    let parsed: Vec< u64 > = lines.into_iter( ).filter_map( |x| x.parse::< u64 >( ).ok( ) ).collect( );
    debug_assert!( parsed.len( ) == n );
  }
  println!( "Time for split: {}ms", now.elapsed().unwrap().as_millis() );

  // Benchmark nom
  let now = SystemTime::now( );
  for _i in 0..10 {
    let (rem_input, parsed) = many0( terminated( nom::character::complete::u64, p_discard( tag( "\n" ) ) ) )( &str ).unwrap( );
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

/// Discard the parser's output.
fn p_discard< 'a, A, F >(
  mut p: F
) -> impl FnMut( &'a str ) -> IResult< &'a str, () >
  where
    F : Parser< &'a str, A, Error< &'a str > >
{
  move |input| {
    let (input, _) = p.parse( input )?;
    Ok( (input, ()) )
  }
}
